#!/usr/bin/env python3
"""Capture 1, third arm: same inference after a container restart."""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                "OmegaClaw-Core", "Autotests", "mock"))
import e0_capture as E
from llm import LlmMockController, LLM_MOCK_PORT
from comm import CommMockServer, COMM_MOCK_PORT

llm = LlmMockController(("0.0.0.0", LLM_MOCK_PORT))
comm = CommMockServer(("0.0.0.0", COMM_MOCK_PORT))
try:
    E.log("waiting for loop after restart")
    assert E.wait_for_loop(), "loop did not come up"
    assert llm.ping(30), "llm mock unreachable"
    DEDUCTION = ('(|- ((--> golden_retriever friendly) (stv 1.0 0.9)) '
                 '((--> friendly family_friendly) (stv 0.9 0.85)))')
    E.run_metta(llm, comm, "c1-run3-postrestart", DEDUCTION)
finally:
    comm.stop(5); llm.stop(5)
