#!/usr/bin/env python3
"""E0 capture driver — OmegaClaw under the deterministic LLM mock.

Standalone: uses only OmegaClaw's own Autotests/mock harness (stdlib) and the
docker CLI. No pytest, no third-party packages.

Everything is written raw to an output directory: full strings, with
timestamps. No summarising here — the summary is the derived decision, and
that goes in research/E0-resultados.md by hand.
"""
import json
import os
import subprocess
import sys
import time
from datetime import datetime, timezone

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "OmegaClaw-Core", "Autotests", "mock"))
sys.path.insert(0, os.path.join(HERE, "OmegaClaw-Core", "Autotests"))

from llm import LlmMockController, LLM_MOCK_PORT      # noqa: E402
from comm import CommMockServer, COMM_MOCK_PORT       # noqa: E402

CONTAINER = os.environ.get("OMEGACLAW_CONTAINER", "omegaclaw")
OUTDIR = os.environ.get("E0_OUTDIR", os.path.join(HERE, "e0-logs"))


def ts():
    return datetime.now(timezone.utc).isoformat()


def log(msg):
    print(f"[{ts()}] {msg}", flush=True)


def docker_logs():
    r = subprocess.run(["docker", "logs", CONTAINER],
                       capture_output=True, text=True)
    return (r.stdout or "") + (r.stderr or "")


def wait_for_loop(timeout=300):
    """The first numeric CHARS_SENT line marks the end of init."""
    import re
    deadline = time.time() + timeout
    while time.time() < deadline:
        if re.search(r"CHARS_SENT: \d+", docker_logs()):
            return True
        time.sleep(2)
    return False


def record(name, payload):
    os.makedirs(OUTDIR, exist_ok=True)
    path = os.path.join(OUTDIR, name)
    with open(path, "w", encoding="utf-8") as f:
        f.write(payload if isinstance(payload, str)
                else json.dumps(payload, indent=2, ensure_ascii=False))
    log(f"recorded {path} ({len(payload)} chars)")
    return path


def run_metta(llm, comm, tag, expr, settle=45):
    """Drive one turn: the mocked LLM answers with exactly (metta "<expr>").

    Returns the raw docker log slice produced by this turn, untruncated.
    """
    before = len(docker_logs())
    prompt = f"[E0-{tag}] evaluate this"
    answer = f'(metta "{expr}")'
    log(f"{tag}: set_answer -> {answer}")
    llm.set_answer(prompt, answer)
    if not comm.send_message(prompt):
        raise RuntimeError(f"{tag}: comm.send_message failed")
    log(f"{tag}: prompt delivered, settling {settle}s")
    time.sleep(settle)
    after = docker_logs()
    slice_ = after[before:]
    record(f"{tag}.rawlog", f"# expr: {expr}\n# captured_at: {ts()}\n\n{slice_}")
    return slice_


def main():
    os.makedirs(OUTDIR, exist_ok=True)
    log(f"output dir: {OUTDIR}")
    llm = LlmMockController(("0.0.0.0", LLM_MOCK_PORT))
    comm = CommMockServer(("0.0.0.0", COMM_MOCK_PORT))
    try:
        log("waiting for agent loop to come up")
        if not wait_for_loop():
            raise RuntimeError("agent loop did not start within 300s")
        log("agent loop is up")
        if not llm.ping(30):
            raise RuntimeError("llm mock did not answer ping")
        log("llm mock reachable from the container")

        # --- Capture 1: same inference, twice in the same session ---
        DEDUCTION = ('(|- ((--> golden_retriever friendly) (stv 1.0 0.9)) '
                     '((--> friendly family_friendly) (stv 0.9 0.85)))')
        run_metta(llm, comm, "c1-run1", DEDUCTION)
        run_metta(llm, comm, "c1-run2", DEDUCTION)

        # --- Capture 1b: same conclusion, two truth values ---
        REVISION = ('(|- ((--> wolf animal) (stv 1.0 0.45)) '
                    '((--> wolf animal) (stv 1.0 0.45)))')
        run_metta(llm, comm, "c1-dup", REVISION)

        # --- Capture 4: large result set through the three stages ---
        BIG = '(collapse (range 1 20000))'  # ~110 KB de salida: por encima de maxFeedback=50000
        run_metta(llm, comm, "c4-big", BIG, settle=60)

        log("done — restart the container and re-run c1-run3 for the "
            "post-restart arm of capture 1")
    finally:
        comm.stop(5)
        llm.stop(5)


if __name__ == "__main__":
    main()
