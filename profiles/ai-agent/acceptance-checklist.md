# Cognitive Forensics Profile v1.0 — acceptance checklist

- Semantic-review result: **pass — request maintainer approval**
- Requested lifecycle status: **provisional**
- Review date: 2026-09-01
- Maintainer approval: **pending a specific operator decision on this completed submission**
- Substantive semantic review: S6 executor, against `ACTA Foundations v1.2` and
  `architecture/profile-architecture-v1.0.md`
- Ratified baseline: `503aefbd0636ba3ef52b782c675b1a901e27466a`

This record separates semantic review, repository acceptance and profile maturity. The
operator's order to start S6 authorized preparation; it could not approve the later text and
implementation hash. Semantic review now passes, but the registry decision remains Proposed
until the operator accepts the exact submission recorded by ADR-009. Even after acceptance,
the requested lifecycle remains `provisional`; it does not claim the independent consumer
required for `stable` status.

## Minimum submission package

| Requirement | Result | Evidence |
|---|---|---|
| Identifier, namespace, semantic version and maintainer | Pass | `README.md` header |
| Scope and explicit non-scope | Pass | `README.md` §1 |
| Event kinds and assertion strength | Pass | `README.md` §2; Rust `types.rs` |
| Lifecycle model | Pass | `README.md` §3; `validate_ai_agent_lifecycle_v0` |
| Material-effect model | Pass | `README.md` §2 |
| Policy reference model | Pass | `README.md` §5; ADR-004 |
| Commitment model | Pass | `README.md` §4; Rust mapper and validators |
| Core and Protocol compatibility | Pass | `README.md` §8 |
| Backward compatibility and migration expectations | Pass | `README.md` §8 |
| Normative example flow | Pass | `README.md` §9; Rust examples/tests; C2 artefacts |
| Markdown specification | Pass | `README.md` |
| Machine-readable validation equivalent | Pass | `rust/src/types.rs`, `rust/src/lib.rs`, tests |
| Example events or bundles | Pass | Rust examples/tests and C2 Artefact 1 bundles |
| Acceptance outcome | Pending | ADR-009 records the exact submission for maintainer review |

## Constitutional checklist

1. **Stable identity:** pass — `ai_agent`, version `1.0`, process type `ai_agent_run`.
2. **Scope/non-scope:** pass — bounded significant acts; no material truth, legality,
   fairness, completeness of thought or institutional judgment.
3. **Narrow event names:** pass — each kind names a specific declared, committed or executed
   act; generic `update`/`review` labels are absent.
4. **Stable lifecycle labels:** pass — each event has one stated meaning and predecessor rule.
5. **Close/reopen/revoke semantics:** pass — v1.0 deliberately defines no closing event and
   therefore makes no completeness claim; adding one is versioned.
6. **Material effects:** pass — the event table identifies `Yes`, `Potential` or `No`.
7. **Significant acts only:** pass — routine token, telemetry and internal-thought capture is
   outside scope.
8. **Minimization/no surveillance:** pass — payloads stay outside events and are represented
   by bounded commitments and references.
9. **Resolvable policy references:** pass — inference policy hashes bind the ADR-004 manifest;
   resolution and substantive validity remain separate claims.
10. **Real commitments:** pass — the specification identifies exact artifacts, formation
    points and disclosure/verification paths; placeholders are invalid.
11. **Ordering/contradiction rules:** pass — lifecycle validation and Chronos requirements are
    explicit, including the ADR-006 boundary condition.
12. **No Core judgment:** pass — mapping uses Core primitives without adding agent or Hyperon
    semantics to Core.
13. **Backward compatibility:** pass — readable minor evolution is separated from semantic
    changes requiring a major version.
14. **Migration notes:** pass — this is the first registered version; future semantic changes
    and additions have explicit version rules.
15. **Dispute-ready examples:** pass for Profile semantics, open for deployment custody —
    Artefact 1 exercises the external-witness contract in a local topology but does not supply
    an independent custodian; Artefact 2 preserves raw conclusion, exact LLM context, later
    premise, process/Chronos checks and the declared transformation.

## Known risks retained by `provisional`

- No independently implemented consumer has yet verified a real bundle.
- v1.0 has no closure event and cannot claim a complete agent run.
- Premise formulation/selection by a nondeterministic mediator remains producer assertion and
  emits the corresponding residual-trust conditions.
- Identity and attestor independence remain limited by inline self-asserted keys until an
  external binding exists.
- The S6 Artefact 1 directory separation is only a topology simulation; actual independent
  custody or an external anchor remains a demonstrator closure gate.

These limits are reportable conditions or maturity constraints, not exceptions to the
constitutional checks above.
