import test from 'node:test'
import assert from 'node:assert/strict'
import { runLocalExchange, STAND_IN_TRANSACTION } from './local-exchange.mjs'

test('both sides of a real x402 + WDK exchange record the same bytes', async () => {
  const { status, body, buyer, seller } = await runLocalExchange()
  assert.equal(status, 200)
  assert.equal(buyer.length, 1)
  assert.equal(seller.length, 1)
  const [b] = buyer
  const [s] = seller

  // The three artifacts both parties hold are byte-identical without any coordination.
  assert.equal(b.payment_signature_header, s.payment_signature_header)
  assert.equal(b.payment_response_header, s.payment_response_header)
  assert.equal(b.response_body_b64, s.response_body_b64)
  assert.equal(Buffer.from(b.response_body_b64, 'base64').toString(), body)

  // Only the buyer holds the challenge; the seller does not re-send it on the paid request.
  assert.ok(b.payment_required_header)
  assert.equal(s.payment_required_header, null)

  // The signed payload is a real EIP-3009 authorization produced by the WDK wallet.
  const payload = JSON.parse(Buffer.from(b.payment_signature_header, 'base64').toString())
  assert.equal(payload.x402Version, 2)
  assert.equal(payload.payload.authorization.value, '1000')
  assert.match(payload.payload.signature, /^0x[0-9a-f]{130}$/)

  // The receipt is the stand-in facilitator's: it proves the header path, not a settlement.
  const receipt = JSON.parse(Buffer.from(b.payment_response_header, 'base64').toString())
  assert.equal(receipt.transaction, STAND_IN_TRANSACTION)
})
