# Next Milestones

1. S9: OpenWorker emission connector and Artefact 1 over OpenWorker (E-9.6), keeping the
   external reference condition. Unblocked by the S8 closure of 2026-09-03.
2. Mainnet anchoring and a resolvable attester identity, the two limits demonstration custody
   does not cover.
3. SCITT mini-E0, which must now budget for choosing and configuring a registration policy:
   SCRAPI declares issuer authentication out of scope, so each Transparency Service imposes
   its own.
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
and both C2 artefacts on 2026-09-01. ADR-009 was Accepted on 2026-09-02; S6 is closed in
executable work and real external custody remains deferred to S8.

Ratified by Rub on 2026-08-31:

- `decisions/ADR-003-inference-recomputation-equality.md`
- `decisions/ADR-004-inference-ruleset-lockfile.md`
- `decisions/ADR-005-inference-commitment-point.md`
- `decisions/ADR-006-chronos-epoch-boundary.md`
- `decisions/ADR-007-residual-trust-codes-e1.md`
- `decisions/ADR-008-mediated-chain-link-fidelity.md`

Ratified by Rub on 2026-09-02:

- `decisions/ADR-009-cognitive-forensics-profile-v1.md` on its fixed eight-file submission,
  with `provisional` lifecycle status.
- `decisions/ADR-010-agent-commerce-profile-v1.md` on its fixed seven-file submission, with
  `experimental` lifecycle status.
- `decisions/ADR-011-cross-attestation-semantics.md` on its fixed cross-attestation submission.

**Gate rule: not a single line of C2 before E0 closes.**
**Profile rule:** ADR-009/010 are Accepted on their exact hash sets; ADR-011 is binding but
cannot retire `TR-SIGNER-SELF` without its detectable external-identity inputs.
**Anchor rule:** satisfied on 2026-09-03. ADR-012/013 were ratified on their complete hash
sets, the schema and the S6 root are on Base Sepolia and external verification passed the full
seven-check predicate. The rule now reads forward: a testnet anchor is demonstration custody,
never production custody, and mock output still never counts as either.

### Resulting execution order

- **Completed in this session:** E0 4/4 · A1 · A3 · B1 offline CLI ·
  `TR-KEY-SELF-ASSERTED` · `TR-SIGNER-SELF` · `TR-NO-ANCHOR` ·
  `TR-ANCHOR-UNVERIFIED` · `TR-TIME-DECLARED`.
- **Implemented / S6:** Cognitive Forensics Profile submission and both C2 OmegaClaw
  artefacts; local fixture, real-history copy and E0-vector checks pass.
- **Completed / S6:** ADR-009/Profile Accepted; S6 closed. The independent-custody gate closed
  on 2026-09-03 with the real Base Sepolia anchor, for the demonstration scope declared in
  `docs/agent-context/S6_C2_HANDOFF.md`.
- **Completed / S7:** exact `agent_commerce` submission and cross-attestation semantics
  Accepted under ADR-010/011; eight Profile tests pass. No report predicate was silently
  changed.
- **Prepared / S8 review:** EAS/Base adapter and machine-consumer report candidates under
  Proposed ADR-012/013. Mock, ABI fixtures, CLI composition and read-only Base deployment
  preflight pass; no external transaction has been sent.
- **A2 / real anchor:** EVM/Base-first, only after a real counterparty and Accepted adapter
  decisions. Until then, mock + `TR-ANCHOR-UNVERIFIED` remains honest product behavior.
- **Always in parallel:** documentary debt and the redefined E-9.8 conversations.

The Cardano/agents/EVM freeze is **lifted** by the Accepted package. Lifting the freeze is not
permission to skip session gates: details in E-9.1, E-9.2 and E-9.4 still require individual
ADRs to move from Proposed to Accepted before their implementation fixes normative choices.

### Active E-9 session sequence

| Session | Scope | Gate |
|---|---|---|
| **S6 — closed** | Accepted Cognitive Forensics Profile; C2 Artefact 1 (silent deletion against a retained witness) and Artefact 2 (ADR-008 link-by-link audit) on OmegaClaw | Custody gate closed on 2026-09-03 by the real Base Sepolia anchor, demonstration scope only. Source history retained in-tree, so the run is deterministically reproducible |
| **S7 — accepted** | Exact `agent_commerce` Profile and named cross-attestation forms in ADR-010/011 | Both ADRs Accepted on fixed hashes; external identity/report integration remains future work |
| **S8 — closed** | EVM/EAS `AnchorBackend` + Base Sepolia publisher/verifier and machine-consumer report; schema registered and S6 root anchored for real | ADR-012/013 ratified `experimental`; seven-check predicate passes over the third-party path. Demonstration custody only: testnet, and the attester identity is not established |
| **S9 — unblocked** | OpenWorker emission connector and public level-1 Artefact 1 | S8 gate/report dependencies resolved on 2026-09-03 |
| **S10** | x402/AP2 evidence-extension proposal | Working demo required |
| **Parallel [RUB]** | E-9.8 conversations, ratifications and community contacts | Non-delegable where specified |
| **Parallel [EJEC]** | Remaining documentary debt; freeze AML in its current internal-reference regime | No dedicated C1 session |

The market-research report cited by E-9 §0 was incorporated literally under `research/` on
2026-09-02. E-9 §0 points to its dated findings and separates them from strategic inference.

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
| `TR-SIGNER-SELF` | ADR-011 Accepted; external identity binding remains future E-9.5 work | **Implemented with A1 under its current predicate**. Retirement is permitted only when report integration proves cryptographic verification plus externally bound, distinct producer/counter-attestor subjects. Counter-signature with inline self-asserted keys does not remove it |
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
