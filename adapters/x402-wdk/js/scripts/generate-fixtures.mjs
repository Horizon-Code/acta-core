// Regenerates the retained fixtures from one live local exchange. The signature nonce and
// validity window change on every run, so the output is NOT reproducible byte for byte: the
// retained files are the evidence, pinned by SHA-256 in the Rust tests. Re-running this
// script means updating those pins deliberately.
import { writeFileSync } from 'node:fs'
import { runLocalExchange } from '../test/local-exchange.mjs'

const { buyer, seller } = await runLocalExchange()
const out = new URL('../../fixtures/', import.meta.url)
writeFileSync(new URL('exchange-weather.buyer.json', out), JSON.stringify(buyer[0], null, 2) + '\n')
writeFileSync(new URL('exchange-weather.seller.json', out), JSON.stringify(seller[0], null, 2) + '\n')
console.log('wrote buyer and seller fixtures')
