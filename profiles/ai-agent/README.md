# ACTA Cognitive Forensics Profile v1.0

- Registry decision: **Proposed — pending operator ratification of this submission**
- Requested profile lifecycle: **provisional**
- Maintainer approval: **pending**
- Semantic review: `acceptance-checklist.md`
- Ratified baseline: `503aefbd0636ba3ef52b782c675b1a901e27466a`
- Namespace: `ai_agent`
- Event-kind version: `1.0`
- Process type: `ai_agent_run`
- Maintainer: ACTA project

This specification formalizes the Cognitive Forensics Profile required before C2. It is
subordinate to the Foundations, the Protocol, the profile compatibility contract and
ADR-003 through ADR-006. The Rust implementation and its tests are the machine-readable
validation artifact for the currently implemented event set.

The implementation order that started S6 did not pre-ratify text written afterwards. This
submission is therefore ready for a specific maintainer decision but does not call itself
Accepted. If accepted, `provisional` maturity remains distinct from registry approval until
the independent-consumer condition in §8 is met.

## 1. Scope

The Profile records bounded, significant acts performed during an AI-agent run. It preserves
the deterministic phase of a computation and declares where a non-deterministic producer
formed or selected premises. It supports reconstruction and dispute; it does not establish
that an instruction, premise, conclusion or action was materially correct.

The Level-2 claim is deliberately narrow:

- one inference hop is recomputable from literal premises, the inference-ruleset lockfile,
  the pinned engine and the raw conclusion;
- a chain is a set of individually recomputable hops joined by links mediated by an LLM;
- each mediated link is classified mechanically as faithful or free;
- ACTA does not claim completeness of the hypotheses considered, nor explain “what the agent
  never thought”.

The Profile MUST NOT place Hyperon, PLN, model-provider or framework semantics in Core. It MUST
NOT treat semantic equivalence, fairness, legality or truth as a verification result.

## 2. Forensic assertion strength

Strength is the claim supported by the event, not necessarily lifecycle order. A later tier
does not make the underlying content true; it commits a stronger observation about what the
system did.

| Tier | Event kind | Assertion supported | Material effect |
|---|---|---|---|
| 1 — declared | `ai_agent.run_started` | A producer declared a bounded run and committed its agent version/environment references | No |
| 1 — declared | `ai_agent.instruction_received` | A producer declared receipt of the exact committed instruction under a policy hash | Potential |
| 2 — committed context | `ai_agent.context_committed` | The exact context/premise artifact and evidence references were committed before a later execution | Potential |
| 2 — independent reinforcement | `ai_agent.human_authorization_recorded` | A bounded authorization artifact and scope were recorded; independence remains conditional on verified identity | Yes |
| 3 — executed | `ai_agent.tool_call_executed` | A named tool invocation occurred with committed input and raw output | Potential |
| 4 — consequential | `ai_agent.significant_action_executed` | A material action was executed with committed input/output, decision reference and policy hash | Yes |

`human_authorization_recorded` is optional and MUST precede any significant action that relies
on it. Its mere presence does not prove that the authorizer is independent or institutionally
authorized.

## 3. Lifecycle

The implemented lifecycle is:

```text
run_started
  -> instruction_received
  -> [human_authorization_recorded]
  -> context_committed
  -> tool_call_executed
  -> significant_action_executed
```

All events MUST share one `run_id`, mapped to the Core `process_id`. `run_started` MUST occur
exactly once and first. A context requires a prior instruction; a tool call requires a prior
context; a significant action requires a prior tool call. If authorization is present, it
MUST occur after the instruction and before the significant action.

The current v1.0 lifecycle has no `run_closed` event. Therefore it makes no claim that the
recorded sequence is a complete run. Adding closure semantics is a versioned Profile change.

## 4. Commitment model

Payload bodies are not copied into ACTA events. Each Profile event commits the exact bounded
artifact named by its payload using the Protocol v0 `sha256:<lowercase-hex>` form. A producer
MUST retain or make disclosable the committed artifact and its canonicalization rule.

For an inference represented as `tool_call_executed`:

- `tool_input_commitment` commits the literal inference input/premises;
- `tool_output_commitment` commits the raw `(eval $s)` result at the ADR-005 commitment point;
- the exact LLM-visible `LAST_SKILL_USE_RESULTS` block is a separate context artifact;
- a `context_committed` event records the context/premises selected for the later hop.

The Profile MUST declare a premise origin. A premise formulated or selected by an LLM or other
non-deterministic process produces `TR-NONDET-INPUT`. This line states that premise reliability
is an assertion of the producer; it does not make the inference result unverifiable once the
literal premise has been committed.

## 5. Policy and recomputation

The Profile mapper MUST receive an explicit `PolicySnapshotV0` from its adapter and MUST NOT
invent a fallback policy hash. When a payload declares a policy hash, the mapper rejects a
different supplied snapshot. The snapshot bytes or manifest must exist and remain resolvable
under the commitment model in §4.

