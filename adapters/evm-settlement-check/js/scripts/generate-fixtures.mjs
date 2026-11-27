// Regenerates the retained fixture from one live local settlement. Addresses, nonce, hashes
// and times differ on every run, so the output is NOT reproducible byte for byte: the
// retained file is the evidence, pinned by SHA-256 in the Rust tests.
import { writeFileSync } from 'node:fs'
import { runSettledExchange } from '../test/settled-exchange.mjs'

const run = await runSettledExchange()
const fixture = {
  schema: 'acta.evm-settlement.fixture.v0',
  note: 'Local Ganache chain and a test token minted from nothing. Nothing of value moved.',
  buyer_record: run.buyer,
  seller_record: run.seller,
  rpc: run.rpc
}
writeFileSync(new URL('../../fixtures/settled-exchange.json', import.meta.url), JSON.stringify(fixture, null, 2) + '\n')
console.log(`wrote fixture for transaction ${run.receipt.transaction}`)
