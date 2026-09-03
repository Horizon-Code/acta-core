#!/usr/bin/env python3
"""Mechanical weekly source surveillance for ACTA.

This program deliberately collects changes without interpreting their relevance.  It stores
only source cursors/digests and writes a neutral weekly inbox.  No LLM is involved.
"""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import hashlib
import html
import json
import os
import re
import sys
import time
import tomllib
import urllib.error
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET
from concurrent.futures import ThreadPoolExecutor, as_completed
from html.parser import HTMLParser
from pathlib import Path
from typing import Any, Callable


class CollectionError(RuntimeError):
    """A visible, source-local collection failure."""


@dataclasses.dataclass(frozen=True)
class Observation:
    source: str
    label: str
    change: str
    link: str
    date: str
    attention: bool = False
    highlighted: bool = False


@dataclasses.dataclass
class Collection:
    source: str
    snapshot: dict[str, Any]
    observations: list[Observation]
    changed: bool


class _TextExtractor(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.parts: list[str] = []
        self._ignored = 0

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag in {"script", "style", "noscript", "svg"}:
            self._ignored += 1

    def handle_endtag(self, tag: str) -> None:
        if tag in {"script", "style", "noscript", "svg"} and self._ignored:
            self._ignored -= 1

    def handle_data(self, data: str) -> None:
        if not self._ignored:
            self.parts.append(data)

    def text(self) -> str:
        return " ".join(" ".join(self.parts).split())


def utc_today() -> dt.date:
    return dt.datetime.now(dt.timezone.utc).date()


def fetch_bytes(
    url: str,
    *,
    headers: dict[str, str] | None = None,
    data: bytes | None = None,
    timeout: int = 15,
    attempts: int = 2,
) -> bytes:
    request_headers = {
        "Accept": "application/atom+xml, application/rss+xml, application/xml, text/html, application/json;q=0.9, */*;q=0.5",
        "User-Agent": "ACTA-vigilancia/1.0 (+https://github.com/Horizon-Code/acta-core)",
    }
    request_headers.update(headers or {})
    last_error: Exception | None = None
    for attempt in range(attempts):
        try:
            request = urllib.request.Request(url, data=data, headers=request_headers)
            with urllib.request.urlopen(request, timeout=timeout) as response:
                return response.read()
        except (urllib.error.URLError, TimeoutError, OSError) as error:
            last_error = error
            if attempt + 1 < attempts:
                time.sleep(2**attempt)
    raise CollectionError(f"no se pudo descargar {url}: {last_error}")


def _local_name(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def _element_text(element: ET.Element, child_name: str) -> str:
    for child in element.iter():
        if _local_name(child.tag) == child_name and child.text:
            return child.text.strip()
    return ""


def parse_feed(payload: bytes) -> list[dict[str, str]]:
    try:
        root = ET.fromstring(payload)
    except ET.ParseError as error:
        raise CollectionError(f"feed XML inválido: {error}") from error

    entries = [node for node in root.iter() if _local_name(node.tag) in {"entry", "item"}]
    parsed: list[dict[str, str]] = []
    for entry in entries:
        title = html.unescape(_element_text(entry, "title"))
        identifier = _element_text(entry, "id") or _element_text(entry, "guid")
        date = (
            _element_text(entry, "updated")
            or _element_text(entry, "published")
            or _element_text(entry, "pubDate")
        )
        link = ""
        for child in entry.iter():
            if _local_name(child.tag) == "link":
                link = child.attrib.get("href", "") or (child.text or "").strip()
                if link:
                    break
        if not identifier:
            identifier = link or hashlib.sha256(f"{title}\0{date}".encode()).hexdigest()
        if title and link:
            parsed.append({"id": identifier, "title": title, "link": link, "date": date})
    if not parsed and _local_name(root.tag).casefold() not in {"feed", "rss", "rdf"}:
        raise CollectionError("la respuesta no es un feed Atom/RSS reconocible")
    return parsed


def _contains_any(value: str, terms: list[str]) -> bool:
    lowered = value.casefold()
    return any(term.casefold() in lowered for term in terms)


def collect_atom(config: dict[str, Any], previous: dict[str, Any]) -> Collection:
    entries = parse_feed(fetch_bytes(config["url"]))
    include_terms = list(config.get("include_terms", []))
    if include_terms:
        entries = [entry for entry in entries if _contains_any(entry["title"], include_terms)]

    current_ids = [entry["id"] for entry in entries]
    previous_ids = set(previous.get("ids", []))
    first_run = "ids" not in previous
    observations: list[Observation] = []
    new_entries = [] if first_run else [entry for entry in entries if entry["id"] not in previous_ids]
    limit = int(config.get("max_observations", len(new_entries)))
    for entry in new_entries[:limit]:
        highlighted = _contains_any(entry["title"], list(config.get("highlight_terms", [])))
        attention = bool(config.get("always_attention", False) or highlighted)
        if config.get("archive_only", False):
            attention = False
        observations.append(
            Observation(
                source=config["display_id"],
                label=config["label"],
                change=f"entrada nueva: {entry['title']}",
                link=entry["link"],
                date=entry["date"][:10] or str(utc_today()),
                attention=attention,
                highlighted=highlighted,
            )
        )
    snapshot = {"ids": current_ids[: int(config.get("max_seen", 250))]}
    return Collection(
        source=config["display_id"],
        snapshot=snapshot,
        observations=observations,
        changed=not first_run and snapshot != previous,
    )


def collect_env_atom_list(config: dict[str, Any], previous: dict[str, Any]) -> Collection:
    raw = os.environ.get(config["environment_variable"], "").strip()
    if not raw:
        raise CollectionError(f"no configurada: falta {config['environment_variable']}")
    try:
        urls = json.loads(raw)
    except json.JSONDecodeError:
        urls = [line.strip() for line in raw.splitlines() if line.strip()]
    if not isinstance(urls, list) or not urls or not all(isinstance(url, str) for url in urls):
        raise CollectionError(f"{config['environment_variable']} debe ser un array JSON de URLs")

    combined: list[dict[str, str]] = []
    for url in urls:
        combined.extend(parse_feed(fetch_bytes(url)))
    synthetic = dict(config)
    synthetic["url"] = urls[0]
    current_ids = [entry["id"] for entry in combined]
    previous_ids = set(previous.get("ids", []))
    first_run = "ids" not in previous
    observations = []
    for entry in reversed(combined):
        if first_run or entry["id"] in previous_ids:
            continue
        observations.append(
            Observation(
                source=config["display_id"],
                label=config["label"],
                change=f"resultado nuevo: {entry['title']}",
                link=entry["link"],
                date=entry["date"][:10] or str(utc_today()),
                attention=True,
            )
        )
    snapshot = {"ids": current_ids[: int(config.get("max_seen", 500))]}
    return Collection(config["display_id"], snapshot, observations, not first_run and snapshot != previous)


def collect_html_block(config: dict[str, Any], previous: dict[str, Any]) -> Collection:
    payload = fetch_bytes(config["url"])
    parser = _TextExtractor()
    parser.feed(payload.decode("utf-8", errors="replace"))
    text = parser.text()
    match = re.search(config["block_pattern"], text, flags=re.IGNORECASE)
    if not match:
        raise CollectionError(f"no se encontró el bloque {config['block_name']}")
    normalized = " ".join(match.group(0).split())
    digest = hashlib.sha256(normalized.encode("utf-8")).hexdigest()
    snapshot = {"sha256": digest}
    first_run = "sha256" not in previous
    observations = []
    if not first_run and digest != previous.get("sha256"):
        observations.append(
            Observation(
                source=config["display_id"],
                label=config["label"],
                change=f"{config['block_name']} modificado",
                link=config["url"],
                date=str(utc_today()),
                attention=True,
                highlighted=True,
            )
        )
    return Collection(config["display_id"], snapshot, observations, not first_run and snapshot != previous)


def _find_lotl_sequence(root: ET.Element) -> str:
    for element in root.iter():
        if _local_name(element.tag) == "TSLSequenceNumber" and element.text:
            return element.text.strip()
    return "unknown"


def lotl_counts(payload: bytes, patterns: list[str]) -> tuple[str, dict[str, int]]:
    try:
        root = ET.fromstring(payload)
    except ET.ParseError as error:
        raise CollectionError(f"LOTL XML inválido: {error}") from error
    counts = {pattern: 0 for pattern in patterns}
    for element in root.iter():
        if _local_name(element.tag) != "ServiceTypeIdentifier" or not element.text:
            continue
        value = element.text.casefold()
        for pattern in patterns:
            if pattern.casefold() in value:
                counts[pattern] += 1
    return _find_lotl_sequence(root), counts


def lotl_locations(payload: bytes) -> list[str]:
    try:
        root = ET.fromstring(payload)
    except ET.ParseError as error:
        raise CollectionError(f"LOTL XML inválido: {error}") from error
    locations = []
    for element in root.iter():
        if _local_name(element.tag) == "TSLLocation" and element.text:
            location = element.text.strip()
            if location.startswith("https://"):
                locations.append(location)
    return sorted(set(locations))


def collect_lotl(config: dict[str, Any], previous: dict[str, Any]) -> Collection:
    lotl_payload = fetch_bytes(config["url"])
    sequence = _find_lotl_sequence(ET.fromstring(lotl_payload))
    locations = lotl_locations(lotl_payload)
    if not locations:
        raise CollectionError("la LOTL no contiene enlaces HTTPS a listas nacionales")
    patterns = list(config["service_patterns"])
    counts = {pattern: 0 for pattern in patterns}
    failures: list[str] = []
    with ThreadPoolExecutor(max_workers=int(config.get("parallel_downloads", 8))) as executor:
        futures = {executor.submit(fetch_bytes, url): url for url in locations}
        for future in as_completed(futures):
            url = futures[future]
            try:
                _, national_counts = lotl_counts(future.result(), patterns)
            except Exception as error:
                failures.append(f"{url}: {error}")
                continue
            for pattern, count in national_counts.items():
                counts[pattern] += count
    if failures:
        sample = "; ".join(failures[:3])
        suffix = f"; y {len(failures) - 3} más" if len(failures) > 3 else ""
        raise CollectionError(f"fallaron {len(failures)}/{len(locations)} listas nacionales: {sample}{suffix}")
    snapshot: dict[str, Any] = {"sequence": sequence, "lists": len(locations), "counts": counts}
    first_run = "sequence" not in previous
    observations: list[Observation] = []
    if not first_run and snapshot != previous:
        previous_counts = previous.get("counts", {})
        increases = [
            pattern for pattern, count in counts.items() if count > int(previous_counts.get(pattern, 0))
        ]
        if increases:
            observations.append(
                Observation(
                    source=config["display_id"],
                    label=config["label"],
                    change="alta detectada en categoría vigilada: " + ", ".join(increases),
                    link=config["url"],
                    date=str(utc_today()),
                    attention=True,
                    highlighted=True,
                )
            )
        else:
            observations.append(
                Observation(
                    source=config["display_id"],
                    label=config["label"],
                    change=f"secuencia LOTL {previous.get('sequence')} → {sequence}; sin altas en categorías vigiladas",
                    link=config["url"],
                    date=str(utc_today()),
                )
            )
    return Collection(config["display_id"], snapshot, observations, not first_run and snapshot != previous)


def _github_headers() -> dict[str, str]:
    headers = {"Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}
    token = os.environ.get("GITHUB_TOKEN", "").strip()
    if token:
        headers["Authorization"] = f"Bearer {token}"
    return headers


def _fetch_json(url: str) -> Any:
    payload = fetch_bytes(url, headers=_github_headers())
    try:
        return json.loads(payload)
    except json.JSONDecodeError as error:
        raise CollectionError(f"JSON inválido en {url}: {error}") from error


def collect_github_issue(config: dict[str, Any], previous: dict[str, Any]) -> Collection:
    issue = _fetch_json(config["url"])
    snapshot = {
        "updated_at": issue.get("updated_at"),
        "state": issue.get("state"),
        "state_reason": issue.get("state_reason"),
        "title": issue.get("title"),
    }
    first_run = "updated_at" not in previous
    observations = []
    if not first_run and snapshot != previous:
        decided = snapshot["state"] != previous.get("state") or snapshot["state_reason"] is not None
        change = f"actividad nueva en issue #{issue.get('number')}"
        if decided:
            change += f"; estado {snapshot['state']}"
        observations.append(
            Observation(
                source=config["display_id"],
                label=config["label"],
                change=change,
                link=issue.get("html_url", config["url"]),
                date=(issue.get("updated_at") or str(utc_today()))[:10],
                attention=decided,
                highlighted=decided,
            )
        )
    return Collection(config["display_id"], snapshot, observations, not first_run and snapshot != previous)


def collect_github_issues(config: dict[str, Any], previous: dict[str, Any]) -> Collection:
    issues = _fetch_json(config["url"])
    if not isinstance(issues, list):
        raise CollectionError("la API de GitHub no devolvió una lista de issues")
    terms = list(config.get("include_terms", []))
    matching = [
        issue
        for issue in issues
        if "pull_request" not in issue and _contains_any(issue.get("title", ""), terms)
    ]
    snapshot_items = {
        str(issue["number"]): {"updated_at": issue.get("updated_at"), "state": issue.get("state")}
        for issue in matching
    }
    previous_items = previous.get("items", {})
    first_run = "items" not in previous
    observations = []
    for issue in reversed(matching):
        key = str(issue["number"])
        if first_run or snapshot_items[key] == previous_items.get(key):
            continue
        observations.append(
            Observation(
                source=config["display_id"],
                label=config["label"],
                change=f"issue #{key} nuevo o actualizado: {issue.get('title', '')}",
                link=issue.get("html_url", config["url"]),
                date=(issue.get("updated_at") or str(utc_today()))[:10],
                attention=True,
                highlighted=True,
            )
        )
    snapshot = {"items": snapshot_items}
    return Collection(config["display_id"], snapshot, observations, not first_run and snapshot != previous)


COLLECTORS: dict[str, Callable[[dict[str, Any], dict[str, Any]], Collection]] = {
    "atom": collect_atom,
    "env_atom_list": collect_env_atom_list,
    "html_block": collect_html_block,
    "lotl": collect_lotl,
    "github_issue": collect_github_issue,
    "github_issues": collect_github_issues,
}


def load_state(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"version": 1, "sources": {}}
    try:
        state = json.loads(path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError) as error:
        raise SystemExit(f"estado inválido en {path}: {error}") from error
    if state.get("version") != 1 or not isinstance(state.get("sources"), dict):
        raise SystemExit(f"estado incompatible en {path}")
    return state


def save_state(path: Path, sources: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {"version": 1, "updated_at": dt.datetime.now(dt.timezone.utc).isoformat(), "sources": sources}
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def _entry_line(observation: Observation) -> str:
    date = f" · {observation.date}" if observation.date else ""
    return (
        f"- [{observation.source}] {observation.label} · {observation.change} · "
        f"<{observation.link}>{date}"
    )


def render_report(
    year: int,
    week: int,
    observations: list[Observation],
    unchanged: list[str],
    errors: list[str],
    max_attention: int,
) -> str:
    attention = sorted(
        (item for item in observations if item.attention),
        key=lambda item: (not item.highlighted, item.source, item.date, item.change),
    )
    selected = attention[:max_attention]
    overflow = attention[max_attention:]
    archive = [item for item in observations if not item.attention] + [
        dataclasses.replace(item, change=f"[exceso del límite de atención] {item.change}")
        for item in overflow
    ]
    lines = [
        f"# Vigilancia — semana {year}-W{week:02d}",
        "",
        f"## Requieren atención (máx. {max_attention})",
    ]
    lines.extend(_entry_line(item) for item in selected)
    if not selected:
        lines.append("(ninguna)")
    lines.extend(["", "## Archivo (sin acción)"])
    lines.extend(_entry_line(item) for item in archive)
    if not archive:
        lines.append("(ninguna)")
    lines.extend(["", "## Fuentes sin cambios"])
    lines.append(", ".join(sorted(set(unchanged))) if unchanged else "(ninguna)")
    lines.extend(["", "## Errores de recolección"])
    lines.extend(f"- {error}" for error in errors)
    if not errors:
        lines.append("(ninguno)")
    lines.append("")
    return "\n".join(lines)


def run(config_path: Path, *, report_date: dt.date | None = None) -> Path:
    with config_path.open("rb") as handle:
        config = tomllib.load(handle)
    root = config_path.resolve().parents[2]
    settings = config["settings"]
    state_path = root / settings["state_path"]
    output_dir = root / settings["output_dir"]
    state = load_state(state_path)
    next_sources = dict(state["sources"])
    observations: list[Observation] = []
    unchanged: list[str] = []
    errors: list[str] = []
    error_sources: set[str] = set()

    for source_key, source_config in config.get("sources", {}).items():
        source = dict(source_config)
        display_id = source.get("display_id", source_key.split("_", 1)[0])
        source["display_id"] = display_id
        if not source.get("enabled", True):
            errors.append(f"[{display_id}] {source.get('label', source_key)} · no configurada: {source.get('reason', 'deshabilitada')}")
            error_sources.add(display_id)
            continue
        collector = COLLECTORS.get(source.get("kind"))
        if collector is None:
            errors.append(f"[{display_id}] {source.get('label', source_key)} · collector desconocido: {source.get('kind')}")
            error_sources.add(display_id)
            continue
        try:
            result = collector(source, state["sources"].get(source_key, {}))
        except Exception as error:  # source failures belong in the report, not in a silent log
            errors.append(f"[{display_id}] {source.get('label', source_key)} · {error}")
            error_sources.add(display_id)
            continue
        next_sources[source_key] = result.snapshot
        observations.extend(result.observations)
        if not result.changed:
            unchanged.append(display_id)

    date = report_date or utc_today()
    unchanged = [source for source in unchanged if source not in error_sources]
    iso = date.isocalendar()
    output_dir.mkdir(parents=True, exist_ok=True)
    report_path = output_dir / f"{iso.year}-W{iso.week:02d}.md"
    report_path.write_text(
        render_report(
            iso.year,
            iso.week,
            observations,
            unchanged,
            errors,
            int(settings.get("max_attention", 5)),
        ),
        encoding="utf-8",
    )
    save_state(state_path, next_sources)
    return report_path


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--config",
        type=Path,
        default=Path("tools/vigilancia/config.toml"),
        help="ruta al fichero TOML de fuentes",
    )
    parser.add_argument("--date", type=dt.date.fromisoformat, help="fecha UTC para una ejecución reproducible")
    arguments = parser.parse_args(argv)
    report = run(arguments.config, report_date=arguments.date)
    print(report)
    return 0


if __name__ == "__main__":
    sys.exit(main())
