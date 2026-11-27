# ADR-021: WDK wallet MCP connector v0

- Status: **Proposed**
- Date: 2026-11-27
- Decision authority requested: operator under ADR-001
- Directional prerequisite: Accepted ADR-015 (`ai_agent` Profile v1.1)
- Precedent: the OpenWorker connector (S9)
- Measured inputs: `adapters/wdk-mcp/fixtures/session-local-wallet.json`
- Implementation: **candidate present** in `adapters/wdk-mcp/` and `demos/s11-wdk-mcp/`. It
  does not ratify itself.
- Numbering: ADR-019 and ADR-020 are Proposed on another branch (x402 connector and evidence
  transport). This ADR does not depend on them.

## Context

Tether's WDK exposes a self-custodial wallet to AI agents through an MCP server. Its
documentation says the preview-and-confirm sequence for transfers is not enforced by the
daemon, that there are no spending limits, and describes no record of tool calls. Run here, the
server executed a transfer that no earlier call had previewed.

OpenWorker (S9) gave ACTA a source that already recorded its tool calls. This source records
nothing, so the connector has to capture as well as map. That changes what it can honestly
claim, and this ADR fixes those limits.

## Proposed decision

### 1. A transparent proxy captures; a mapper maps

The proxy forwards every byte between client and server unchanged and keeps each `tools/call`
request line and response line as transmitted. The record format is `acta.mcp.session.v0`.

### 2. The connector never decides

It does not approve, block, limit or rewrite. This is a boundary, not an omission: a recorder
that enforces is a control, and its record becomes evidence about itself.

### 3. What stands in for what the proxy cannot see

- **Instruction.** The proxy cannot see the AI client's prompts. `instruction_received` commits
  the deployer's standing instruction, supplied when the proxy starts. Its hash is the policy
  hash.
- **Context.** `context_committed` commits the server's `initialize` answer and tool catalogue.
- **Human authorization.** Never emitted. Approval happens in the AI client and does not cross
  the wire.

### 4. Classification is the deployer's declaration

Which tools are significant, which argument makes a tool preview, and which error codes mean a
control refused are declared in the proxy configuration and carried in the record. The mapper
applies the declaration; it does not infer.

### 5. Executed, failed, refused

- Any answered call is `tool_call_executed`, with the answer committed whatever it says.
- A declared-significant tool that executed is additionally `significant_action_executed`.
- An error with a declared refusal code is `action_denied`. Any other error is a failure —
  plumbing under ADR-015 §1 — and is never a denial.
- A call with no answer, or answered with a protocol error, is reported as unrepresented.

### 6. Findings are facts, not verdicts

`executed_without_preview` states that no earlier call in the session previewed the same
arguments. `no_control_observed` states that nothing in the session shows a control deciding.
Neither says the agent did wrong.

## Consequences

- ACTA gains a second agent source and its first capture component, without changing Core,
  Protocol or either Profile, and without touching any file fixed by a ratified hash table.
- The custody question becomes sharper than in S9: the record is produced on the agent's own
  machine. Until the epoch root is handed to someone else or anchored, the record is
  tamper-evident only against whoever kept an earlier root.
- A second runtime and pinned third-party packages enter the test surface. CI is not changed.

## Not decided here

- Any change to Core, Protocol, Profile vocabulary or the report.
- Where the epoch root goes. Anchoring uses the existing EAS adapter and is not exercised.
- Any enforcement component. If one is wanted it is a separate product decision, outside ACTA's
  frozen identity.
- MCP over HTTP.

## Fixed submission

The hash table is left for the ratification pass.

- `adapters/wdk-mcp/README.md`
- `adapters/wdk-mcp/js/package.json`
- `adapters/wdk-mcp/js/package-lock.json`
- `adapters/wdk-mcp/js/src/recorder.mjs`
- `adapters/wdk-mcp/js/bin/acta-mcp-proxy.mjs`
- `adapters/wdk-mcp/js/test/recorder.test.mjs`
- `adapters/wdk-mcp/js/test/bare-smoke.mjs`
- `adapters/wdk-mcp/js/test/local-session.mjs`
- `adapters/wdk-mcp/js/test/local-session.test.mjs`
- `adapters/wdk-mcp/js/scripts/generate-fixtures.mjs`
- `adapters/wdk-mcp/fixtures/session-local-wallet.json`
- `adapters/wdk-mcp/rust/Cargo.toml`
- `adapters/wdk-mcp/rust/Cargo.lock`
- `adapters/wdk-mcp/rust/src/lib.rs`
- `adapters/wdk-mcp/rust/tests/mapping_v0.rs`
- `demos/s11-wdk-mcp/Cargo.toml`
- `demos/s11-wdk-mcp/Cargo.lock`
- `demos/s11-wdk-mcp/src/main.rs`
- `demos/s11-wdk-mcp/README.md`
