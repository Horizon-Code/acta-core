# C2 — OmegaClaw

Status: **S6 implementation complete; external-custody gate still open**. ADR-009/Profile is
Accepted; only actual independent retention or a verified external anchor remains pending.

C2 contains two independent demonstrators. Both remain outside Core and Protocol:

- `artifact1/` seals exact `memory/history.metta` records as
  `ai_agent.context_committed` events, builds Chronos/Merkle/receipt evidence and detects a
  later silent deletion against an epoch root placed outside the operator directory. The
  local topology is a simulation, not independent custody.
- `artifact2/` audits mediated inference links with ADR-008's
  `bytes(P) == bytes(T(C))` predicate, same-process and verified-Chronos checks, and emits the
  faithful/free result with its inseparable selection disclaimer.

## Verified executions

Artifact 1 was run both with its deterministic fixture and with the real OmegaClaw history
retained from E0 in the local `omegaclaw-memory` Docker volume. The source volume was mounted
read-only and copied to a temporary directory before mutation.

The retained real history contained 100 timestamped records (21,642 bytes). Together with
the two Profile opening events, it sealed as 102 lifecycle bundles to epoch root
`bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba`.
After deleting the unique `[E0-c1-dup]` record from the copy, the OmegaClaw-form check still
passed with 99 readable records while the ACTA comparison returned detection exit code 2 for
sequence change, the missing committed record and count change. The original volume was not
modified.

Artifact 2 passed its E0 reference vector with five links classified faithful over an
explicitly synthetic verification envelope. That result is only the measured ceiling of five
isomorphic tasks under explicit byte-copy instruction; it is not a mediator rate or a claim
that E0 already had ACTA receipts/lockfiles. Its source-level interception patch is derived
from OmegaClaw commit
`642c53676cf795cb7a0030823b36018c029b1416` and records `RAW_EVAL` before
`normalize_string`.

## Run

```bash
cd demos/c2-omegaclaw/artifact1
./run-demo.sh

cd ../artifact2
python3 -m unittest -v
python3 audit_links.py vectors/e0-capture3.json --format text
```

Neither demo needs network access or credentials. Artifact 1's external directory is a
topology simulation, not proof of an independent institution or public-chain anchor; the
deployment gate does not close until a real party retains the receipt/root or a verified
external anchor exists. The Proposed S8 EAS adapter supplies the path but no transaction has
yet been sent. Artifact
2's E0 strings are captured data, while its `process_ref` and verified-Chronos positions are
test envelopes because E0 predates ACTA instrumentation. The individual README files state
the complete claim boundaries.
