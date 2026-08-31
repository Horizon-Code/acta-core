#!/usr/bin/env python3
"""E0 capture 3: five real-LLM two-hop chains over the test channel.

The test channel is only transport. Anthropic remains the unscripted mediator.
This driver starts the host RPC endpoint before the existing container, sends
one task at a time, records complete Docker-log slices, and stops the container
as soon as the five observations are present.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path


HERE = Path(__file__).resolve().parent
SUBSTRATE = Path(os.environ.get("E0_SUBSTRATE", HERE.parents[2] / "e0-substrate"))
MOCK = SUBSTRATE / "OmegaClaw-Core" / "Autotests" / "mock"
sys.path.insert(0, str(MOCK))

from comm import COMM_MOCK_PORT, CommMockServer  # noqa: E402


CONTAINER = os.environ.get("OMEGACLAW_CONTAINER", "omegaclaw")
OUTDIR = Path(os.environ.get("E0_OUTDIR", HERE))

TASKS = [
    ("t1", "golden_retriever", "friendly", "family_friendly", "good_pet"),
    ("t2", "sparrow", "bird", "animal", "living_thing"),
    ("t3", "copper", "metal", "conductor", "useful_material"),
    ("t4", "oak", "tree", "plant", "organism"),
    ("t5", "salmon", "fish", "aquatic_animal", "living_thing"),
]


def ts() -> str:
    return datetime.now(timezone.utc).isoformat()


def note(message: str) -> None:
    print(f"[{ts()}] {message}", flush=True)


def docker(*args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["docker", *args], check=check, capture_output=True, text=True
    )


def logs() -> str:
    result = docker("logs", CONTAINER, check=False)
    return (result.stdout or "") + (result.stderr or "")


def instruction(a: str, b: str, c: str, d: str) -> str:
    return (
        "Perform exactly two inference hops with the metta skill. "
        f"Hop 1 must be (|- ((--> {a} {b}) (stv 1.0 0.9)) "
        f"((--> {b} {c}) (stv 0.9 0.85))). Wait for its result. "
        f"In a later turn, hop 2 must use the returned conclusion about {a} "
        f"and {c} as its first premise and ((--> {c} {d}) (stv 0.9 0.8)) "
        "as its second premise. Copy that first premise byte-for-byte from "
        "LAST_SKILL_USE_RESULTS. Do not call metta for anything else. After "
        "hop 2, send DONE and make no more tool calls."
    )


COMMAND_RE = re.compile(
    r'COMMAND_RETURN: \(\(metta "(.*?)"\)\s+"?(.*?)"?\)\)', re.S
)


def observed_calls(text: str) -> list[tuple[str, str]]:
    return COMMAND_RE.findall(text)


def wait_for(predicate, timeout: int, interval: float = 2.0) -> bool:
    deadline = time.time() + timeout
    while time.time() < deadline:
        if predicate():
            return True
        time.sleep(interval)
    return False


def write_raw(tag: str, task: str, captured: str) -> Path:
    OUTDIR.mkdir(parents=True, exist_ok=True)
    path = OUTDIR / f"c3-{tag}.rawlog"
    payload = f"# task: {task}\n# captured_at: {ts()}\n\n{captured}"
    path.write_text(payload, encoding="utf-8")
    note(f"recorded {path} ({len(payload)} chars)")
    return path


def main() -> int:
    server = CommMockServer(("0.0.0.0", COMM_MOCK_PORT))
    summary: list[dict[str, object]] = []
    try:
        note("starting existing Anthropic/test container after channel server")
        docker("start", CONTAINER)
        if not wait_for(lambda: bool(server.getLastMessage()), 180):
            raise RuntimeError("test-channel version handshake not observed")
        note("test channel bound; Anthropic remains the configured provider")

        for tag, a, b, c, d in TASKS:
            task = instruction(a, b, c, d)
            before = len(logs())
            note(f"{tag}: delivering two-hop task")
            if not server.send_message(task, timeout=20):
                raise RuntimeError(f"{tag}: message delivery failed")

            def has_two_calls() -> bool:
                return len(observed_calls(logs()[before:])) >= 2

            if not wait_for(has_two_calls, 150):
                captured = logs()[before:]
                write_raw(tag, task, captured)
                raise RuntimeError(f"{tag}: fewer than two metta calls in 150 s")

            time.sleep(4)
            captured = logs()[before:]
            path = write_raw(tag, task, captured)
            calls = observed_calls(captured)
            summary.append(
                {
                    "task": tag,
                    "captured_at": ts(),
                    "rawlog": path.name,
                    "metta_call_count": len(calls),
                    "calls": [
                        {"input": call_input, "command_return": call_output}
                        for call_input, call_output in calls
                    ],
                }
            )
            note(f"{tag}: observed {len(calls)} metta calls")

        summary_path = OUTDIR / "c3-summary.raw.json"
        summary_path.write_text(
            json.dumps(summary, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
        note(f"recorded {summary_path}")
        return 0
    finally:
        docker("stop", CONTAINER, check=False)
        server.stop(5)


if __name__ == "__main__":
    raise SystemExit(main())
