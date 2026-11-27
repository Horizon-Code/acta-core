// End to end, on one machine, with every piece that exists today:
//
//   real x402 + WDK exchange  ->  seller seals its events  ->  bundles cross a Hypercore
//   ->  buyer checks them, seals its observations  ->  a third party verifies both
//   ->  the seller rewrites its log and the buyer's checkpoint notices
//
// Not exercised, and said so in the output: settlement (stand-in facilitator), peer discovery
// over a network (the two Hypercores are piped in-process), identity and anchoring.

import { execFileSync } from 'node:child_process'
import { mkdtempSync, mkdirSync, writeFileSync, readdirSync, readFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import Hypercore from 'hypercore'
import { runLocalExchange } from '../test/local-exchange.mjs'
import { publishBundles, readBundles, compareWithCheckpoint, createEvidenceAdvertiser, EVIDENCE_HEADER } from '../src/transport.mjs'

const demo = fileURLToPath(new URL('../../../../demos/s10-x402-wdk/', import.meta.url))
const acta = (...args) => execFileSync('cargo', ['run', '--quiet', '--manifest-path', join(demo, 'Cargo.toml'), '--', ...args], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim()
const field = (output, name) => output.split('\n').find((line) => line.startsWith(name + '='))?.slice(name.length + 1)
const step = (n, text) => console.log(`\n[${n}] ${text}`)

const work = mkdtempSync(join(tmpdir(), 'acta-e2e-'))
const sellerCore = new Hypercore(join(work, 'seller-core'))
await sellerCore.ready()

step(1, 'A buyer agent pays a seller agent over x402, signing with a WDK wallet')
const exchange = await runLocalExchange({ sellerMiddleware: [createEvidenceAdvertiser({ coreKey: sellerCore.key })] })
const advertised = exchange.headers.get(EVIDENCE_HEADER)
writeFileSync(join(work, 'seller-record.json'), JSON.stringify(exchange.seller[0]))
writeFileSync(join(work, 'buyer-record.json'), JSON.stringify(exchange.buyer[0]))
console.log(`    HTTP ${exchange.status}, body ${exchange.body}`)
console.log(`    seller advertised its evidence log: ${advertised.slice(0, 16)}…`)

step(2, 'The seller seals what it did into signed bundles')
const sold = acta('seal-seller', join(work, 'seller-record.json'), join(work, 'seller-bundles'))
const transactionId = field(sold, 'transaction')
console.log(`    ${field(sold, 'events')} events, epoch root ${field(sold, 'epoch_root').slice(0, 16)}…`)

step(3, 'The bundles cross to the buyer over a Hypercore')
const files = readdirSync(join(work, 'seller-bundles')).sort()
await publishBundles(sellerCore, files.map((name) => new Uint8Array(readFileSync(join(work, 'seller-bundles', name)))))
const buyerCore = new Hypercore(join(work, 'buyer-core'), Buffer.from(advertised, 'hex'))
await buyerCore.ready()
const a = sellerCore.replicate(true)
const b = buyerCore.replicate(false)
a.pipe(b).pipe(a)
const received = await readBundles(buyerCore, { transactionId })
mkdirSync(join(work, 'received'))
received.bundles.forEach((bytes, index) => writeFileSync(join(work, 'received', String(index).padStart(4, '0') + '.json'), bytes))
console.log(`    ${received.bundles.length} bundles received for ${transactionId.slice(0, 21)}…`)

step(4, 'The buyer checks them against what it signed and received, and seals its own side')
const bought = acta('seal-buyer', join(work, 'buyer-record.json'), join(work, 'received'), join(work, 'buyer-bundles'))
console.log(`    ${field(bought, 'events')} events, findings: ${bought.includes('finding=') ? 'yes' : 'none'}`)

step(5, 'A third party verifies both sides holding only the bundles')
console.log('    ' + acta('verify', join(work, 'received'), join(work, 'buyer-bundles')).split('\n')[0])

step(6, 'Later, the seller rewrites its evidence log')
await sellerCore.truncate(2)
await sellerCore.append(Buffer.from('{"bundle":"rewritten"}'))
await buyerCore.update({ wait: true })
for (const finding of await compareWithCheckpoint(buyerCore, received.checkpoint)) {
  console.log('    finding=' + JSON.stringify(finding))
}
console.log('    the bundles the buyer kept still verify: ' + acta('verify', join(work, 'received'), join(work, 'buyer-bundles')).split('\n')[0])

await sellerCore.close()
await buyerCore.close()
console.log(`
not exercised here:
- settlement: the facilitator is a stand-in and the transaction hash is made up
- peer discovery: the two logs are connected in-process, not over a network
- identity: both signing keys are public demo seeds
- anchoring: no epoch root was published to any chain
artifacts: ${work}`)
