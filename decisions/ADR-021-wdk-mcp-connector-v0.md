# ADR-021: WDK wallet MCP connector v0

- Status: **Accepted**
- Date: 2026-11-27
- Accepted: 2026-11-27 by the operator under ADR-001
- Ratified submission commit: `4c6cd51ae99847cd305a2aa9c18abfa4fc01996c`
- Lifecycle status: `experimental`
- Directional prerequisite: Accepted ADR-015 (`ai_agent` Profile v1.1 vocabulary)
- Precedent: the OpenWorker connector (S9)
- Measured inputs: `adapters/wdk-mcp/fixtures/session-local-wallet.json`
- Implementation: in `adapters/wdk-mcp/` and `demos/s11-wdk-mcp/`, ratified with this ADR.
- Independent of ADR-019 and ADR-020 (x402 connector and evidence transport).

## Context

Tether's WDK exposes a self-custodial wallet to AI agents through an MCP server. Its
documentation says the preview-and-confirm sequence for transfers is not enforced by the
daemon, that there are no spending limits, and describes no record of tool calls. Run here, the
server executed a transfer that no earlier call had previewed.

OpenWorker (S9) gave ACTA a source that already recorded its tool calls. This source records
nothing, so the connector has to capture as well as map. That changes what it can honestly
claim, and this ADR fixes those limits.

## Decision

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

## Submission fixed for review

Fixed at commit `4c6cd51ae99847cd305a2aa9c18abfa4fc01996c`.

| File | SHA-256 |
|---|---|
| `adapters/wdk-mcp/README.md` | `516c0de299ddcc8c44d53fcfbf010f78101abcd35d336cdb720c564efbda9d0d` |
| `adapters/wdk-mcp/js/package.json` | `f994b292c4511b5a15c99c8b34bb898975cacc173e99546fcf4ad2419f5b7e2b` |
| `adapters/wdk-mcp/js/package-lock.json` | `c5d75e88c0ddbc46c6b8321656cce5be4c299a8bc48c0ccf4c26b289dc4036f3` |
| `adapters/wdk-mcp/js/src/recorder.mjs` | `7f65e79a8836cbb5753e7c1dd476916ceef56748af6a04262f62d2a40bcbe566` |
| `adapters/wdk-mcp/js/bin/acta-mcp-proxy.mjs` | `78ce3c9c7ad8a0d9dd9228600d9b69a42209a26532518a7ee6f1549a175a95ed` |
| `adapters/wdk-mcp/js/test/recorder.test.mjs` | `73c7759078439349ac3c274ba433af6ad2f694a3ab21023018c00ed36660b769` |
| `adapters/wdk-mcp/js/test/bare-smoke.mjs` | `2692b6147f9dd275766be3c5c26d3ba8fcc8c69623c5959eff8d952a0009bdde` |
| `adapters/wdk-mcp/js/test/local-session.mjs` | `e681c3d6c7ae33af45671b6c636e18ce3b0cd1fa57fdab1d2598d27ef233da2b` |
| `adapters/wdk-mcp/js/test/local-session.test.mjs` | `839af75d80d33d20ffb6ce767692507bce04038858d9a64316cf4dd82bf40cac` |
| `adapters/wdk-mcp/js/scripts/generate-fixtures.mjs` | `8c308ed31555448d6a252fa6393ee8c038d4bb297322979498fd0d4894dc70b0` |
| `adapters/wdk-mcp/fixtures/session-local-wallet.json` | `1625b3df7d651f66f1b5b8bc08fde833522c147bf73d444ac1a21a74517288df` |
| `adapters/wdk-mcp/rust/Cargo.toml` | `917ef9788700444b1b834fa764213173cd0a84812ca55dc394ce0e829d2faf6f` |
| `adapters/wdk-mcp/rust/Cargo.lock` | `a36a4c64b539c00350f374790be33040379f4527b3a99ad9f0d232a2747a9aab` |
| `adapters/wdk-mcp/rust/src/lib.rs` | `14650266b7e93c58e70a9d417786ccafea52e5ed6a944adf3bbb0681610e6fb3` |
| `adapters/wdk-mcp/rust/tests/mapping_v0.rs` | `1a61e42911ae6ca0b65b355374bc361b11cafb372aac62ffc5076df177318781` |
| `demos/s11-wdk-mcp/Cargo.toml` | `f2647840846bc8533c2915792fdea12167ead68c0aa11293fba62e286667f961` |
| `demos/s11-wdk-mcp/Cargo.lock` | `06affcabcea1a3c91a781273fc832fc015d1242c3083e99b85bf096fce0b5b90` |
| `demos/s11-wdk-mcp/src/main.rs` | `e4a2ab2fa0a63dcb7e49488469ba5d095f82bcd400ff0ff9f9455dbbae911275` |
| `demos/s11-wdk-mcp/README.md` | `6cf9c125b433cbf8cdee1eaec23cac93f3f4cbcaa745e6701a556357b8a61f6f` |

## Ratification

Ratified explicitly by the operator on 2026-11-27 under ADR-001 against commit
`4c6cd51ae99847cd305a2aa9c18abfa4fc01996c` and the 19 SHA-256 values above. The
ratification accepts the exact submission with `experimental` lifecycle status. A byte change
requires a refreshed table and review of the affected part before it remains ratified.

What the ratification does not cover: the `ai_agent` Profile v1.1 crate this connector maps
onto is implemented under ADR-017, which remains Proposed. The whole mapping carries that
status until ADR-017 is decided. Anchoring of the resulting bundles is not exercised.
