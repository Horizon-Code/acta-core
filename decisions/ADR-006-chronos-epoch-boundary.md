# ADR-006: Chronos continuity across epoch boundaries

- Status: Accepted
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

## Decision

Chronos continuity is per process and does not reset at an epoch boundary.

For two consecutive events of the same process:

- if both events belong to the same epoch, the later event's `prev_event_hash` MUST equal the
  earlier event's hash;
- if the later event is the first event for that process in a new epoch, its
  `prev_event_hash` MUST still equal the hash of the immediately preceding event of that
  process, even when one or more intervening epochs contained no event for that process;
- `prev_event_hash = None` declares process genesis, not epoch genesis. It does not by itself
  prove that no earlier event exists.

`epoch_id` identifies the Merkle inclusion and closure unit. It does not define the lifetime
of the process and does not authorize a chain reset.

A verifier MUST first establish that all supplied events belong to the same `process_id` and
preserve its `process_type`; Chronos linkage alone does not establish that precondition. When it
receives a complete process chain, it MUST require `None` only on the first supplied process
event and verify every later link, including links whose adjacent events have different
`epoch_id` values. When it receives only a suffix or one isolated epoch, it cannot infer genesis
from the first supplied item: it needs an explicitly supplied expected predecessor or must
report that boundary continuity was not verified.

Implementation of this ADR requires corresponding normative wording in
`protocol/ACTA_Protocol_v0.md` and test vectors covering a process that spans two epochs.
It also requires a suffix-verification API that accepts an expected predecessor; the current
`verify_event_chain_v0` API treats the first supplied event as genesis and is insufficient for
that case. Its start mode must distinguish `ProcessGenesis`, `ExpectedPredecessor(hash)` and
`PredecessorNotSupplied`; an `Option<hash>` would ambiguously conflate genesis with missing
evidence. Ratification fixes the decision but does not itself start implementation under the
current operational freeze.

## Consequences

- With a complete same-process chain or an expected predecessor, an epoch rotation cannot hide
  a chain reset behind a valid new Merkle root. Without either, global completeness is not
  claimed.
- The existing `ChronosRefV0` representation and receipt signing payload remain unchanged.
- Whole-process verification remains possible across any number of epochs.
- Standalone epoch verification must distinguish Merkle inclusion from predecessor
  continuity; proving one does not imply the other.
- Recorders and epoch builders must retain the last event hash for every process that may
  receive a later event; lifecycle and retention policy remain Profile/institution concerns.

Minimum acceptance vectors cover: adjacent events across two epochs; intervening epochs with no
event for the process; an improper `None` after a boundary; a suffix with an expected
predecessor; a suffix without one reported as unverified rather than genesis; and mixed
`process_id` input rejected by the composed process-plus-Chronos verification.

## Future report condition

`TR-CHRONOS-BOUNDARY-UNVERIFIED` belongs to the dossier-detected register when the declared
verification scope starts from a suffix or isolated epoch and neither the complete preceding
chain nor an expected predecessor is supplied. Its affirmative text is: “El tramo suministrado
es internamente consistente; verificar su continuidad con la historia anterior requiere la
cadena completa o un predecesor esperado.” It must land with the suffix-verification API, not
before. An expected predecessor that is supplied but does not match is a verification failure,
not this residual condition.

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

## Ratification

Ratified explicitly by the operator on 2026-08-31 under ADR-001.
