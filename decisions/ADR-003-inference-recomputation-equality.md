# ADR-003: Inference Recomputation Equality

- Status: Accepted
- Date: 2026-08-31
- Evidence: `research/E0-resultados.md`, capture 1

## Context

Recomputing an inference can produce several conclusions, and each conclusion carries a truth
value. A verifier needs a mechanical equality predicate that does not adjudicate semantic
equivalence and does not hide engine-dependent differences.

E0 capture 1 ran the same OmegaClaw NAL inference three times, including once after a container
restart. Under SWI-Prolog 10.0.2 and the measured import closure, the serialized output and its
order were byte-identical. `unique-atom` compares the whole atom; the truth value is part of
that atom.

## Decision

For an inference profile, recomputation equality is equality of the **canonical serialization
of `(conclusion, truth_value)` pairs**. Where the result semantics designate a nondeterministic
set, the Profile MUST define and version a deterministic normalization before commitment and
then serialize the normalized sequence. The order compared here is that canonical order, never
incidental engine iteration order. Where order is semantically part of the result, it remains
evidence and MUST NOT be normalized away.

The predicate is valid only when the recomputation environment is fixed by the applicable
inference-ruleset lockfile, including engine and version. It must not degrade silently to:

- conclusion-only equality;
- set membership;
- unordered-set comparison without the declared E-7 canonical normalization;
- approximate numeric equality; or
- semantic equivalence decided by a model or operator.

If the required lockfile is absent or does not match, the verifier cannot assert recomputation
equality under this ADR. It reports the corresponding residual trust condition when that code
becomes implementable under the report-code calendar.

This predicate belongs to the inference Profile/verifier. ACTA Core remains unaware of
Hyperon, NAL, PLN or any particular inference engine.

## Consequences

- Truth-value changes remain visible even when the conclusion term is unchanged.
- Differences after the declared E-7 normalization remain evidence; incidental iteration order
  of a nondeterministic result set is not evidence.
- Changing engine, version, closure or canonical serialization invalidates the predicate's
  premise and requires a new lockfile/profile version.
- Capture 1's three identical runs become the initial reference vector. A second vector with
  one conclusion and two different truth values remains desirable empirical reinforcement,
  but is not represented as already observed.

## Ratification

Ratified explicitly by the operator on 2026-08-31 under ADR-001.
