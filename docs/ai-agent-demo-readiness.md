# ACTA AI Agents v0 Demo Readiness

## 1. Status

ACTA AI Agents v0 profile/demo closure candidate once tests pass.

## 2. Scope

This profile records significant actions executed by external AI agent systems.

ACTA does not define how agents work.
ACTA does not run agents.
ACTA does not judge whether the agent acted correctly.

## 3. Minimum Flow

run_started
-> instruction_received
-> context_committed
-> tool_call_executed
-> significant_action_executed

Optional reinforced flow:

run_started
-> instruction_received
-> human_authorization_recorded
-> context_committed
-> tool_call_executed
-> significant_action_executed

## 4. What Is Closed

- event set;
- payloads;
- payload validation;
- lifecycle validation;
- AI Agent -> Core mapping;
- positive tests;
- negative tests;
- e2e bundle verification;
- Verification Report usage;
- no AI Agent semantics in Core.

## 5. What ACTA Can Claim

- event was registered;
- event hash verifies;
- event belongs to run/process;
- receipt exists and verifies;
- epoch/root was checked;
- Merkle proof verifies or fails;
- bundle is internally consistent or not;
- commitments/references exist structurally.

## 6. What ACTA Does NOT Claim

- agent acted correctly;
- action was legal;
- action was just;
- action was optimal;
- policy was substantively fulfilled;
- external policy was correct;
- output was materially true;
- institution must sanction;
- agent is guilty;
- deployment was substantively legitimate.

## 7. Deferred Debt

- real LangChain/CrewAI/AutoGen/OpenAI Agents SDK integration;
- real complex agent;
- UI;
- product final;
- human-readable/regulator report;
- Cardano/Midnight/DID/ZK/storage;
- policy_commitment / NormativeRefV0;
- advanced human authorization;
- multi-agent orchestration;
- marketplace/disputes/governance;
- commercial demo.

## 8. Forbidden In Core

- AI Agent enums;
- AI Agent lifecycle;
- AI Agent payload semantics;
- tool execution logic;
- prompt semantics;
- authorization semantics;
- agent framework integration;
- legal judgment;
- sanctions;
- institutional enforcement.

## 9. Definition Of Done

- cargo fmt --all passes;
- cargo test --workspace passes;
- valid AI Agent significant action flow exists;
- lifecycle validation outside Core exists;
- AI Agent -> Core mapping exists;
- flow produces verifiable bundle;
- Verification Report explains local verification;
- positive and negative tests exist;
- Core remains free of AI Agent semantics.
