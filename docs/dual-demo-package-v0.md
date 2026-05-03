# Dual Demo Package v0 — AML + AI Agents

## 1. Status

Current technical status:

- Core v0-alpha frozen corregido: FINAL.
- Verification Report Tecnico/Local: closed and corrected.
- AML Profile: real crate, outside Core.
- AI Agents Profile/Demo v0: closed with accepted debt.
- Core remains neutral, small, auditable and substrate-independent.
- This package is documentation only.
- This is not product, UI, backend, integration, marketing, or commercial launch.

## 2. Purpose of this package

This document consolidates what ACTA currently demonstrates through two technical demos:

1. AML / account freeze.
2. AI Agent / significant action.

It is intended to support Mesa Ejecutiva, Producto, and Negocio in evaluating:

- what is already proven technically;
- what is common between both verticals;
- what is profile-specific;
- what remains as technical, product, and commercial debt;
- what should come next.

## 3. What the AML demo demonstrates

AML demonstrates a compliance-significant process:

`process_opened` -> `risk_scored` -> `manual_review_completed` -> `account_frozen`

What this proves:

- AML semantics live outside Core.
- AML Profile owns AML events, payloads, and lifecycle.
- AML Profile maps domain events to generic ACTA Core Events.
- Core validates structure, hashes, receipts, epochs, Merkle proofs, and bundles.
- The AML freeze flow can produce a verifiable bundle.
- Verification Report can explain local technical verification.
- ACTA can reconstruct why an account freeze event was recorded structurally.
- ACTA does not decide whether the freeze was legally correct or justified.

Minimum AML event set:

- `aml.process_opened`
- `aml.risk_scored`
- `aml.manual_review_completed`
- `aml.account_frozen`

Preserved/accepted optional debt:

- `aml.transfer_flagged`
- `aml.account_released`
- `aml.process_closed`

## 4. What the AI Agents demo demonstrates

AI Agents v0 demonstrates a significant action executed by an external AI agent system.

Minimum flow:

`run_started` -> `instruction_received` -> `context_committed` -> `tool_call_executed` -> `significant_action_executed`

Optional reinforced flow:

`run_started` -> `instruction_received` -> `human_authorization_recorded` -> `context_committed` -> `tool_call_executed` -> `significant_action_executed`

What this proves:

- AI Agent semantics live outside Core.
- ACTA does not run agents.
- ACTA does not define agent frameworks.
- ACTA does not integrate LangChain/CrewAI/AutoGen/OpenAI Agents SDK.
- AI Agent Profile owns events, payloads, and lifecycle.
- AI Agent Profile maps domain events to generic ACTA Core Events.
- The significant action flow can produce a verifiable bundle.
- Verification Report can explain local technical verification.
- ACTA can reconstruct structurally what action was executed, under what references/commitments.
- ACTA does not decide whether the agent acted correctly, legally, optimally, or justly.

Minimum AI Agent event set:

- `ai_agent.run_started`
- `ai_agent.instruction_received`
- `ai_agent.context_committed`
- `ai_agent.tool_call_executed`
- `ai_agent.significant_action_executed`

Optional:

- `ai_agent.human_authorization_recorded`

## 5. Common pattern demonstrated by both demos

Shared architecture:

External System -> External Profile/Demo -> ACTA Core Event -> hash -> receipt -> epoch -> Merkle proof -> bundle -> Verification Report

```txt
External domain system
    ↓
Profile-specific event/payload/lifecycle
    ↓
Mapping to ACTA Core Event
    ↓
Validated canonical event hash
    ↓
Receipt
    ↓
Local epoch
    ↓
Merkle proof
    ↓
Bundle
    ↓
verify_bundle_v0
    ↓
Verification Report Tecnico/Local
```

Common technical conclusion:

- ACTA Core can support multiple domains without creating separate architectures.
- Domain meaning is profile-owned; verifiability infrastructure is Core-owned.
- The same verification pipeline works across verticals (AML and AI Agents) while Core stays domain-neutral.

## 6. What ACTA can claim now

Across both demos, ACTA can claim:

- an event was registered in a deterministic protocol shape;
- the event hash is reproducible and verifiable;
- the event belongs to a process/run identifier;
- a receipt exists and can be validated structurally;
- local epoch root and Merkle proof checks can pass or fail deterministically;
- a bundle can be internally consistent or inconsistent;
- a Verification Report can summarize local technical checks and failures.

## 7. What ACTA does not claim

Across both demos, ACTA does not claim:

- material truth of external evidence contents;
- legality, justice, or optimality of decisions/actions;
- substantive policy correctness or substantive policy compliance;
- institutional authority replacement;
- sanctions, guilt assignment, or enforcement.

ACTA decomposes trust; it does not eliminate trust.

## 8. What is still missing for real product

Not covered by this package:

- product-grade backend and operational workflows;
- end-user and operator UI;
- identity, permissioning, and secure key-management operations;
- observability, SLOs, incident response, and production hardening;
- integration with real enterprise systems and data governance controls;
- deployment and change-management procedures;
- regulator-facing and business-facing reporting layers.

## 9. What is still missing for serious commercial exposure

Still pending before serious external/commercial exposure:

- validated integrations with real customer environments;
- independent security review and deeper assurance posture;
- legal/compliance operating model around institutional accountability;
- commercial packaging, support model, and service boundaries;
- partner/channel readiness and implementation playbooks.

## 10. Next technical milestone recommendation

Recommended next milestone:

- harden a profile-agnostic "dual-demo verification pack" that runs both AML and AI Agent end-to-end verification flows in a single reproducible technical package, with standardized artifacts and pass/fail evidence outputs for technical review.

Milestone intent:

- preserve Core neutrality and freeze discipline;
- improve repeatability and audit handoff quality;
- prepare a bridge from technical demo validity to pre-product engineering readiness.

## 11. Governance of boundaries (explicit)

Boundary commitments maintained:

- Core remains small, neutral, auditable, and substrate-independent.
- Profiles own domain semantics.
- Core contains no AML or AI Agent semantics.
- No governance/tokenomics layer is introduced here.
- No Cardano/Midnight real anchoring, DID, ZK, or external storage dependency is introduced here.
