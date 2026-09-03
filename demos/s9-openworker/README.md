# S9 Artefact 1 — OpenWorker audit trail under ACTA

An OpenWorker audit trail is a file the operator controls. Editing it leaves no trace that the
operator's own tooling can find. This demonstrator seals the same trail through the thin
connector into signed Chronos/Merkle bundles, and shows what changes.

```bash
cargo run --manifest-path demos/s9-openworker/Cargo.toml
```

## What it shows

1. The trail seals into 12 lifecycle bundles over 5 tool calls, with an epoch root.
2. Deleting an **approved** tool call: OpenWorker's own check still passes — the file parses —
   while ACTA's external check fails on `epoch_root`, lifecycle count and tool-call count.
3. Deleting the **denied** force-push: also detected, but by a materially weaker mechanism, and
   the demonstrator says so out loud instead of taking the win.

## The condition that makes it probatory (E-9.6)

The comparison proves something to a third party **only** if the epoch root is retained outside
operator control. A root the operator holds is a root the operator can rewrite. The script must
state this explicitly; without it the demo claims more than it proves.

In S6 that condition was met by anchoring the C2 root on Base Sepolia, with the four precisions
in `docs/agent-context/S6_C2_HANDOFF.md`. S9 inherits the same requirement and the same
precisions — including that a testnet anchor is demonstration custody, never production custody.

## The hole this demo does not hide

Deleting the denied force-push does **not** break `epoch_root`, because Profile v1.0 cannot
express a refusal, so ACTA never committed it. The deletion is caught only by a plain tool-call
count the witness happens to carry. Remove that count and the denial becomes invisible.

A refusal is the single most incriminating record in an agent trail, and today it rests on the
weakest check ACTA offers. Say this in the demo. A demonstration that hides its own limit is
worth nothing, and this one is fixable — it is an argument for Profile v1.1.

## Note for the script, not for the code

When this goes public, someone will ask **"why not SCITT?"** — an IETF standard that does
tamper-evidence generically, with deployed implementations. The answer must be prepared, not
improvised:

A SCITT Transparency Service is an operator. Registration requires its permission, and the
mini-E0 measured that issuer acceptance is decided per-service, not by the standard: RFC 9943
puts issuer authentication explicitly out of scope, DataTrails requires an account with OAuth
client credentials, and `scitt-ccf-ledger` applies a JavaScript or Rego policy the administrator
writes. Even the accepted signature algorithms differ between services.

That is not a defect of SCITT — it is its model, and it buys interoperability with tools that
already exist. But it reintroduces someone to trust and someone to ask. ACTA is self-contained
and anchors against public substrate, where there is nobody to ask.

The honest framing is complementary, not competitive: neutrality, interoperability and legal
effect are three different properties, and the eventual demonstration is the same root anchored
three ways with the report saying what each one buys. Do not claim SCITT is worse. Claim it
answers a different question.
