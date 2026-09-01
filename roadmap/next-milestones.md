# Next Milestones

1. **Close S6:** obtain explicit acceptance of Proposed ADR-009/Profile and place an Artefact 1
   receipt/root in real independent custody (or an external anchor). Both C2 implementations
   and their local verification are complete.
2. Await S6 closure and an explicit order before S7; do not start `agent_commerce` or
   cross-attestation early.
3. Incorporate the 31-Aug market-research report when the operator supplies it; do not invent
   a replacement.
4. Define the profile registration/review workflow linked to ADRs.
5. Add CI checks to detect misplaced files by authority level.

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
the selected sample measured a 5/5 ceiling in isomorphic tasks under explicit byte-copy
instructions, not a general mediator rate. Auxiliary attempts showed temporary omission and
redundant recomputation. Full record, exact strings, raw logs and drivers are in
`research/E0-resultados.md` and `research/e0-logs/`.

C2 is no longer blocked by E0 or ratification. The operator ratified the consolidated
E-9 + E-1 + E-2 + E-6 package on 2026-08-31 against commit
`503aefbd0636ba3ef52b782c675b1a901e27466a`; E-1 and E-2 are
binding as ADR-007 and ADR-008. S6 then implemented the Cognitive Forensics Profile submission
and both C2 artefacts on 2026-09-01; profile approval and real external custody remain closure
gates.

Ratified by Rub on 2026-08-31:

- `decisions/ADR-003-inference-recomputation-equality.md`
- `decisions/ADR-004-inference-ruleset-lockfile.md`
- `decisions/ADR-005-inference-commitment-point.md`
- `decisions/ADR-006-chronos-epoch-boundary.md`
- `decisions/ADR-007-residual-trust-codes-e1.md`
- `decisions/ADR-008-mediated-chain-link-fidelity.md`

**Gate rule: not a single line of C2 before E0 closes.**
**Profile rule:** the S6 implementation is not an Accepted Profile or closed demonstrator until
Proposed ADR-009 receives specific operator approval.
**Validation rule: no real anchor adapter before a real counterparty validates the need and the applicable ADRs are Accepted.**

### Resulting execution order

- **Completed in this session:** E0 4/4 · A1 · A3 · B1 offline CLI ·
  `TR-KEY-SELF-ASSERTED` · `TR-SIGNER-SELF` · `TR-NO-ANCHOR` ·
  `TR-ANCHOR-UNVERIFIED` · `TR-TIME-DECLARED`.
- **Implemented / S6:** Cognitive Forensics Profile submission and both C2 OmegaClaw
  artefacts; local fixture, real-history copy and E0-vector checks pass.
- **Now / S6 closure:** accept ADR-009/Profile and arrange real independent custody or an
  external anchor for Artefact 1.
- **Next only after closure and explicit order / S7:** follow S7–S10 below, one gated session
  at a time. Do not begin S7 early.
- **A2 / real anchor:** EVM/Base-first, only after a real counterparty and Accepted adapter
  decisions. Until then, mock + `TR-ANCHOR-UNVERIFIED` remains honest product behavior.
- **Always in parallel:** documentary debt and the redefined E-9.8 conversations.

The Cardano/agents/EVM freeze is **lifted** by the Accepted package. Lifting the freeze is not
permission to skip session gates: details in E-9.1, E-9.2 and E-9.4 still require individual
ADRs to move from Proposed to Accepted before their implementation fixes normative choices.

### Active E-9 session sequence

| Session | Scope | Gate |
|---|---|---|
| **S6 — implementation complete, closure pending** | Proposed Cognitive Forensics Profile; C2 Artefact 1 (silent deletion with simulated external topology) and Artefact 2 (ADR-008 link-by-link audit) on OmegaClaw | Open: explicit ADR-009/Profile approval + real independent custody/anchor. Local real-history copy and E0-vector checks pass |
| **S7** | Specify `agent_commerce` and named cross-attestation forms as Proposed ADRs | Do not start before S6 closes |
| **S8** | EVM/EAS `AnchorBackend` mock → Base testnet plus machine-consumer report priority | Real counterparty validated; applicable ADRs Accepted |
| **S9** | OpenWorker emission connector and public level-1 Artefact 1 | S8 gate/report dependencies resolved |
| **S10** | x402/AP2 evidence-extension proposal | Working demo required |
| **Parallel [RUB]** | E-9.8 conversations, ratifications and community contacts | Non-delegable where specified |
| **Parallel [EJEC]** | Remaining documentary debt; freeze AML in its current internal-reference regime | No dedicated C1 session |

