# OpenWorker → ACTA connector (S9)

Status: **implementation candidate**. The connector is thin by design: it translates OpenWorker
tool-call records into `ai_agent` Profile events and then into Core events. Nothing else. No
transport, no policy, no judgement.

Core never learns that OpenWorker exists. The crate is isolated from the repository workspace,
which also keeps the root `Cargo.toml` and `Cargo.lock` — both fixed by the ADR-012/013 hash
tables — untouched.

## What OpenWorker gives us

OpenWorker (`andrewyng/openworker`, MIT) records every tool call with its approval provenance —
auto-approved, user-approved or denied, with the reviewer's reasoning attached — and persists it
with the conversation. It supports MCP natively, so the connector plugs in without an upstream
change. No PR is proposed: their contribution policy discourages it.

## Mapping, and why it is ordered this way

ADR-009 orders the six `ai_agent` event kinds by forensic assertion strength. The approval
provenance maps onto that ordering, and the mapping refuses to overstate:

| OpenWorker | ACTA events | Why |
|---|---|---|
| `auto_approved` | `context_committed` + `tool_call_executed` | A reviewer model let it through. That is a machine assertion. Emitting `human_authorization_recorded` for it would claim a human acted when none did. |
| `user_approved` | `human_authorization_recorded` + `context_committed` + `tool_call_executed` + `significant_action_executed` | A human answered an escalation. In Profile terms that is a significant action with its authorization recorded first. |
| `denied` | none | See below. |

The reviewer's reasoning is committed as context in both approved paths — it is evidence either
way. Only its forensic strength differs, and that difference is carried by the presence or
absence of `human_authorization_recorded`, never by the reasoning itself.

## Two measured limits of Profile v1.0

Both are reported through `MappedRunV0::unrepresented`, each with a commitment over the original
record, so an omission is committed rather than silent. Neither is worked around, because every
available workaround asserts something the evidence does not support.

**1. A refusal cannot be expressed.** None of the six event kinds means refusal, and
`human_authorization_recorded` has no outcome field — recording a denial there would invert its
meaning and claim authorization where a human refused.

**2. Only one authorized escalation per run can be placed in temporal order.** The lifecycle
validator rejects any `human_authorization_recorded` that follows a `significant_action_executed`.
A run where a human approves twice cannot carry both authorizations in the order they happened.
Emitting both up front would misrepresent when they occurred — the exact distortion ACTA exists
to detect — so the later escalation keeps its tool call and reports the lost authorization.

Both limits are arguments for a Profile v1.1. Changing the Profile is an operator decision under
ADR-009's versioning rules and is not made here.

## Retention

`fixtures/conversation-security-review.json` is retained in-tree and pinned by SHA-256 in the
test suite, so the mapping is reproducible from the repository with no container and no external
state. This follows the rule that came out of losing the C2 evidence: a run intended as
demonstration material persists outside ephemeral storage in the same act that generates it.

## Tests

```bash
cargo test --manifest-path adapters/openworker-mcp/rust/Cargo.toml
```
