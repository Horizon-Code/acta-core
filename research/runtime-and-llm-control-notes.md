# Runtime and LLM Control Notes

Research notes extracted from a mixed governance/runtime draft.

## Focus

- Runtime reliability when decision systems are non-deterministic.
- LLM interaction constraints and anti-hallucination controls.
- Execution observability and full version traceability.

## Key notes

- Working principle: "The LLM proposes, ACTA validates and executes."
- Proposed structured output contract includes decision, reasoning, confidence, policy reference, and status.
- Anti-hallucination state: `insufficient_information` to block unsafe act generation.
- Runtime guardrails should include schema validation, field validation, policy checks, and fallback logic.
- Observability targets include input/context/prompt/output plus model, prompt, policy, and TAP versioning.

## Status

Exploratory. This file is non-binding research and must not redefine architecture or decisions.
Binding adoption requires explicit rewrite under `architecture/` or `decisions/`.
