#!/usr/bin/env node
// Transparent stdio proxy: client <-> this <-> MCP server. Bytes are forwarded untouched in
// both directions; a copy of each line goes to the recorder. The session record is written
// when the session ends.
//
//   acta-mcp-proxy --config proxy.json --out session.json -- node path/to/wdk-mcp.mjs
import { spawn } from 'node:child_process'
import { readFileSync, writeFileSync, existsSync } from 'node:fs'
import { createSessionRecorder, createLineSplitter } from '../src/recorder.mjs'

const args = process.argv.slice(2)
const split = args.indexOf('--')
const option = (name) => { const i = args.indexOf(name); return i >= 0 && i < (split < 0 ? args.length : split) ? args[i + 1] : undefined }
const configPath = option('--config')
const outPath = option('--out')
const command = split < 0 ? [] : args.slice(split + 1)
if (!configPath || !outPath || command.length === 0) {
  process.stderr.write('usage: acta-mcp-proxy --config <proxy.json> --out <session.json> -- <server command...>\n')
  process.exit(2)
}
if (existsSync(outPath)) {
  process.stderr.write(`acta-mcp-proxy: ${outPath} already exists; refusing to overwrite a session record\n`)
  process.exit(2)
}

const config = JSON.parse(readFileSync(configPath, 'utf8'))
const recorder = createSessionRecorder({
  sessionId: config.session_id,
  agentRef: config.agent_ref,
  deployedByRef: config.deployed_by_ref,
  standingInstruction: config.standing_instruction,
  declared: {
    significantTools: config.significant_tools,
    previewArguments: config.preview_arguments,
    denialCodes: config.denial_codes
  }
})

const server = spawn(command[0], command.slice(1), { stdio: ['pipe', 'pipe', 'inherit'] })
const fromClient = createLineSplitter(recorder.clientLine)
const fromServer = createLineSplitter(recorder.serverLine)

process.stdin.on('data', (chunk) => { fromClient.push(chunk); server.stdin.write(chunk) })
process.stdin.on('end', () => server.stdin.end())
server.stdout.on('data', (chunk) => { fromServer.push(chunk); process.stdout.write(chunk) })

let written = false
function finish (code) {
  if (written) return
  written = true
  fromClient.flush()
  fromServer.flush()
  writeFileSync(outPath, JSON.stringify(recorder.close(), null, 2) + '\n', { flag: 'wx' })
  process.exitCode = code ?? 0
}
server.on('exit', (code) => finish(code))
server.on('error', (error) => { process.stderr.write(`acta-mcp-proxy: ${error.message}\n`); finish(1) })
for (const signal of ['SIGINT', 'SIGTERM']) process.on(signal, () => server.kill(signal))
