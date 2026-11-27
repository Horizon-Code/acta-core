# S11 demonstrator: a tamper-evident record of what an agent did with a wallet

Status: **demonstration material** over the implementation candidate in `adapters/wdk-mcp/`.

```bash
cd demos/s11-wdk-mcp && cargo run
cargo run -- seal <session.json> <out-dir>     # seal any recorded session into bundle files
```

One real session against Tether's `wdk-mcp` is sealed into signed Chronos/Merkle bundles: six
tool calls, three significant actions (two transfers and a signature), and a finding that one
transfer executed without a preview. Then the unpreviewed transfer is deleted from the record
and the epoch root no longer matches.

## What it does not show

Presenting this demo without these statements makes it a false claim.

- **Nothing of value moved.** The chain is local and the funds are made up.
- **The record does not protect itself.** It was sealed on the same machine that produced it,
  with a public demo key, and no root was anchored or handed to anyone. The edit is caught only
  because the demo kept the original root.
- **Nothing was prevented.** The proxy records; the unpreviewed transfer executed.
- **No human approval is shown or denied.** It is not observable at this layer.
