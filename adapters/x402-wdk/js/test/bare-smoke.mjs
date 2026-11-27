// Runs on Bare (and on Node.js). No test runner, no built-in modules: Bare has neither.
// It exercises both recorders with hand-made request and response objects.
import { createBuyerRecorder, createSellerRecorder, bytesToBase64 } from '../src/recorder.mjs'

function check (condition, message) {
  if (!condition) throw new Error('bare-smoke: ' + message)
}

const body = new Uint8Array([123, 34, 111, 107, 34, 58, 49, 125]) // {"ok":1}
const headers = (map) => ({ get: (name) => map[name.toLowerCase()] ?? null })
const fakeFetch = async (input, init) => {
  const paid = init && init.headers && init.headers['payment-signature']
  return paid
    ? { status: 200, headers: headers({ 'payment-response': 'cmVjZWlwdA==' }), clone: () => ({ arrayBuffer: async () => body.buffer }) }
    : { status: 402, headers: headers({ 'payment-required': 'cmVxdWlyZWQ=' }) }
}

const buyer = createBuyerRecorder({ recorderRef: 'buyer', counterpartyRef: 'seller' })
const fetch = buyer.wrap(fakeFetch)
await fetch('https://seller.example/x')
await fetch('https://seller.example/x', { headers: { 'payment-signature': 'c2lnbmVk' } })
check(buyer.records.length === 1, 'one buyer record')
check(buyer.records[0].payment_required_header === 'cmVxdWlyZWQ=', 'challenge kept')
check(buyer.records[0].response_body_b64 === 'eyJvayI6MX0=', 'body kept as base64')
check(/^\d{4}-\d\d-\d\dT/.test(buyer.records[0].response_seen_at), 'ISO timestamp')

const seller = createSellerRecorder({ recorderRef: 'seller', counterpartyRef: 'buyer' })
const res = { statusCode: 200, getHeaders: () => ({ 'payment-response': 'cmVjZWlwdA==' }), write () { return true }, end () { return this } }
seller.middleware({ method: 'GET', url: '/x', headers: { 'payment-signature': 'c2lnbmVk' } }, res, () => {})
res.write('{"ok"')
res.end(new Uint8Array([58, 49, 125]))
check(seller.records.length === 1, 'one seller record')
check(seller.records[0].response_body_b64 === buyer.records[0].response_body_b64, 'both sides hold the same body')
check(bytesToBase64(new Uint8Array([1, 2])) === 'AQI=', 'base64 padding')

console.log('bare-smoke: ok on ' + (typeof Bare !== 'undefined' ? 'Bare ' + Bare.version : 'Node.js ' + process.version))
