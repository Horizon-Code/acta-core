# ADR-005: Inference Commitment Point

- Status: Accepted
- Date: 2026-08-31
- Evidence: `research/E0-resultados.md`, capture 4

## Context

OmegaClaw transforms a skill result between evaluation and the next LLM turn. In the measured
large-result case, the original 108,889-character result became a 50,000-character tail,
starting in the middle of token `11668` and lacking the opening parenthesis. The path also
passes through `normalize_string` and `string-safe`.

Committing only the LLM-visible context would therefore commit a malformed, harness-dependent
fragment that cannot close against recomputation of the engine output.

## Decision

For inference-profile evidence, the commitment point is the **raw output value of `(eval $s)`
before `normalize_string`**.

“Raw output” means the canonical byte serialization of that returned value, defined and
versioned by the Profile. Logger quoting, escaping and line framing are transport encodings and
are not themselves the committed bytes.

The exact `LAST_SKILL_USE_RESULTS` block/context delivered to the LLM is recorded as a separate
artifact. It is not substituted for the committed engine output.

When a later premise is compared with an earlier conclusion, any deterministic bridge between
the two is the declared harness transformation `T` fixed by the lockfile. Undeclared rewriting
is producer assertion, not evidence.

Instrumentation of this point belongs to the adapter/Profile integration around the target
runtime. It does not add Hyperon concepts to ACTA Core.

## Consequences

- Re-execution compares like with like: engine output against committed engine output.
- Truncation and formatting remain auditable because the model-visible context is preserved,
  but they cannot corrupt the primary commitment silently.
- The source/version and relevant configuration of the harness become evidence dependencies.
- Large or malformed context artifacts remain reportable without pretending they were the
  engine conclusion.

## Ratification

Ratified explicitly by the operator on 2026-08-31 under ADR-001.
