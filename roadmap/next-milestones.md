# Next Milestones

1. Ratify architecture draft set in `architecture/`.
2. Define profile registration/review workflow linked to ADRs.
3. Replace placeholder profile markdown files with normative AML profile specifications.
4. Add CI checks to detect misplaced files by authority level.

## E0 gate

Source: `E0-protocolo-y-enmiendas.md` (Parte 1). Deliverable: `research/E0-resultados.md`.

E0 is a decision gate, not a pre-check. Its four captures decide, respectively: the verifier
equality predicate, the shape of the import lockfile, the nature of `TR-CHAIN-MEDIATED` in the
report, and whether committing results is viable at all.

| Block | Closing criterion |
|---|---|
| **E0** Experiment | All four captures are documented and each has its derived decision written down |

**Gate rule: not a single line of C2 before E0 closes.**
**Validation rule: not a single line of the real Cardano adapter before the three validation conversations.**

### Resulting execution order

- **Now, in parallel:** E0 (four captures) · A1 (signature verification) · A3 (inline keys in
  bundle) · §9 validation conversations (with someone who signs off on compliance in banking or
  healthcare).
- **After E0, with the four decisions written:** lockfile specification (E-4, shape per capture
  2) · verifier equality predicate (per capture 1) · final C2 redesign (per captures 3 and 4).
- **After the conversations:** A2 real Cardano adapter. Until then, mock +
  `TR-ANCHOR-UNVERIFIED` is the product working as designed.
- **Always in parallel:** documentary debt + amendments E-1..E-8.

### Amendments E-1..E-8: pending ratification

The amendments live in `E0-protocolo-y-enmiendas.md` (Parte 2) and are **non-binding while
they stay in `roadmap/`**, per `decisions/ADR-001-repository-authority-levels.md`. Ratifying
each one means rewriting it under `architecture/` or `decisions/`:

| Amendment | Subject | Target level | Blocked by |
|---|---|---|---|
| E-1 | Four new `TR-*` report codes on top of the nine in Directiva §3 | `architecture/` (report model) | Per-code, see below |
| E-2 | `TR-CHAIN-MEDIATED` per-link annotation; faithful-link predicate | `architecture/` + verifier | Capture 3 informs the framing, not the predicate |
| E-3 | Report split into structural vs detected conditions | `architecture/` (report model) | — |
| E-4 | Import-closure lockfile as (path, hash) manifest | `profiles/` + ADR | **Capture 2** |
| E-5 | Level 2 witness by reproducibility, not by transport | `architecture/` | — |
| E-6 | C2 Artefact 2 redesign; C2 rises in priority | `roadmap/` + demo docs | **Captures 3 and 4** |
| E-7 | Canonicalization rule written as a pair | `profiles/` + ADR | — |
| E-8 | Content provenance vs process provenance; EU dates | `research/` + compliance docs | — |

### Report code calendar

General rule, valid for all thirteen codes: **a code is implemented together with the
functionality that makes its condition detectable, never before.** A code whose condition the
verifier cannot evaluate is not a report line — it is a promise in the source.

| Code | Blocked by | Lands with |
|---|---|---|
| `TR-ENGINE-UNPINNED` | Lockfile shape (E0 capture 2) | Post-E0 |
| `TR-IMPORT-UNPINNED` | Lockfile shape (E0 capture 2) | Post-E0 |
| `TR-CHAIN-MEDIATED` | Scope predicate (E0 capture 3) | Post-E0 |
| `TR-KEY-SELF-ASSERTED` | Nothing in E0 — but its condition is only detectable once inline keys exist | **A3** |
| The nine base codes of §3 | Specified in `directiva-construccion-2026-08-31.md` §3; pending implementation code by code, same rule (e.g. `TR-ANCHOR-UNVERIFIED` not before the verifier has a checkable notion of anchor) | With their own verification |

Until then: `grep -r "TR-" core/` returns nothing outside documentation.

The report **mechanism** is not blocked by any of this and is already in place (see below).

### Report structure: done, empty on purpose

- `VerificationConditionRegisterV0` splits the report into its two registers (E-3): what the
  version never guarantees, and what was detected in this dossier. The first register is a
  scale, not a disclaimer list — closing a phase deletes a line from it.
- Condition codes are **data, not enum variants**: adding one must never require touching
  `acta-core`. Codes carrying domain semantics belong to the Profile; the Core defines the
  mechanism and at most substrate codes. The invariant is protected by a test that does not
  compile against a closed enum.
- Core v0 records zero conditions. The hole has the right shape and is empty.
