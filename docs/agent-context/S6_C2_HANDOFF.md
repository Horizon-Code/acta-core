# S6 C2 Handoff

**Completion date:** 2026-09-01
**Authority:** operator ratification of E-9 + E-1 + E-2 + E-6 on baseline commit
`503aefbd0636ba3ef52b782c675b1a901e27466a`

## Implemented in S6

- E-9 is Accepted and the old Cardano/agents/EVM freeze is lifted.
- E-1 and E-2 are binding as ADR-007 and ADR-008; E-6 is Accepted roadmap scope.
- The Cognitive Forensics Profile v1.0 is formalized under `profiles/ai-agent/README.md`, has
  passed semantic review and awaits specific operator approval through Proposed ADR-009.
- C2 Artefact 1 and Artefact 2 are implemented under `demos/c2-omegaclaw/`.

S6 is not declared closed: Artefact 1 still needs custody of a receipt/epoch root by a real
independent party (or an external anchor), and the Profile needs maintainer approval.

## Artefact 1

The demonstrator parses exact OmegaClaw `memory/history.metta` blocks without normalization,
maps each block to `ai_agent.context_committed`, verifies signed bundles offline and compares
the present history with a locally separated epoch witness that exercises the external-
retention contract.

Verified paths:

- deterministic three-record fixture: local check passes before and after deletion; ACTA
  detects the missing middle record;
- retained E0 `omegaclaw-memory` volume, mounted read-only: 100 records / 21,642 bytes sealed
  as 102 lifecycle events (two opening events plus 100 contexts), original verification
  passed, a copied `[E0-c1-dup]` record was deleted, local form remained valid with 99 records
  and ACTA detected sequence/membership/count change with exit code 2.

The real-history epoch root for that local run was
`bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba`.
The source/witness hashes and exact findings are recorded in
`demos/c2-omegaclaw/artifact1/e0-real-history-run.md`.

Limits: the demo signing seed is public, keys are inline, and the external-party directory is
a topology simulation. It proves the mechanics and the need to retain a commitment outside
operator control; it does not supply institutional independence or a public anchor and cannot
close that E-9.6 criterion.

## Artefact 2

The auditor validates the exact declared transform, full `ProcessRefV0` equality and Chronos
precedence, then emits faithful/free classification with its evidence strings.
`TR-CHAIN-MEDIATED` remains present even at 5/5; `TR-NONDET-INPUT` separately names
mediator-selected/formulated premises.

The E0 vector binds t2/t3/t4/t5/t7 strings and raw-log line provenance to
`research/e0-logs/c3-measurements.json`. Its 5/5 is emitted only as a setup-specific ceiling
under explicit copy instruction, with the selection disclaimer in the same report object.
t1/t6 remain narrative evidence outside the denominator.

The source patch `0001-log-raw-eval-before-normalization.patch` modifies only
`src/loop.metta` at the step-5 eval boundary. `prepare_loop_source.sh` reconstructs the source
from pinned commit `642c536`, verifies base/final hashes and prepares a read-only bind mount;
it does not modify the checkout or use the plugin API.

Limits: E0 supplies real raw/context/premise strings, but predates ACTA receipts and the
ADR-004 lockfile. The vector's `process_ref`, Chronos positions and minimum-closure hash
binding form a synthetic verification envelope, labelled as such in every classification.
The S6 auditor refuses an input that self-asserts a historical lockfile. A new live capture
must populate these fields from actual events, chain verification and the real ADR-004
verifier.

## Verification commands

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test -p acta-ai-agent-profile
PATH="$HOME/.cargo/bin:$PATH" cargo test --offline \
  --manifest-path demos/c2-omegaclaw/artifact1/Cargo.toml
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover \
  -s demos/c2-omegaclaw/artifact2 -p 'test_*.py' -v
```

## Still pending

- Specific operator acceptance of Proposed ADR-009 and its exact Profile submission.
- Real independent custody of the Artefact 1 receipt/root, or an external anchor. A sibling
  local directory is not sufficient.
- The operator-announced 31-Aug market-research report has not been supplied; do not
  reconstruct it.
- S7 is next but has **not started**. It requires its own session and Proposed ADRs for
  `agent_commerce` and named cross-attestation semantics.
- A real anchor adapter remains gated on a real counterparty and Accepted adapter decisions.
