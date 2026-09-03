# ADR-017: Profile v1.1 implementation submission

- Status: **Proposed**
- Date: 2026-09-03
- Decision authority requested: operator under ADR-001
- Implements: Accepted ADR-015 (`ai_agent` v1.1) and Accepted ADR-016 (`agent_commerce` v1.1)
- Governed by: principles 4.16, 4.17 and 4.18 of `architecture/profile-architecture-v1.0.md`

## Context

ADR-015 and ADR-016 ratified vocabulary and said an implementation would arrive as its own
submission with its own hash table. This is that submission. It does not reopen either
vocabulary.

## Shape of the submission, and why

**Both v1.1 profiles live in new crates rather than inside the v1.0 crates.** `types.rs`,
`lib.rs` and `Cargo.toml` of both v1.0 profiles are fixed by the ADR-009 and ADR-010 hash
tables, and adding a module would have required editing a pinned `lib.rs`. Editing them would
have broken two ratified tables to implement a version that explicitly adds rather than
reinterprets.

The separation is not only bookkeeping. A v1.0 dossier must keep validating under v1.0 rules
and keep telling the truth about itself, so v1.0's validator stays authoritative for v1.0
records and is consumed here, never modified. Both new crates are isolated from the workspace,
which also keeps the root `Cargo.toml` and `Cargo.lock` — fixed by ADR-012/013 — untouched.

Every entry of ADR-009, ADR-010, ADR-012 and ADR-013 was verified byte-identical after this
work.

## What is implemented

### `ai_agent` v1.1

- `action_denied` as its own event kind, carrying what was attempted, the control that refused
  it and its nature (human, reviewer model, policy), a commitment over the stated reasoning, and
  when. No verdict about whether refusing was correct.
- `session_opened` / `session_closed` as chained declared intervals.
- `CoverageSnapshotV1`: the shape of `PolicySnapshotV0`, its own struct, so no coverage hash is
  ever put into `policy_hash`.
- Agent identity carried on session open, with its self-assertion ceiling declared as a
  structural condition rather than hidden.
- The ADR-015 §5 ordering rule corrected: several authorized escalations per run, in the order
  they occurred, with pairing preserved — each authorization must be consumed by a later
  significant action, so an authorization that authorizes nothing is still invalid.

### `agent_commerce` v1.1

- `action_refused_against_mandate`, bound to the mandate manifest it was checked against.
- `dispute_resolved` with the six outcomes, the resolver and its nature, the norm resolved
  against, and the `dispute_opened` it closes by event hash.
- Mandate-declared milestones compared against the record, with `expires_at` distinguishing an
  open window from a closed one. No terminal event, no completeness claim.

### Residual-trust conditions

Structural conditions land now for both profiles, and retirement is per producing version and
never retroactive: a v1.0 dossier keeps reporting `TR-DENIAL-UNSUPPORTED`,
`TR-REFUSAL-UNSUPPORTED` and `TR-DISPUTE-RESOLUTION-UNSUPPORTED` forever, and an unrecognised
version is assumed to be the weaker one.

Three detected conditions also land, because the checks that make them detectable are in this
submission: `TR-DISPUTE-OPEN`, `TR-RESOLVER-SELF-INTERESTED` and `TR-MANDATE-COVERAGE-GAP`.

`TR-COVERAGE-GAP` is deliberately **not** implemented. Detecting it requires comparing declared
intervals against activity evidenced outside ACTA, which this submission does not have. Emitting
the code with no check behind it would be the same defect the whole v1.1 effort exists to
remove.

Surfacing any of these in the machine dossier requires `tools/acta-verifier`, which is fixed by
the ADR-013 table. That integration is therefore **not** in this submission and needs its own
table refresh.

## Demonstrated, not asserted

The S9 demonstrator runs on v1.1 and the ADR-015 closure criterion is met by execution:

- the connector no longer drops anything: `unrepresented` is empty for the retained fixture;
- the denied `git push --force origin main` is `action_denied` inside the Merkle tree;
- **deleting it now breaks `epoch_root`** rather than surviving on a tool-call count the witness
  happened to carry.

