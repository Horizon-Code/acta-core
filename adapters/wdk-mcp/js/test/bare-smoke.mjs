// Runs on Bare (and on Node.js). No test runner, no built-in modules.
import { createSessionRecorder, createLineSplitter } from '../src/recorder.mjs'

function check (condition, message) {
  if (!condition) throw new Error('bare-smoke: ' + message)
}

const recorder = createSessionRecorder({ sessionId: 's', agentRef: 'a', deployedByRef: 'o', standingInstruction: 'x' })
const client = createLineSplitter(recorder.clientLine)
const server = createLineSplitter(recorder.serverLine)
client.push('{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}\n{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"send_token","arguments":{}}}\n')
server.push(Uint8Array.from('{"result":{},"jsonrpc":"2.0","id":1}\n{"result":{"content":[]},"jsonrpc":"2.0","id":2}\n', (c) => c.charCodeAt(0)))
const record = recorder.close()
check(record.initialize.response === '{"result":{},"jsonrpc":"2.0","id":1}', 'initialize answer kept')
check(record.calls.length === 1 && record.calls[0].tool === 'send_token', 'tool call kept')
check(record.calls[0].response !== null, 'answer matched from a byte chunk')
check(/^\d{4}-\d\d-\d\dT/.test(record.closed_at), 'ISO timestamp')
console.log('bare-smoke: ok on ' + (typeof Bare !== 'undefined' ? 'Bare ' + Bare.version : 'Node.js ' + process.version))
