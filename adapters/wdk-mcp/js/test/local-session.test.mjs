import test from 'node:test'
import assert from 'node:assert/strict'
import { runLocalSession, PROXY_CONFIG } from './local-session.mjs'

test('a real wallet session is recorded as it crossed the wire', async () => {
  const { results, session, address } = await runLocalSession()

  // What the real server did.
  assert.equal(results.tools.length, 16)
  assert.equal(results.preview.body.preview, true)
  assert.equal(results.previewed.body.success, true)
  assert.match(results.previewed.body.txHash, /^0x[0-9a-f]{64}$/)
  assert.equal(results.unpreviewed.body.success, true, 'the server executes a transfer nobody previewed')
  assert.equal(results.tooMuch.isError, true)
  assert.equal(results.tooMuch.body.code, 'INSUFFICIENT_FUNDS')
  assert.match(results.signed.body.signature, /^0x[0-9a-f]{130}$/)

  // What the proxy kept.
  assert.equal(session.session_id, PROXY_CONFIG.session_id)
  assert.equal(JSON.parse(session.initialize.response).result.serverInfo.name, 'wdk-wallet')
  assert.ok(session.tool_catalog.response.includes('send_token'))
  assert.deepEqual(session.calls.map((call) => call.tool), ['get_balance', 'send_token', 'send_token', 'send_token', 'send_token', 'sign_message'])
  assert.ok(session.calls.every((call) => call.response !== null))
  assert.ok(session.calls[2].response.includes(results.previewed.body.txHash.slice(2)), 'the recorded answer is the one the client got')
  assert.equal(session.unparsed_lines, 0)
  assert.ok(session.closed_at >= session.calls.at(-1).responded_at)
  assert.ok(!JSON.stringify(session).includes('seedPhrase'))
  assert.ok(JSON.stringify(session).includes(address.slice(2)))
})