The market-research report cited by E-9 §0 is pending operator delivery. Once received, add it
under `research/` and cite it with dates while separating measured facts from strategic
inference. Do not synthesize a substitute.

### Amendments E-1..E-8: ratification status

The amendments originate in `E0-protocolo-y-enmiendas.md` (Parte 2) and are non-binding there
by themselves, per `decisions/ADR-001-repository-authority-levels.md`. Architecture or
verification choices become binding when materialized at the corresponding authority level;
roadmap sequencing such as E-6 can be accepted at roadmap level without redefining a higher
layer:

| Amendment | Subject | Target level | Blocked by |
|---|---|---|---|
| E-1 | Four new `TR-*` report codes on top of the nine in Directiva §3 | `decisions/` | **Accepted via ADR-007** |
| E-2 | `TR-CHAIN-MEDIATED` per-link annotation; faithful-link predicate | `decisions/` + verifier | **Accepted via ADR-008** |
| E-3 | Report split into structural vs detected conditions | `architecture/` (report model) | — |
| E-4 | Import-closure lockfile as (path, hash) manifest | `profiles/` + ADR | **Ratified via ADR-004** |
| E-5 | Level 2 witness by reproducibility, not by transport | `architecture/` | — |
| E-6 | C2 Artefact 2 redesign; C2 rises in priority | `roadmap/` + demo docs | **Accepted in consolidated E-9 package** |
| E-7 | Canonicalization rule written as a pair | `profiles/` + ADR | — |
| E-8 | Content provenance vs process provenance; EU dates | `research/` + compliance docs | — |

### Report code calendar

General rule, valid for every report condition code: **a code is implemented together with the
functionality that makes its condition detectable, never before.** A code whose condition the
verifier cannot evaluate is not a report line — it is a promise in the source.

| Code | Blocked by | Lands with |
|---|---|---|
| `TR-ENGINE-UNPINNED` | Detection defined by ADR-007; lockfile implementation conforming to ADR-004 | With inference lockfile verification |
| `TR-IMPORT-UNPINNED` | Detection defined by ADR-007; lockfile implementation conforming to ADR-004 | With inference lockfile verification |
| `TR-CHAIN-MEDIATED` | Predicate Accepted in ADR-008; Cognitive Forensics Profile/report composition | S6 Artefact 2 |
| `TR-KEY-SELF-ASSERTED` | — | **Implemented with A3** |
| `TR-SIGNER-SELF` | — | **Implemented with A1**; retirable only after a new Accepted predicate detects an independent attestor **and** identity is externally bound. Counter-signature with inline self-asserted keys does not remove it |
| `TR-NO-ANCHOR`, `TR-ANCHOR-UNVERIFIED` | — | **Implemented with B1** |
| `TR-TIME-DECLARED` | — | **Implemented with the B2 report slice** |
| `TR-CHRONOS-BOUNDARY-UNVERIFIED` | ADR-006 + declared chain scope and optional expected predecessor | Suffix/chain verification API + report composition (`DossierDetected`) |
| Remaining base codes of §3 | Specified in `directiva-construccion-2026-08-31.md` §3; pending implementation code by code | With their own verification |

`grep -r "TR-" core/rust/acta-core/src` returns nothing: the implemented codes live in the
external verifier that can evaluate them.

The future boundary condition is dossier-detected: the supplied evidence verifies continuity
from its first available event, while the preceding boundary remains unverified until the
verifier receives either the complete earlier chain or an expected predecessor.

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
