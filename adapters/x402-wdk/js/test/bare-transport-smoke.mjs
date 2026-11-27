// Runs on Bare (and on Node.js): two Hypercores, bundles across, a rewrite noticed.
import Hypercore from 'hypercore'
import { publishBundles, readBundles, compareWithCheckpoint } from '../src/transport.mjs'

const isBare = typeof Bare !== 'undefined'
const os = isBare ? await import('bare-os') : await import('node:os')
const base = os.tmpdir() + '/acta-bare-' + Date.now() + '-'
const encode = (text) => Uint8Array.from(text, (c) => c.charCodeAt(0))
const bundle = (n) => encode(JSON.stringify({ protocol: 'acta.v0', event: { process_ref: { process_id: 'x402:aa' }, n } }))

const seller = new Hypercore(base + 'seller')
await seller.ready()
const buyer = new Hypercore(base + 'buyer', seller.key)
await buyer.ready()
const a = seller.replicate(true)
const b = buyer.replicate(false)
a.pipe(b).pipe(a)

await publishBundles(seller, [bundle(0), bundle(1)])
const { bundles, checkpoint } = await readBundles(buyer, { transactionId: 'x402:aa' })
if (bundles.length !== 2) throw new Error('expected 2 bundles, got ' + bundles.length)
await seller.truncate(1)
await seller.append(bundle(9))
await buyer.update({ wait: true })
const findings = await compareWithCheckpoint(buyer, checkpoint)
if (findings.length !== 2) throw new Error('rewrite not noticed: ' + JSON.stringify(findings))
await seller.close()
await buyer.close()
console.log('bare-transport-smoke: ok on ' + (isBare ? 'Bare ' + Bare.version : 'Node.js ' + process.version))