## Submission fixed for review

### `ai_agent` v1.1

| File | SHA-256 |
|---|---|
| `profiles/ai-agent/v1_1/rust/Cargo.toml` | `20e22995dd597fc5c9cd62498aff3a0a965776012a82f2e921950b9977aba3b9` |
| `profiles/ai-agent/v1_1/rust/Cargo.lock` | `bb30c3b7d0c1b9de99973b29c1218ef5b0891125f069e2516141df0e44a660c8` |
| `profiles/ai-agent/v1_1/rust/src/lib.rs` | `acd46c74709f2617c52849c35c6aa0bce49857a111b0240a7e956dd0e1629bfc` |
| `profiles/ai-agent/v1_1/rust/src/lifecycle.rs` | `46e150c2b14e99478d4c6aea2afb8e9fa3c6ce508c28fc404b7a4b9f7095ff04` |
| `profiles/ai-agent/v1_1/rust/src/conditions.rs` | `722570e24004d982fc910025f49cd5a24ab2655a1f4cde3257468c8e81ab8078` |
| `profiles/ai-agent/v1_1/rust/tests/profile_v1_1.rs` | `1e4e95484a60c56b4fad2b843252074465b2c7276e5837d70d29f582fed6642f` |

### `agent_commerce` v1.1

| File | SHA-256 |
|---|---|
| `profiles/agent-commerce/v1_1/rust/Cargo.toml` | `35ba4229a7ecb36c14525c1c80aaeb181a0177b32066418a0a4a503a869db125` |
| `profiles/agent-commerce/v1_1/rust/Cargo.lock` | `6f7e9ff9b38434978036079c93f85ad8d1c46ad75d9a000c2ceb372184970c71` |
| `profiles/agent-commerce/v1_1/rust/src/lib.rs` | `a518fc5767bc618e8637be998fab3d3afa96a97f82cd52641874937d3a90b137` |
| `profiles/agent-commerce/v1_1/rust/src/coverage.rs` | `502fcc5cf080dc8898770949996d1df0928eec11b0c7e07f7ab6cec5864ccd19` |
| `profiles/agent-commerce/v1_1/rust/src/conditions.rs` | `993bb46077d9a18c58e3e05947ba2edc4a8be3a0996aa063b8da53e10495e5d8` |
| `profiles/agent-commerce/v1_1/rust/examples/transaction_v1_1.rs` | `ebe977873d07da094e265b0c2f46a04cb6ec508d63df3f64c76f1d87a794fa12` |
| `profiles/agent-commerce/v1_1/rust/tests/profile_v1_1.rs` | `0810df4046862f409a7a7acd1936ea396addb423eba7be89d1665d398b2598d0` |

### Connector and demonstrator, migrated to v1.1

Not part of either profile, fixed here because the closure criterion is demonstrated with them.

| File | SHA-256 |
|---|---|
| `adapters/openworker-mcp/rust/Cargo.toml` | `77e624b3d3d888a173034723a09f901f25f7fa347df167779e8363f5b70787b9` |
| `adapters/openworker-mcp/rust/src/lib.rs` | `4e477a3d1b7e3f2a7e629daf02bffe7d5b1cee636e482e28d3dcc1ce4b44e86b` |
| `adapters/openworker-mcp/rust/tests/mapping_v1_1.rs` | `10e3c466c4151061af3eb0b24a2b8388299871eea2bea44814610eac435935db` |
| `demos/s9-openworker/src/main.rs` | `a59d90beb6ad1e80d23c8a810043e3d766f3b23d96fdbd79a638053e4fbd3e77` |

## Non-decisions

This ADR does not reopen ADR-015 or ADR-016, does not modify any v1.0 profile, does not change
Core or Protocol v0, does not integrate any condition into the machine dossier, and does not
implement `TR-COVERAGE-GAP`.

## Ratification requested

The operator may ratify this submission only against the complete fixed hash sets above. Until
then v1.1 is an implementation candidate: the code runs and its tests pass, but no producer is
told to emit v1.1 and no dossier claims it.
