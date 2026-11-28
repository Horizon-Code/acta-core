// Settles one payment on a local chain and, while that chain is still running, has the Rust
// checker ask it over HTTP JSON-RPC. This is the only place the `http` client is exercised
// against a real node; it is still a local node.
import { execFile } from 'node:child_process'
import { mkdtempSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { runSettledExchange } from '../test/settled-exchange.mjs'

const manifest = fileURLToPath(new URL('../../rust/Cargo.toml', import.meta.url))
// Asynchronous on purpose: the chain lives in this process and must keep answering.
const check = (recordPath, rpcUrl) => new Promise((resolve) => {
  execFile('cargo', ['run', '--quiet', '--manifest-path', manifest, '--features', 'http', '--example', 'check', '--', recordPath, rpcUrl],
    { encoding: 'utf8' }, (error, stdout) => resolve({ exitCode: error ? error.code : 0, stdout }))
})

const run = await runSettledExchange({
  whileChainIsUp: async (settled, rpcUrl) => {
    const dir = mkdtempSync(join(tmpdir(), 'acta-s12-'))
    const good = join(dir, 'buyer-record.json')
    writeFileSync(good, JSON.stringify(settled.buyer))

    // The same record with its receipt pointing at a transaction nobody mined.
    const receipt = JSON.parse(Buffer.from(settled.buyer.payment_response_header, 'base64').toString())
    receipt.transaction = '0x' + 'ab'.repeat(32)
    const forged = join(dir, 'forged-record.json')
    writeFileSync(forged, JSON.stringify({ ...settled.buyer, payment_response_header: Buffer.from(JSON.stringify(receipt)).toString('base64') }))

    return { good: await check(good, rpcUrl), forged: await check(forged, rpcUrl) }
  }
})

const verdict = (result) => JSON.parse(result.stdout).verdict
console.log(`settled transaction ${run.receipt.transaction}`)
console.log(`live check of the real receipt:   ${verdict(run.live.good)} (exit ${run.live.good.exitCode})`)
console.log(`live check of a made-up receipt:  ${verdict(run.live.forged)} (exit ${run.live.forged.exitCode})`)
if (verdict(run.live.good) !== 'confirmed' || verdict(run.live.forged) !== 'not_confirmed') process.exit(1)
