#!/usr/bin/env python3
"""Profile-level audit of inference links mediated by OmegaClaw's LLM loop.

This is demo code.  It deliberately does not import or modify acta-core: the meaning of an
inference premise and of the harness transform belongs above Core.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any


SCHEMA = "acta.c2.inference-link-input.v0"
REPORT_SCHEMA = "acta.c2.inference-link-report.v0"
CHAIN_CODE = "TR-CHAIN-MEDIATED"
NONDET_CODE = "TR-NONDET-INPUT"
SELECTION_DISCLAIMER = (
    "la selección de qué conclusiones se realimentan es del productor y este dossier "
    "no la acota"
)
NORMALIZE_DECLARATION = {"encoding": "utf-8", "errors": "ignore"}
STRING_SAFE_DECLARATION = {
    "double_double_quote": "_quote_",
    "newline": "_newline_",
    "apostrophe": "_apostrophe_",
}
LAST_CHARS_DECLARATION = {"side_retained": "tail", "unit": "characters"}
SHA256_RE = re.compile(r"^sha256:[0-9a-f]{64}$")


class InputError(ValueError):
    """The demo input cannot support the claimed comparison."""


def _require(mapping: dict[str, Any], key: str, expected: type) -> Any:
    value = mapping.get(key)
    # bool subclasses int in Python. Exact types keep true/false out of numeric evidence.
    if type(value) is not expected:
        raise InputError(f"{key!r} must be {expected.__name__}")
    return value


def _require_nonempty(mapping: dict[str, Any], key: str) -> str:
    value = _require(mapping, key, str)
    if not value:
        raise InputError(f"{key!r} must not be empty")
    return value


def _canonical_sha256(value: Any) -> str:
    payload = json.dumps(
        value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")
    return "sha256:" + hashlib.sha256(payload).hexdigest()


def _require_sha256(mapping: dict[str, Any], key: str) -> str:
    value = _require(mapping, key, str)
    if not SHA256_RE.fullmatch(value):
        raise InputError(f"{key!r} must be sha256:<64 lowercase hex>")
    return value


def _transform_descriptor(harness: dict[str, Any]) -> dict[str, Any]:
    return {
        "transform_order": harness["transform_order"],
        "normalize_string": harness["normalize_string"],
        "string-safe": harness["string-safe"],
        "last_chars": harness["last_chars"],
        "maxFeedback": harness["maxFeedback"],
    }


def _harness_config_descriptor(harness: dict[str, Any]) -> dict[str, Any]:
    return {
        **_transform_descriptor(harness),
        "maxHistory": harness["maxHistory"],
        "source_ref": harness["source_ref"],
        "source_hashes": harness["source_hashes"],
    }


def _source_hashes(container: dict[str, Any]) -> dict[str, str]:
    hashes = _require(container, "source_hashes", dict)
    expected = {
        "helper_py",
        "loop_metta_base",
        "utils_metta",
        "loop_metta_probe",
    }
    if set(hashes) != expected:
        raise InputError(f"source_hashes must contain exactly {sorted(expected)!r}")
    return {key: _require_sha256(hashes, key) for key in sorted(expected)}


def verify_transform_binding(
    context: dict[str, Any], harness: dict[str, Any]
) -> dict[str, Any]:
    kind = _require(context, "kind", str)
    if kind != "verification-envelope":
        raise InputError("S6 accepts only a synthetic verification-envelope; integrate the "
                         "ADR-004 verifier before claiming a historical lockfile")
    historical = _require(context, "historical_lockfile_present", bool)
    if historical:
        raise InputError("S6 cannot verify a historical ADR-004 lockfile")
    binding = _require(context, "lockfile_binding", dict)
    declared_sources = _source_hashes(binding)
    declared_config = _require_sha256(binding, "harness_config_sha256")
    declared_transform = _require_sha256(binding, "transform_sha256")
    harness_sources = _source_hashes(harness)
    computed_config = _canonical_sha256(_harness_config_descriptor(harness))
    computed_transform = _canonical_sha256(_transform_descriptor(harness))
    if declared_sources != harness_sources:
        raise InputError("lockfile binding does not match harness source_hashes")
    if declared_config != computed_config:
        raise InputError("lockfile binding harness_config_sha256 mismatch")
    if declared_transform != computed_transform:
        raise InputError("lockfile binding transform_sha256 mismatch")
    return {
        "kind": kind,
        "historical_lockfile_present": historical,
        "basis": _require_nonempty(context, "basis"),
        "adr004_historical_lockfile_verified": False,
        "lockfile_binding": {
            "source_hashes": harness_sources,
            "harness_config_sha256": computed_config,
            "transform_sha256": computed_transform,
            "transform_hash_recomputed": True,
        },
    }


def _process_ref(container: dict[str, Any]) -> dict[str, str]:
    process_ref = _require(container, "process_ref", dict)
    return {
        "process_id": _require_nonempty(process_ref, "process_id"),
        "process_type": _require_nonempty(process_ref, "process_type"),
    }


def harness_transform(raw: str, harness: dict[str, Any]) -> str:
    """Apply declared T in measured code order.

    T is normalize_string, then string-safe, then last_chars(maxFeedback).  JSON transports
    Unicode strings, but the encode/decode step is retained so lone surrogates are discarded
    in the same way as the measured ``errors=ignore`` helper.
    """

    declared_order = _require(harness, "transform_order", list)
    expected_order = ["normalize_string", "string-safe", "last_chars(maxFeedback)"]
    if declared_order != expected_order:
        raise InputError(
            "harness transform_order must declare the measured order "
            f"{expected_order!r}"
        )
    if harness.get("normalize_string") != NORMALIZE_DECLARATION:
        raise InputError(
            f"normalize_string must be declared as {NORMALIZE_DECLARATION!r}"
        )
    if harness.get("string-safe") != STRING_SAFE_DECLARATION:
        raise InputError(f"string-safe must be declared as {STRING_SAFE_DECLARATION!r}")
    if harness.get("last_chars") != LAST_CHARS_DECLARATION:
        raise InputError(f"last_chars must be declared as {LAST_CHARS_DECLARATION!r}")
    max_feedback = _require(harness, "maxFeedback", int)
    if max_feedback <= 0:
        raise InputError("maxFeedback must be positive")
    _require(harness, "maxHistory", int)
    _require_nonempty(harness, "source_ref")
    _source_hashes(harness)

    normalized = raw.encode("utf-8", errors="ignore").decode(
        "utf-8", errors="ignore"
    )
    safe = (
        normalized.replace('""', "_quote_")
        .replace("\n", "_newline_")
        .replace("'", "_apostrophe_")
    )
    return safe[-max_feedback:]


def audit_link(link: dict[str, Any], harness: dict[str, Any]) -> dict[str, Any]:
    link_id = _require(link, "link_id", str)
    conclusion = _require(link, "conclusion", dict)
    premise = _require(link, "premise", dict)

    conclusion_bytes = _require(conclusion, "serialized", str).encode("utf-8")
    transformed = harness_transform(conclusion["serialized"], harness)
    transformed_bytes = transformed.encode("utf-8")
    premise_bytes = _require(premise, "serialized", str).encode("utf-8")

    conclusion_process = _process_ref(conclusion)
    premise_process = _process_ref(premise)
    same_process = conclusion_process == premise_process
    chronology = _require(link, "chronos", dict)
    order_verified = _require(chronology, "order_verified", bool)
    conclusion_index = _require(chronology, "conclusion_index", int)
    premise_index = _require(chronology, "premise_index", int)
    if conclusion_index < 0 or premise_index < 0:
        raise InputError(f"{link_id}: Chronos indices must be non-negative")
    precedes = order_verified and conclusion_index < premise_index
    transformed_bytes_equal = premise_bytes == transformed_bytes

    predicates = {
        "bytes_premise_eq_raw_conclusion": premise_bytes == conclusion_bytes,
        "bytes_premise_eq_T_conclusion": transformed_bytes_equal,
        "same_process_ref": same_process,
        "conclusion_precedes_premise_in_verified_chronos_order": precedes,
    }
    faithful = transformed_bytes_equal and same_process and precedes
    nondeterministic_origin = _require(premise, "nondeterministic_origin", str)
    if nondeterministic_origin not in {"selected", "formulated", "none"}:
        raise InputError(
            f"{link_id}: nondeterministic_origin must be selected, formulated, or none"
        )
    context_evidence = _require(link, "context_evidence", dict)
    exact_context = _require_nonempty(
        context_evidence, "last_skill_use_results_exact"
    )
    source = _require(link, "source", dict)
    source_evidence = {
        "measurement": _require_nonempty(source, "measurement"),
        "rawlog": _require_nonempty(source, "rawlog"),
        "conclusion_line": _require(source, "conclusion_line", int),
        "context_line": _require(source, "context_line", int),
        "premise_call_line": _require(source, "premise_call_line", int),
    }
    if any(source_evidence[key] <= 0 for key in (
        "conclusion_line", "context_line", "premise_call_line"
    )):
        raise InputError(f"{link_id}: source line numbers must be positive")

    return {
        "link_id": link_id,
        "classification": "faithful" if faithful else "free",
        "predicates": predicates,
        "failed_faithful_predicates": [
            name
            for name in (
                "bytes_premise_eq_T_conclusion",
                "same_process_ref",
                "conclusion_precedes_premise_in_verified_chronos_order",
            )
            if not predicates[name]
        ],
        "process_ref": conclusion_process if same_process else {
            "conclusion": conclusion_process,
            "premise": premise_process,
        },
        "evidence_strings": {
            "raw_conclusion": conclusion["serialized"],
            "transformed_conclusion_T_C": transformed,
            "last_skill_use_results_exact": exact_context,
            "next_premise": premise["serialized"],
        },
        "premise_nondeterministic_origin": nondeterministic_origin,
        "source": source_evidence,
    }


def audit_document(document: dict[str, Any]) -> dict[str, Any]:
    if document.get("schema") != SCHEMA:
        raise InputError(f"schema must be {SCHEMA!r}")
    harness = _require(document, "harness", dict)
    # Validate T before hashing it, then require its computed hashes to match the envelope.
    harness_transform("", harness)
    verification = verify_transform_binding(
        _require(document, "verification_context", dict), harness
    )
    mediator = _require(document, "mediator", dict)
    mediator_present = _require(mediator, "present", bool)
    mediator_nondeterministic = _require(mediator, "nondeterministic", bool)
    if not mediator_present and mediator_nondeterministic:
        raise InputError("an absent mediator cannot be declared nondeterministic")
    mediator_kind = _require(mediator, "kind", str)
    if mediator_present and not mediator_kind:
        raise InputError("a present mediator requires a non-empty kind")
    scope = _require(document, "measurement_scope", dict)
    description = _require(scope, "description", str)
    interpretation = _require(scope, "interpretation", str)
    explicit_copy = _require(scope, "explicit_byte_copy_instruction", bool)
    links = _require(document, "links", list)
    if not links:
        raise InputError("links must not be empty")

    audited = [audit_link(_require_link(item), harness) for item in links]
    classification_basis = "contract-validation-over-synthetic-verification-envelope"
    for link in audited:
        link["classification_basis"] = classification_basis
    faithful = sum(link["classification"] == "faithful" for link in audited)
    total = len(audited)
    nondeterministic_input = any(
        link["premise_nondeterministic_origin"] in {"selected", "formulated"}
        for link in audited
    )
    if explicit_copy:
        scope_line = (
            f"Techo empírico bajo instrucción explícita de copia: {faithful} de {total}"
        )
    else:
        scope_line = f"Muestra observada: {faithful} de {total}"
    statement = (
        f"{scope_line} premisas coinciden byte a byte con T(C), pertenecen al mismo "
        "process_ref y siguen a C en el orden Chronos declarado como verificado en la "
        "envoltura sintética; "
        f"{SELECTION_DISCLAIMER}."
    )

    chain_condition_emitted = mediator_present and mediator_nondeterministic
    conditions = []
    if chain_condition_emitted:
        conditions.append(
            {
                "code": CHAIN_CODE,
                "meaning": (
                    "la cadena contiene eslabones unidos por un mediador no determinista"
                ),
            }
        )
    if nondeterministic_input:
        conditions.append(
            {
                "code": NONDET_CODE,
                "meaning": (
                    "al menos una premisa está declarada como formulada o seleccionada "
                    "por un proceso no determinista; su fiabilidad sigue siendo una "
                    "afirmación del productor"
                ),
            }
        )

    chain_mediation = {
        "condition_emitted": chain_condition_emitted,
        "faithful": faithful,
        "free": total - faithful,
        "total": total,
        "explicit_byte_copy_instruction": explicit_copy,
        "measurement_scope": description,
        "interpretation": interpretation,
        "classification_basis": classification_basis,
        "required_selection_disclaimer": SELECTION_DISCLAIMER,
        "statement": statement,
    }
    if chain_condition_emitted:
        chain_mediation["code"] = CHAIN_CODE

    return {
        "schema": REPORT_SCHEMA,
        "verification_context": verification,
        "mediator": {
            "present": mediator_present,
            "nondeterministic": mediator_nondeterministic,
            "kind": mediator_kind,
        },
        "harness_transform": {
            "source_ref": harness["source_ref"],
            "order": harness["transform_order"],
            "normalize_string": harness["normalize_string"],
            "string-safe": harness["string-safe"],
            "last_chars": harness["last_chars"],
            "maxFeedback": harness["maxFeedback"],
            "maxHistory": harness["maxHistory"],
        },
        "conditions": conditions,
        # The count and its mandatory caveat intentionally share one object.
        "chain_mediation": chain_mediation,
        "links": audited,
        "narrative_evidence": document.get("narrative_evidence", []),
    }


def _require_link(value: Any) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise InputError("each links entry must be an object")
    return value


def render_text(report: dict[str, Any]) -> str:
    chain = report["chain_mediation"]
    rows = [
        "ACTA C2 — auditoría de cadena mediada",
        chain["statement"],
        f"Alcance: {chain['measurement_scope']}",
        f"Interpretación: {chain['interpretation']}",
        "Condiciones: " + ", ".join(item["code"] for item in report["conditions"]),
    ]
    rows.extend(
        f"- {link['link_id']}: {link['classification']}"
        + (
            ""
            if not link["failed_faithful_predicates"]
            else " (fallan: " + ", ".join(link["failed_faithful_predicates"]) + ")"
        )
        for link in report["links"]
    )
    return "\n".join(rows) + "\n"


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path, help="JSON input with mediated links")
    parser.add_argument(
        "--format", choices=("json", "text"), default="json", dest="output_format"
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    try:
        document = json.loads(args.input.read_text(encoding="utf-8"))
        report = audit_document(document)
    except (OSError, json.JSONDecodeError, InputError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2
    if args.output_format == "text":
        print(render_text(report), end="")
    else:
        print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
