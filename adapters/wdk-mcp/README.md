# WDK wallet MCP → ACTA connector (S11)

Status: **implementation candidate** under Proposed ADR-021. Nothing here is binding until the
operator ratifies it.

Tether's Wallet Development Kit ships an MCP server, `wdk-mcp`, that lets an AI agent operate a
self-custodial wallet: read balances, send, swap, bridge, sign. Its own documentation states
what it leaves open:

- previewing a transfer before executing it is guidance for the AI client; the daemon does not
  enforce it;
- `sign_message` signs immediately;
- there are no per-transaction or cumulative spending limits;
- no record of tool calls is described.

This connector supplies the record, and only the record. Core never learns that MCP or WDK
exist. The crates are isolated from the repository workspace.

## Two pieces

| Piece | Where | What it does |
|---|---|---|
| Recorder and proxy | `js/src/recorder.mjs`, `js/bin/acta-mcp-proxy.mjs` | Sits between the AI client and the server on stdio. Forwards every byte untouched and keeps each `tools/call` request and response line as transmitted. |
| Mapper | `rust/` | Turns a recorded session into `ai_agent` Profile v1.1 events and Core events. |

```bash
acta-mcp-proxy --config proxy.json --out session.json -- wdk-mcp
```

`proxy.json` is the deployer's declaration: who the agent is, who deployed it, the standing
instruction it operates under, which tools count as significant, which argument makes a tool
preview, and which error codes mean a control refused.

## Mapping

| What the session shows | ACTA event |
|---|---|
| The proxy starts | `session_opened`, `run_started` |
| The deployer's standing instruction | `instruction_received` — its hash is the policy hash of every event |
| The server's identity and tool catalogue | `context_committed` |
| Any answered tool call, whatever the answer | `tool_call_executed` |
| A declared-significant tool that executed (not a preview, not an error) | additionally `significant_action_executed`, whose `decision_ref` points at the matching preview or says `none-recorded` |
| An error carrying a declared refusal code | `action_denied` |
| The proxy stops | `session_closed` |

Commitments are over the request and response lines as transmitted. An answer is read for two
facts only: whether the server flagged an error, and whether that error carries a declared
refusal code.

Findings are facts about the record, not verdicts: `executed_without_preview`,
`no_control_observed`, `unparsed_lines`.

## Measured limits

Found by running a real session against `wdk-mcp` 1.0.0-beta.9, not reasoned.

1. **The proxy does not stop anything.** In the retained session a transfer nobody previewed
   executed, and the record says so. A recorder that also decided would be a policy engine
   whose record is evidence about itself.
2. **The record is only as safe as where its root goes.** The proxy runs beside the agent.
   Whoever controls that machine can rebuild the session and seal it again. Protection starts
   when the epoch root leaves the machine — a counterparty's copy or an anchor. Nothing here is
   anchored.
3. **Human approval is invisible.** Approval dialogs live in the AI client and never cross the
   MCP wire, so `human_authorization_recorded` is never emitted. Its absence means "not
   observable here", not "nobody approved".
4. **The instruction is the standing one.** The proxy cannot see the prompts typed into the AI
   client. `instruction_received` commits what the deployer declared, and nothing per task.
5. **No refusal was observed live.** WDK's transaction policies are an SDK feature marked
   experimental; `wdk-mcp` produced no policy refusal in any session run here. The
   `action_denied` path is tested with a synthetic answer carrying WDK's documented code.
6. **Significance is declared, not detected.** Which tools matter and how they preview comes
   from `proxy.json`. A wrong declaration yields a wrong classification, and the declaration is
   in the record for anyone to read.
7. **Only stdio.** MCP over HTTP is not handled.
8. **Identities are self-declared.** `TR-SIGNER-SELF` and `TR-KEY-SELF-ASSERTED` stay in force.

## Retention

`fixtures/session-local-wallet.json` is one live session: the real `wdk-mcp` and its daemon, a
freshly generated throwaway wallet, and transfers that executed on an in-process local chain
with made-up funds. No public network was touched and nothing of value moved. The seed phrase
was never read by the driver and is not in the record. Pinned by SHA-256 in the Rust tests.

## Tests

```bash
cd adapters/wdk-mcp/rust && cargo test     # mapper: 10 tests over the retained fixture

cd adapters/wdk-mcp/js
npm test                      # recorder, no dependencies needed
npm ci                        # pinned third-party packages for everything below
npm run test:bare             # recorder on the Bare runtime
npm run test:local-session    # real wdk-mcp session through the proxy, on a local chain
```

Versions exercised: `@tetherto/wdk-cli` 1.0.0-beta.9, `ganache` 7.9.2, Node.js 22, Bare 1.39.0.
