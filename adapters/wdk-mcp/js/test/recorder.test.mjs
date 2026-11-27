import test from 'node:test'
import assert from 'node:assert/strict'
import { createSessionRecorder, createLineSplitter, SESSION_SCHEMA } from '../src/recorder.mjs'

const clock = () => { let t = Date.UTC(2026, 10, 27, 15, 0, 0); return () => (t += 1000) }
const base = { sessionId: 's1', agentRef: 'agent', deployedByRef: 'owner', standingInstruction: 'pay invoices under 1 USDT' }

test('keeps each tool call exactly as it crossed, matched by id', () => {
  const r = createSessionRecorder({ ...base, now: clock(), declared: { significantTools: ['send_token'], previewArguments: { send_token: { argument: 'dryRun', default: true } }, denialCodes: ['RULE_DENIED'] } })
  const init = '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}'
  const initOk = '{"result":{"serverInfo":{"name":"wdk-wallet"}},"jsonrpc":"2.0","id":1}'
  const callA = '{"jsonrpc":"2.0","id":"a","method":"tools/call","params":{"name":"send_token","arguments":{"to":"0x1",  "amount":"1"}}}'
  const callB = '{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"get_balance","arguments":{}}}'
  r.clientLine(init); r.serverLine(initOk)
  r.clientLine('{"jsonrpc":"2.0","method":"notifications/initialized"}')
  r.clientLine(callA); r.clientLine(callB)
  r.serverLine('{"result":{"content":[]},"jsonrpc":"2.0","id":7}')
  r.serverLine('{"jsonrpc":"2.0","method":"notifications/progress","params":{}}')
  r.serverLine('{"result":{"content":[]},"jsonrpc":"2.0","id":"a"}')
  const record = r.close()

  assert.equal(record.schema, SESSION_SCHEMA)
  assert.equal(record.initialize.request, init)
  assert.equal(record.initialize.response, initOk)
  assert.equal(record.tool_catalog, null)
  assert.deepEqual(record.calls.map((c) => [c.jsonrpc_id, c.tool, c.request === (c.tool === 'send_token' ? callA : callB), c.response !== null]), [
    ['"a"', 'send_token', true, true],
    ['7', 'get_balance', true, true]
  ])
  assert.equal(record.calls[0].request, callA, 'whitespace inside the line is preserved')
  assert.ok(record.calls[0].responded_at > record.calls[1].responded_at, 'answers are stamped when they arrive')
  assert.deepEqual(record.declared, { significant_tools: ['send_token'], preview_arguments: { send_token: { argument: 'dryRun', default: true } }, denial_codes: ['RULE_DENIED'] })
  assert.equal(record.opened_at, '2026-11-27T15:00:01.000Z')
  assert.ok(record.closed_at > record.opened_at)
})

test('an unanswered call stays unanswered and junk is counted, not repaired', () => {
  const r = createSessionRecorder(base)
  r.clientLine('{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"sign_message","arguments":{}}}')
  r.clientLine('not json')
  r.serverLine('{"result":{},"jsonrpc":"2.0","id":99}')
  r.serverLine('[1,2')
  const record = r.close()
  assert.equal(record.calls[0].response, null)
  assert.equal(record.calls[0].responded_at, null)
  assert.equal(record.unparsed_lines, 2)
})

test('the recorder refuses to run without declared parties', () => {
  assert.throws(() => createSessionRecorder({ ...base, agentRef: '' }), TypeError)
  assert.throws(() => createSessionRecorder({ ...base, standingInstruction: undefined }), TypeError)
})

test('the line splitter handles chunk boundaries and CRLF', () => {
  const lines = []
  const s = createLineSplitter((line) => lines.push(line))
  s.push('{"a":1}\n{"b"')
  s.push(Buffer.from(':2}\r\n\n{"c":3}'))
  s.flush()
  assert.deepEqual(lines, ['{"a":1}', '{"b":2}', '{"c":3}'])
})

test('a multi-byte character cut by a chunk boundary survives', () => {
  const lines = []
  const s = createLineSplitter((line) => lines.push(line))
  const bytes = Buffer.from('{"m":"aé€😀₮"}\n')
  for (const byte of bytes) s.push(Uint8Array.of(byte))
  assert.deepEqual(lines, ['{"m":"aé€😀₮"}'])
})
