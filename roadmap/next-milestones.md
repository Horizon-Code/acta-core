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

**Status on 2026-08-31: gate open, 4/4.** All four captures have measured data and a written
derived decision. Capture 3 ran through a local WebSocket channel with a real Anthropic model:
the selected sample measured 5/5 faithful links, while auxiliary attempts showed temporary
omission and redundant recomputation by the mediator. Full record, exact strings, raw logs and
drivers are in `research/E0-resultados.md` and `research/e0-logs/`.

C2 is no longer blocked by E0, but has not started: its redesign and E-1/E-2 still require
ratification at `architecture/` / `decisions/` level. The lockfile shape, verifier equality
predicate and commitment point have proposed ADRs pending operator acceptance.

Proposals awaiting Rub's ratification:

- `decisions/ADR-003-inference-recomputation-equality.md`
- `decisions/ADR-004-inference-ruleset-lockfile.md`
- `decisions/ADR-005-inference-commitment-point.md`

**Gate rule: not a single line of C2 before E0 closes.**
**Validation rule: not a single line of the real Cardano adapter before the three validation conversations.**

### Resulting execution order

- **Completed in this session:** E0 4/4 · A1 · A3 · B1 offline CLI ·
  `TR-KEY-SELF-ASSERTED` · `TR-SIGNER-SELF` · `TR-NO-ANCHOR` ·
  `TR-ANCHOR-UNVERIFIED` · `TR-TIME-DECLARED`.
- **Now:** Rub ratifies the E0 ADR proposals · specify the lockfile and equality predicate ·
  §9 validation conversations (with someone who signs off on compliance in banking or
  healthcare).
- **After E0 ratification:** final C2 redesign (per captures 3 and 4), preceded by the Cognitive
  Forensics Profile.
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
| E-2 | `TR-CHAIN-MEDIATED` per-link annotation; faithful-link predicate | `architecture/` + verifier | Unblocked; capture 3 adopts declared `T` |
| E-3 | Report split into structural vs detected conditions | `architecture/` (report model) | — |
| E-4 | Import-closure lockfile as (path, hash) manifest | `profiles/` + ADR | **Capture 2** |
| E-5 | Level 2 witness by reproducibility, not by transport | `architecture/` | — |
| E-6 | C2 Artefact 2 redesign; C2 rises in priority | `roadmap/` + demo docs | Unblocked; pending ratification |
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
| `TR-CHAIN-MEDIATED` | Ratification of capture 3's declared-`T` predicate | Post-E0 |
| `TR-KEY-SELF-ASSERTED` | — | **Implemented with A3** |
| `TR-SIGNER-SELF` | — | **Implemented with A1** |
| `TR-NO-ANCHOR`, `TR-ANCHOR-UNVERIFIED` | — | **Implemented with B1** |
| `TR-TIME-DECLARED` | — | **Implemented with the B2 report slice** |
| Remaining base codes of §3 | Specified in `directiva-construccion-2026-08-31.md` §3; pending implementation code by code | With their own verification |

`grep -r "TR-" core/rust/acta-core/src` returns nothing: the implemented codes live in the
external verifier that can evaluate them.

The report **mechanism** is not blocked by any of this and is already in place (see below).

### Report structure: done; Core remains empty by design

- `VerificationConditionRegisterV0` splits the report into its two registers (E-3): what the
  version never guarantees, and what was detected in this dossier. The first register is a
  scale, not a disclaimer list — closing a phase deletes a line from it.
- Condition codes are **data, not enum variants**: adding one must never require touching
  `acta-core`. Codes carrying domain semantics belong to the Profile; the Core defines the
  mechanism and at most substrate codes. The invariant is protected by a test that does not
  compile against a closed enum.
- Core v0 records zero conditions. The attestation adapter composes the first two evaluable
  codes over that neutral mechanism without adding cryptography or identity semantics to Core.
