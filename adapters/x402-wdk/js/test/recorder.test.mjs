import test from 'node:test'
import assert from 'node:assert/strict'
import { createBuyerRecorder, createSellerRecorder, bytesToBase64, EXCHANGE_SCHEMA } from '../src/recorder.mjs'

const clock = () => {
  let t = Date.UTC(2026, 10, 27, 12, 0, 0)
  return () => (t += 1000)
}

function fakeSeller ({ settle = true } = {}) {
  return async (input, init) => {
    const headers = new Headers((init && init.headers) || (input.headers ?? {}))
    if (!headers.has('payment-signature')) {
      return new Response('{}', { status: 402, headers: { 'PAYMENT-REQUIRED': 'cmVxdWlyZWQ=' } })
    }
    return new Response('{"weather":"sunny"}', {
      status: 200,
      headers: settle ? { 'PAYMENT-RESPONSE': 'cmVjZWlwdA==' } : {}
    })
  }
}

test('base64 matches the platform encoder for every tail length', () => {
  for (const text of ['', 'a', 'ab', 'abc', 'abcd', '{"weather":"sunny","temperature":70}']) {
    const bytes = new TextEncoder().encode(text)
    assert.equal(bytesToBase64(bytes), Buffer.from(bytes).toString('base64'))
  }
})

test('buyer recorder keeps the exchange exactly as transmitted', async () => {
  const recorder = createBuyerRecorder({ recorderRef: 'buyer', counterpartyRef: 'seller', now: clock() })
  const fetch = recorder.wrap(fakeSeller())
  const url = 'https://seller.example/weather'

  assert.equal((await fetch(url)).status, 402)
  assert.equal(recorder.records.length, 0, 'an unpaid challenge is not an exchange')

  const paid = await fetch(new Request(url, { headers: { 'PAYMENT-SIGNATURE': 'c2lnbmVk' } }))
  assert.equal(await paid.text(), '{"weather":"sunny"}', 'the caller still reads the body')

  assert.deepEqual(recorder.records, [{
    schema: EXCHANGE_SCHEMA,
    side: 'buyer',
    recorder_ref: 'buyer',
    counterparty_ref: 'seller',
    request: { method: 'GET', url },
    payment_required_header: 'cmVxdWlyZWQ=',
    payment_signature_header: 'c2lnbmVk',
    response_status: 200,
    payment_response_header: 'cmVjZWlwdA==',
    response_body_b64: Buffer.from('{"weather":"sunny"}').toString('base64'),
    signature_seen_at: '2026-11-27T12:00:01.000Z',
    response_seen_at: '2026-11-27T12:00:02.000Z'
  }])
})

test('buyer recorder reads the signature from init headers too', async () => {
  const recorder = createBuyerRecorder({ recorderRef: 'buyer', counterpartyRef: 'seller' })
  const fetch = recorder.wrap(fakeSeller({ settle: false }))
  await fetch('https://seller.example/weather', { headers: { 'Payment-Signature': 'c2lnbmVk' } })
  assert.equal(recorder.records[0].payment_signature_header, 'c2lnbmVk')
  assert.equal(recorder.records[0].payment_response_header, null, 'a missing receipt stays missing')
  assert.equal(recorder.records[0].payment_required_header, null, 'no challenge was seen, none is invented')
})

test('seller recorder captures what was received and what was sent', () => {
  const recorder = createSellerRecorder({ recorderRef: 'seller', counterpartyRef: () => 'buyer', now: clock() })
  const sent = []
  const res = {
    statusCode: 200,
    getHeaders: () => ({ 'payment-response': 'cmVjZWlwdA==' }),
    write (chunk) { sent.push(chunk); return true },
    end (chunk) { if (chunk) sent.push(chunk); return this }
  }
  let nexted = 0
  recorder.middleware({ method: 'get', originalUrl: '/weather', headers: { 'payment-signature': 'c2lnbmVk' } }, res, () => nexted++)
  res.write('{"weather":')
  res.end(Buffer.from('"sunny"}'))

  assert.equal(nexted, 1)
  assert.equal(sent.length, 2, 'the response still goes out')
  assert.deepEqual(recorder.records, [{
    schema: EXCHANGE_SCHEMA,
    side: 'seller',
    recorder_ref: 'seller',
    counterparty_ref: 'buyer',
    request: { method: 'GET', url: '/weather' },
    payment_required_header: null,
    payment_signature_header: 'c2lnbmVk',
    response_status: 200,
    payment_response_header: 'cmVjZWlwdA==',
    response_body_b64: Buffer.from('{"weather":"sunny"}').toString('base64'),
    signature_seen_at: '2026-11-27T12:00:01.000Z',
    response_seen_at: '2026-11-27T12:00:02.000Z'
  }])
})

test('seller recorder encodes string chunks as UTF-8 without the platform encoder', () => {
  const recorder = createSellerRecorder({ recorderRef: 'seller', counterpartyRef: 'buyer' })
  const res = { statusCode: 200, getHeaders: () => ({}), write () { return true }, end () { return this } }
  recorder.middleware({ method: 'GET', url: '/x', headers: { 'payment-signature': 'x' } }, res, () => {})
  const text = 'aé€😀 ₮'
  res.end(text)
  assert.equal(recorder.records[0].response_body_b64, Buffer.from(text, 'utf8').toString('base64'))
})

test('seller recorder ignores unpaid requests', () => {
  const recorder = createSellerRecorder({ recorderRef: 'seller', counterpartyRef: 'buyer' })
  const res = { end () {} }
  const end = res.end
  recorder.middleware({ method: 'GET', url: '/weather', headers: {} }, res, () => {})
  assert.equal(res.end, end, 'the response is left untouched')
  assert.equal(recorder.records.length, 0)
})

test('recorders refuse to run without declared parties', () => {
  assert.throws(() => createBuyerRecorder({ recorderRef: '', counterpartyRef: 'seller' }), TypeError)
  assert.throws(() => createSellerRecorder({ recorderRef: 'seller' }), TypeError)
})
