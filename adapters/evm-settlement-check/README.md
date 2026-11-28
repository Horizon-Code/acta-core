# EVM settlement check (S12)

Status: **implementation candidate** under Proposed ADR-022. Nothing here is binding until the
operator ratifies it.

The x402 connector (ADR-019) records that the buyer *saw a receipt* and reads nothing: it
commits to bytes. This adapter is where reading happens. It opens the signed authorization and
the receipt, asks a node of the chain the receipt names, and reports whether the transaction
that receipt points at is the transfer the buyer authorized.

It lives apart from the connector so the connector's rule — bytes, not meaning — stays intact,
and so that everything this check trusts is visible in one place.

## What it checks

Ten named checks, each reported pass or fail with its detail:

| Check | Source |
|---|---|
| `scheme_is_exact` | signed payload |
| `authorization_matches_accepted_terms` | signed payload: the signed transfer is the accepted amount to the accepted payee |
| `receipt_reports_success_on_the_accepted_network` | receipt header |
| `receipt_payer_is_the_authorizer` | receipt header against signed payload |
| `node_is_on_the_named_chain` | `eth_chainId` |
| `transaction_is_mined` | `eth_getTransactionReceipt` |
| `transaction_succeeded` | receipt status |
| `transaction_called_the_accepted_asset` | receipt `to` |
| `asset_logged_the_transfer` | `Transfer(from, to, value)` emitted by the asset |
| `asset_logged_this_authorization` | EIP-3009 `AuthorizationUsed(authorizer, nonce)` emitted by the asset |

The last one is what ties the transaction to *this* payment rather than to another payment of
the same amount between the same parties.

The verdict is `confirmed` only if all ten pass. The node's two answers are committed as
received, so the check can be re-run later against the same bytes with no network.

## What a confirmed verdict does not mean

Returned with every result, never folded into the verdict:

- `SINGLE-NODE` — one node was asked; the verdict is what that node answered.
- `FINALITY-NOT-CHECKED` — block depth and finality were not examined.
- `ASSET-IDENTITY-NOT-CHECKED` — the asset is whatever contract the buyer accepted; whether it
  is the token it claims to be is not examined.

## Chains

No chain is built in. The chain profile is derived from the CAIP-2 network the payment itself
names (`eip155:<id>`), and the node has to confirm the chain id. The check needs only two
standard JSON-RPC methods, so it applies to any EVM chain a node can be reached for.

## Measured limits

1. **Only a local chain has been exercised.** The retained fixture is a payment that really
   settled — a WDK wallet signing, the x402 facilitator code submitting, a transfer mined — on
   an in-process Ganache chain with a test token minted from nothing. No public network,
   test or main, has been asked.
2. **The token is a test contract**, `contracts/TestToken.sol`, written for this purpose and
   not audited. A real stablecoin contract may emit additional logs or use the `bytes`
   signature overload; the check tolerates extra logs but has not met a real one.
3. **The result is a sidecar report, not an ACTA event.** `agent_commerce` has no event for "a
   third party checked the settlement", and none is invented here.
4. **Only the `exact` scheme with EIP-3009.** Permit2 and other x402 schemes are not handled
   and fail `scheme_is_exact` or `asset_logged_this_authorization`.
5. **Amounts above 2^128** are treated as not matching.

## Tests

```bash
cd adapters/evm-settlement-check/rust
cargo test                                             # 11 tests over the retained fixture
cargo run --example check -- ../fixtures/settled-exchange.json

cd ../js && npm ci
npm run test:settled-exchange    # a payment that really settles on a local chain
npm run live-check               # the Rust checker asking that running node over HTTP
npm run compile                  # rebuilds contracts/TestToken.json byte for byte
```

Versions exercised: `@x402/*` 2.26.0, `@tetherto/wdk-wallet-evm` 1.0.0-beta.27, `viem` 2.58.8,
`ganache` 7.9.2, `solc` 0.8.28, Node.js 22.
