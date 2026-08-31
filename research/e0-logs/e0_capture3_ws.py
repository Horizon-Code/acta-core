#!/usr/bin/env python3
"""Capture E0/3 through a local WebSocket channel and a real LLM.

The API key is inherited from ``ANTHROPIC_API_KEY`` and is only forwarded to
Docker by name (``docker run -e ANTHROPIC_API_KEY``).  It is never serialized
by this driver.  Raw container log slices are the evidence; extraction and
classification live in a separate analysis step.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path


HERE = Path(__file__).resolve().parent
SUBSTRATE = Path(
    os.environ.get("E0_SUBSTRATE", HERE.parents[2] / "e0-substrate")
)
WS_HELPERS = SUBSTRATE / "OmegaClaw-Core" / "Autotests" / "mock_websocket"
sys.path.insert(0, str(WS_HELPERS))

from ws_driver import WsMockDriver  # noqa: E402


CONTAINER = "omegaclaw"
IMAGE = os.environ.get("E0_IMAGE", "omegaclaw:e0-c3")
MODEL = "claude-haiku-4-5-20251001"
PORT = int(os.environ.get("E0_WS_PORT", "9876"))
TOKEN = "e0-capture3-local"
OUTDIR = Path(os.environ.get("E0_OUTDIR", HERE))
MEMORY_VOLUME = os.environ.get(
    "E0_MEMORY_VOLUME", "omegaclaw-e0-c3-r2-memory"
)
RUN_LABEL = os.environ.get("E0_RUN_LABEL", "r2")

TASKS = [
    ("t1", "golden_retriever", "friendly", "family_friendly", "good_pet"),
    ("t2", "sparrow", "bird", "animal", "living_thing"),
    ("t3", "copper", "metal", "conductor", "useful_material"),
    ("t4", "oak", "tree", "plant", "organism"),
    ("t5", "salmon", "fish", "aquatic_animal", "living_thing"),
    ("t6", "granite", "rock", "mineral", "material"),
    ("t7", "rose", "flower", "plant", "organism"),
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


def start_container() -> None:
    if not os.environ.get("ANTHROPIC_API_KEY"):
        raise RuntimeError("ANTHROPIC_API_KEY is not set")
    docker("rm", "-f", CONTAINER, check=False)
    docker("volume", "create", MEMORY_VOLUME)
    result = docker(
        "run",
        "-d",
        "-i",
        "-t",
        "--name",
        CONTAINER,
        "--security-opt",
        "no-new-privileges:true",
        "--init",
        "--add-host=host.docker.internal:host-gateway",
        "--tmpfs",
        "/tmp:size=64m,mode=1777",
        "--tmpfs",
        "/var/tmp:size=64m,mode=1777",
        "--tmpfs",
        "/run:size=16m,mode=755",
        "--volume",
        f"{MEMORY_VOLUME}:/PeTTa/repos/OmegaClaw-Core/memory",
        "-e",
        "ANTHROPIC_API_KEY",
        "-e",
        "OMEGACLAW_AUTH_SECRET=0000",
        IMAGE,
        "commchannel=websocket",
        "provider=Anthropic",
        "embeddingprovider=Local",
        "securityPolicyPath=/PeTTa/repos/OmegaClaw-Core/profile/policy.yaml",
        f"WS_URL=ws://172.17.0.1:{PORT}",
        f"WS_TOKEN={TOKEN}",
        f"model={MODEL}",
    )
    note(f"container started: {result.stdout.strip()[:12]}")


def instruction(tag: str, a: str, b: str, c: str, d: str) -> str:
    hop1 = (
        f"(car-atom (|- ((--> {a} {b}) (stv 1.0 0.9)) "
        f"((--> {b} {c}) (stv 0.9 0.85))))"
    )
    hop2_suffix = f"((--> {c} {d}) (stv 0.9 0.8))"
    return (
        f"[E0-{tag}] Perform exactly two inference hops in two different loop "
        "iterations. In the first iteration issue exactly one skill call, metta "
        f"{hop1}. Then wait for its actual result. In a later iteration, read "
        "LAST_SKILL_USE_RESULTS and copy the returned conclusion byte-for-byte "
        "as the first premise of a second |- call; use "
        f"{hop2_suffix} as its second premise and wrap that second |- call in "
        "car-atom too. Do not infer or retype the expected first result yourself; "
        "copy the observed result. Do not call metta for anything else. After the "
        f"second result, send exactly [E0-{tag}-DONE]."
    )


def raw_eval_count(text: str) -> int:
    return text.count("| e0_capture3 | (RAW_EVAL: (metta ")


def has_valid_chain(text: str, a: str, b: str, c: str, d: str) -> bool:
    """True once a task-specific hop 1 is followed by its linked hop 2."""
    metta_lines = [
        line
        for line in text.splitlines()
        if "| e0_capture3 | (RAW_EVAL: (metta " in line
    ]
    hop1_at = None
    for index, line in enumerate(metta_lines):
        if f"(--> {a} {b})" in line and f"(--> {b} {c})" in line:
            hop1_at = index
            break
    if hop1_at is None:
        return False
    return any(
        f"(--> {a} {c})" in line and f"(--> {c} {d})" in line
        for line in metta_lines[hop1_at + 1 :]
    )


def write_raw(tag: str, task: str, captured: str) -> Path:
    OUTDIR.mkdir(parents=True, exist_ok=True)
    path = OUTDIR / f"c3-{tag}.rawlog"
    path.write_text(
        f"# task: {task}\n# captured_at: {ts()}\n\n{captured}",
        encoding="utf-8",
    )
    note(f"{tag}: recorded {path.name} ({path.stat().st_size} bytes)")
    return path


def main() -> int:
    ws = WsMockDriver(PORT, token=TOKEN)
    manifest: list[dict[str, object]] = []
    try:
        start_container()
        if not ws.wait_for_connection(timeout=180):
            raise RuntimeError("WebSocket channel did not connect in 180 s")
        note("WebSocket channel connected")
        ws.drain_agent_replies(max_wait=1)

        selected_tags = {
            tag.strip()
            for tag in os.environ.get("E0_TASK_TAGS", "").split(",")
            if tag.strip()
        }
        tasks = [task for task in TASKS if not selected_tags or task[0] in selected_tags]
        for tag, a, b, c, d in tasks:
            task = instruction(tag, a, b, c, d)
            before = len(logs())
            note(f"{tag}: injecting two-hop task")
            ws.inject_user_message(task)

            deadline = time.time() + 240
            captured = ""
            while time.time() < deadline:
                captured = logs()[before:]
                if has_valid_chain(captured, a, b, c, d):
                    time.sleep(3)
                    captured = logs()[before:]
                    break
                time.sleep(1)
            else:
                write_raw(tag, task, captured)
                raise RuntimeError(f"{tag}: no valid linked pair of metta calls")

            path = write_raw(tag, task, captured)
            replies = ws.drain_agent_replies(max_wait=1)
            manifest.append(
                {
                    "task": tag,
                    "captured_at": ts(),
                    "rawlog": path.name,
                    "raw_eval_count": raw_eval_count(captured),
                    "valid_linked_pair": has_valid_chain(captured, a, b, c, d),
                    "agent_replies": [text for _, text in replies],
                }
            )

        manifest_path = OUTDIR / f"c3-{RUN_LABEL}-manifest.raw.json"
        manifest_path.write_text(
            json.dumps(manifest, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
        note(f"recorded {manifest_path.name}")
        return 0
    finally:
        docker("stop", CONTAINER, check=False)
        ws.stop(5)


if __name__ == "__main__":
    raise SystemExit(main())
