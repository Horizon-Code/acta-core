#!/usr/bin/env python3
"""E0 capture 3 — faithful/free ratio on a real two-hop chain.

Real LLM (Anthropic), scripted delivery over the test comm channel. The LLM is
NOT scripted: what it drags from hop 1 into hop 2 is exactly what is measured.
"""
import os, re, sys, time
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                "OmegaClaw-Core", "Autotests", "mock"))
import e0_capture as E
from comm import CommMockServer, COMM_MOCK_PORT

TASKS = [
    ("t1", "golden_retriever", "friendly", "family_friendly", "good_pet"),
    ("t2", "sparrow",          "bird",     "animal",          "living_thing"),
    ("t3", "copper",           "metal",    "conductor",       "useful_material"),
    ("t4", "oak",              "tree",     "plant",           "organism"),
]

def instruction(a, b, c, d):
    return (
        f"Do exactly two separate metta skill calls, one per turn, nothing else. "
        f"First call: (|- ((--> {a} {b}) (stv 1.0 0.9)) ((--> {b} {c}) (stv 0.9 0.85))). "
        f"Wait for its result. Second call: take the conclusion about {a} and {c} "
        f"that the first call returned, use it verbatim as the first premise, and "
        f"pair it with ((--> {c} {d}) (stv 0.9 0.8)). Do not restate or reformat "
        f"the conclusion; copy it exactly as returned."
    )

def split_top(s):
    """Split a MeTTa list body '(x) (y)' into its top-level elements."""
    out, depth, start = [], 0, None
    for i, ch in enumerate(s):
        if ch == "(":
            if depth == 0: start = i
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0 and start is not None:
                out.append(s[start:i+1]); start = None
    return out

def metta_calls(log):
    """Every (metta "...") the agent issued, with its COMMAND_RETURN payload."""
    return re.findall(r'COMMAND_RETURN: \(\(metta "(.*?)"\) "?(.*?)"?\)\)', log, re.S)

def main():
    # The agent's comm client connects OUT during initChannels. The server has
    # to be listening before the container comes up, or the channel never binds.
    comm = CommMockServer(("0.0.0.0", COMM_MOCK_PORT))
    rows = []
    try:
        import subprocess
        E.log("restarting container; server already listening")
        subprocess.run(["docker", "restart", "omegaclaw"], capture_output=True)
        E.log("waiting for the agent's version handshake on the test channel")
        deadline = time.time() + 300
        while time.time() < deadline:
            if comm.getLastMessage():
                E.log(f"channel bound: {comm.getLastMessage()!r}"); break
            time.sleep(3)
        else:
            raise RuntimeError("test channel never bound")
        time.sleep(10)
        for tag, a, b, c, d in TASKS:
            before = len(E.docker_logs())
            msg = instruction(a, b, c, d)
            E.log(f"{tag}: sending two-hop task")
            if not comm.send_message(msg):
                E.log(f"{tag}: delivery failed"); continue
            time.sleep(150)
            slice_ = E.docker_logs()[before:]
            E.record(f"c3-{tag}.rawlog", f"# task: {msg}\n# at: {E.ts()}\n\n{slice_}")
            calls = metta_calls(slice_)
            E.log(f"{tag}: {len(calls)} metta call(s) observed")
            rows.append((tag, a, c, calls))
    finally:
        comm.stop(5)

    print("\n================ CAPTURA 3 — eslabones ================")
    for tag, a, c, calls in rows:
        print(f"\n--- {tag} ---")
        for i, (arg, ret) in enumerate(calls):
            print(f"  hop{i+1} IN : {arg[:200]}")
            print(f"  hop{i+1} OUT: {ret[:200]}")
        if len(calls) >= 2:
            hop1_out = calls[0][1]
            hop2_in = calls[1][0]
            concls = split_top(hop1_out)
            prems = split_top(hop2_in[hop2_in.find("|-")+2:] if "|-" in hop2_in else hop2_in)
            print(f"  conclusiones comprometibles del salto 1: {len(concls)}")
            for p in prems:
                literal = p in concls
                print(f"  premisa salto 2 {'FIEL  ' if literal else 'LIBRE '}: {p[:160]}")
                if not literal:
                    for cc in concls:
                        if cc[:25] == p[:25]:
                            print(f"      más parecida en salto 1 : {cc[:160]}")

if __name__ == "__main__":
    main()
