# ADR-006: Chronos continuity across epoch boundaries

- Status: Proposed — pending operator ratification
- Date: 2026-08-31
- Decision authority: operator, under ADR-001

## Context

`ChronosRefV0` carries both an `epoch_id` and an optional `prev_event_hash`, but ACTA v0 does
not state whether the first event placed in a new epoch starts a new hash chain or points to
the preceding event of the same process.

An epoch is a closure and publication unit for an ordered Merkle tree. A process can outlive
one epoch. Treating every epoch boundary as a new genesis would therefore make a batching
decision silently reset process continuity. The existing v0 object shape can represent a
continuous link without adding a field or changing canonical encoding.

This ADR resolves only the boundary rule. It does not define epoch duration, closure policy,
publication substrate, or institutional completeness requirements.

## Proposed decision

Chronos continuity is per process and does not reset at an epoch boundary.

For two consecutive events of the same process:

- if both events belong to the same epoch, the later event's `prev_event_hash` MUST equal the
  earlier event's hash;
- if the later event is the first event for that process in a new epoch, its
  `prev_event_hash` MUST still equal the hash of the last event of that process in the
  preceding epoch;
- `prev_event_hash = None` denotes process genesis, not epoch genesis.

`epoch_id` identifies the Merkle inclusion and closure unit. It does not define the lifetime
of the process and does not authorize a chain reset.

A verifier that receives a complete process chain MUST require `None` only on its first
process event and MUST verify every later link, including links whose adjacent events have
different `epoch_id` values. A verifier that receives only a suffix or one isolated epoch
cannot infer genesis from the first supplied item: it needs an explicitly supplied expected
predecessor or must report that boundary continuity was not verified.

Acceptance of this ADR requires corresponding normative wording in
`protocol/ACTA_Protocol_v0.md` and test vectors covering a process that spans two epochs.
No implementation or Protocol change is authorized while this ADR remains `Proposed`.

## Consequences

- Epoch rotation cannot hide a process-chain reset behind a valid new Merkle root.
- The existing `ChronosRefV0` representation and receipt signing payload remain unchanged.
- Whole-process verification remains possible across any number of epochs.
- Standalone epoch verification must distinguish Merkle inclusion from predecessor
  continuity; proving one does not imply the other.
- Recorders and epoch builders must retain the last event hash per open process when placing
  its next event into a later epoch.

## Alternatives considered

### Start a new genesis at every epoch

Rejected in this proposal because it makes epoch rotation erase direct process continuity
and weakens anti-omission exactly at the batching boundary.

### Link epochs only by previous epoch root

Not selected for v0. It would prove an ordering between closure units but would not identify
the preceding event of a process, and it would require a new protocol field and canonical
encoding rule.

## Sources

- `protocol/ACTA_Protocol_v0.md` §§2.7, 6 and 7.
- `roadmap/acta-estado-consolidado.md` §8.1.
- `roadmap/directiva-construccion-2026-08-31.md` §6.
- `research/imported-notes/analisis-filosofico-claude-codex.md` §2.4 and §7,
  recommendation 4.
