import test from 'node:test'
import assert from 'node:assert/strict'
import { mkdtempSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import Hypercore from 'hypercore'
import { publishBundles, readBundles, checkpoint, compareWithCheckpoint, createEvidenceAdvertiser, EVIDENCE_HEADER } from '../src/transport.mjs'

const storage = () => mkdtempSync(join(tmpdir(), 'acta-transport-'))
const bundle = (processId, n) => new TextEncoder().encode(JSON.stringify({ protocol: 'acta.v0', event: { process_ref: { process_id: processId }, n } }, null, 2) + '\n')

async function pair (t) {
  const seller = new Hypercore(storage())
  await seller.ready()
  const buyer = new Hypercore(storage(), seller.key)
  await buyer.ready()
  const a = seller.replicate(true)
  const b = buyer.replicate(false)
  a.pipe(b).pipe(a)
  t.after(async () => { await seller.close(); await buyer.close() })
  return { seller, buyer }
}

test('bundles reach the buyer byte for byte, filtered by transaction', async (t) => {
  const { seller, buyer } = await pair(t)
  const mine = [bundle('x402:aa', 0), bundle('x402:aa', 1)]
  await publishBundles(seller, [mine[0], bundle('x402:bb', 0), new TextEncoder().encode('not json'), mine[1]])

  const { bundles, checkpoint: at } = await readBundles(buyer, { transactionId: 'x402:aa' })
  assert.deepEqual(bundles.map((b) => Buffer.from(b).toString()), mine.map((b) => Buffer.from(b).toString()))
  assert.equal(at.length, 4)
  assert.equal(at.fork, 0)
  assert.deepEqual(at, await checkpoint(seller), 'both peers hold the same checkpoint')
  assert.deepEqual(await compareWithCheckpoint(buyer, at), [])
})

test('the buyer cannot publish and the seller must publish bytes', async (t) => {
  const { seller, buyer } = await pair(t)
  await assert.rejects(publishBundles(buyer, [bundle('x402:aa', 0)]), /cannot write/)
  await assert.rejects(publishBundles(seller, ['{"a":1}']), TypeError)
  await assert.rejects(readBundles(buyer, {}), TypeError)
})

test('a seller rewriting its log is caught by the retained checkpoint, not by the log', async (t) => {
  const { seller, buyer } = await pair(t)
  await publishBundles(seller, [bundle('x402:aa', 0), bundle('x402:aa', 1)])
  const { checkpoint: retained } = await readBundles(buyer, { transactionId: 'x402:aa' })

  await seller.truncate(1)
  await seller.append(bundle('x402:aa', 'rewritten'))
  await buyer.update({ wait: true })

  // The log itself raises nothing: the buyer's copy now simply holds the new history.
  const after = await readBundles(buyer, { transactionId: 'x402:aa' })
  assert.match(Buffer.from(after.bundles[1]).toString(), /rewritten/)

  const findings = await compareWithCheckpoint(buyer, retained)
  assert.deepEqual(findings.map((f) => f.finding), ['history_rewritten', 'tree_hash_changed'])
})

test('appending more bundles is not a rewrite', async (t) => {
  const { seller, buyer } = await pair(t)
  await publishBundles(seller, [bundle('x402:aa', 0)])
  const { checkpoint: retained } = await readBundles(buyer, { transactionId: 'x402:aa' })
  await publishBundles(seller, [bundle('x402:cc', 0)])
  await buyer.update({ wait: true })
  assert.deepEqual(await compareWithCheckpoint(buyer, retained), [])
})

test('the advertiser marks paid responses only', () => {
  const key = 'ab'.repeat(32)
  const advertise = createEvidenceAdvertiser({ coreKey: key })
  const set = []
  const res = { setHeader: (name, value) => set.push([name, value]) }
  advertise({ headers: {} }, res, () => {})
  assert.deepEqual(set, [])
  advertise({ headers: { 'Payment-Signature': 'x' } }, res, () => {})
  assert.deepEqual(set, [[EVIDENCE_HEADER, key]])
  assert.throws(() => createEvidenceAdvertiser({ coreKey: 'short' }), TypeError)
})