An inference represented as `tool_call_executed` uses `policy_type: inference_ruleset`; its
`policy_hash` MUST be the canonical manifest hash defined by ADR-004. The manifest fixes the
full executable closure, immutable provenance roots, engine/version and every harness setting
that affects the declared bridge `T`. Non-inference events use the governing Profile/domain
policy type supplied by their adapter; they do not inherit an `internal` placeholder.

Recomputation equality follows ADR-003: byte equality of the Profile's canonical serialization
of `(conclusion, truth_value)` pairs, after any versioned normalization for result sets whose
order is declared non-semantic. It is unavailable when the required lockfile is absent or does
not match.

## 6. Mediated-link predicate

For conclusion `C` from hop `n` and a carried premise `P` at hop `n+1`, the link is faithful
only when all of the following hold:

1. `bytes(P) == bytes(T(C))`;
2. both records have the same `process_ref`;
3. the event committing `C` precedes the event committing `P` in Chronos;
4. the manifest fixes the source/version/configuration of `T`.

For the OmegaClaw v0.1.19 reference vector, `T` is the declared code-order pipeline:

```text
normalize_string(raw, invalid_utf8 = ignore)
  -> string-safe("" -> _quote_, newline -> _newline_, ' -> _apostrophe_)
  -> last_chars(maxFeedback = 50000)
```

The notation ratified in E-2 is
`T = normalize_string ∘ string-safe ∘ last_chars(maxFeedback)`; the executable stage order
above is authoritative for this reference vector and MUST be encoded unambiguously in the
lockfile. A transformation absent from that declaration makes the link free.

A free link is a producer assertion, including a semantically equivalent paraphrase. The
verifier MUST NOT ask a model to decide equivalence.

Every aggregate line MUST keep the metric and selection disclaimer together:

> N de M premisas coinciden literalmente con `T` aplicada a conclusiones previamente
> comprometidas de este proceso; la selección de qué conclusiones se realimentan es del
> productor y este dossier no la acota.

`TR-CHAIN-MEDIATED` applies to every chain that crosses the mediator, regardless of its
faithful/free ratio. `TR-NONDET-INPUT` names the origin of a premise; the two conditions are
related but not interchangeable.

The 5/5 E0 sample is only a setup-specific empirical ceiling under explicit byte-copy
instruction. It MUST NOT be emitted as a general mediator rate. Auxiliary attempts t1 and t6
show omission and recomputation and MUST remain attached to that narrative limitation.

## 7. Chronos and epoch boundaries

All link verification is per process. Epoch rotation does not reset continuity. Complete-chain
verification or an expected predecessor is required across a supplied boundary as specified by
ADR-006. Without either, the composed report emits
`TR-CHRONOS-BOUNDARY-UNVERIFIED`; a supplied but wrong predecessor is a verification failure.

## 8. Compatibility and evolution

- Core and Protocol v0 remain unchanged.
- The current Rust implementation covers the six event kinds in §2 and enforces §3.
- New event kinds or changed meanings require a Profile version change.
- Existing v1.0 events remain readable under later minor versions; changing commitment meaning,
  lifecycle order or forensic assertion strength requires a major version.
- Framework adapters MUST remain thin mappings into this Profile and MUST disclose any event
  they cannot observe at the declared commitment point.

The Profile remains `provisional` until an independently implemented consumer verifies a real
bundle and the C2 reference vectors pass outside the producer environment.

## 9. Normative S6 flow

1. Commit the instruction and literal premises.
2. Execute the inference and commit the raw ADR-005 output.
3. Preserve the exact LLM-visible context separately.
4. Commit the later premise selected by the mediator.
5. Verify lockfile, same process and Chronos precedence.
6. Classify the link by `bytes(P) == bytes(T(C))`.
7. Emit the per-link record plus the inseparable aggregate disclaimer.
8. Retain the receipt/epoch root outside the operator before demonstrating silent deletion.

## 10. Conformance artifacts

- Implementation: `profiles/ai-agent/rust/`
- Lifecycle and mapping tests: `profiles/ai-agent/rust/tests/`
- E0 raw vectors: `research/e0-logs/`
- Empirical decisions: `research/E0-resultados.md` §§3–4
- C2 demonstrators: `demos/c2-omegaclaw/`
- Registry review: `profiles/ai-agent/acceptance-checklist.md`

The S6 submission checklist is: Profile tests pass; neither Core nor Protocol contains
AI-agent semantics; both C2 artifacts run without network credentials; deletion is checked
against the external-witness contract (with actual independent custody remaining a deployment
gate); link reports preserve raw/context/premise strings, the declared `T`, process/Chronos
checks, per-link classification and the selection disclaimer.
