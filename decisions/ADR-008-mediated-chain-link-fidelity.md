# ADR-008: Mediated-chain link fidelity

- Status: Accepted
- Date: 2026-08-31
- Decision authority: operator, under ADR-001
- Evidence: `research/E0-resultados.md`, capture 3

## Context

An OmegaClaw multi-hop inference destroys its AtomSpace after each call and relies on an LLM
to formulate the next call. Between a committed conclusion and the next premise, the harness
also applies deterministic transformations. Comparing the next premise directly with the raw
conclusion would therefore attribute declared harness behavior to the nondeterministic
mediator.

E0 capture 3 measured five isomorphic two-hop tasks under an explicit instruction to copy the
conclusion byte for byte. All five selected links were faithful and `T` happened to be the
identity on those short ASCII payloads. Capture 4 independently proves that `T` is not the
identity in general. Attempts t1 and t6 also recorded temporary omission and redundant
recomputation, demonstrating that a faithful-link ratio does not measure the mediator's
selection behavior.

## Decision

For a mediated inference chain, a premise `P` of hop `n+1` is a **faithful link** to a
committed conclusion `C` of an earlier hop if and only if all of the following hold:

1. `bytes(P) == bytes(T(C))`;
2. `C.process_ref == P.process_ref`; and
3. `C` precedes `P` in the verified Chronos sequence.

`C` is the canonical committed conclusion serialization governed by ADR-003 and committed at
the raw evaluation boundary fixed by ADR-005. `T` is the deterministic harness transformation,
including its order, source/version and relevant configuration, declared and fixed by the
ADR-004 lockfile. In the measured OmegaClaw harness it covers `normalize_string`,
`string-safe`, `last_chars(maxFeedback)` and the recorded transport framing. An undeclared or
unfixed transformation cannot establish fidelity.

A premise in the declared mediated-chain scope that does not satisfy all three conditions is a
**free link**: its formulation is producer assertion for the purposes of this mechanical
predicate. Semantic equivalence, paraphrase quality and intent are deliberately outside the
predicate.

`TR-CHAIN-MEDIATED` applies whenever the declared chain contains a nondeterministic mediator;
it is not removed merely because every observed link is faithful. Its report line MUST carry
the faithful/free measurement and the selection disclaimer together. The required meaning is:

> N of M premises match byte for byte, after the declared transformation T, conclusions
> previously committed in this process; selection of which conclusions are fed back belongs
> to the producer and this dossier does not constrain it.

The numerator, denominator, declared `T`, chain scope and selection disclaimer are one report
unit. A verifier must not present the ratio alone.

## Interpretation of the E0 vector

The observed 5/5 is only the empirical ceiling of five isomorphic tasks under an explicit
byte-copy instruction. It is not a general mediator rate, a spontaneous-behavior estimate or
a claim transferable to other prompts and tasks. Attempts t1 and t6 are retained as narrative
evidence of omission/recomputation and must accompany that number when the vector is used in
C2.

## Consequences

- Declared deterministic harness changes are discounted without allowing semantic judgment
  into the verifier.
- Process mixing and retroactive matches cannot inflate the faithful count.
- A perfect faithful-link count does not claim completeness, good selection or an unmediated
  chain.
- This predicate and its report composition belong to the inference Profile/verifier. ACTA
  Core remains unaware of Hyperon, LLMs and mediated chains.

## Compatibility

This ADR composes ADR-003 canonical pair equality, ADR-004 lockfile dependencies, ADR-005's
commitment point and ADR-006's Chronos ordering. It changes none of them and does not alter
Core types, Protocol v0 or canonical encoding.

## Ratification

Ratified explicitly by the operator on 2026-08-31 as E-2 within the consolidated
E-9 + E-1 + E-2 + E-6 package, against commit
`503aefbd0636ba3ef52b782c675b1a901e27466a`.
