// Regenerates the retained fixture from one live local session. The wallet, the transaction
// hashes and the timestamps differ on every run, so the output is NOT reproducible byte for
// byte: the retained file is the evidence, pinned by SHA-256 in the Rust tests.
import { writeFileSync } from 'node:fs'
import { runLocalSession } from '../test/local-session.mjs'

const { session } = await runLocalSession()
writeFileSync(new URL('../../fixtures/session-local-wallet.json', import.meta.url), JSON.stringify(session, null, 2) + '\n')
console.log(`wrote fixture with ${session.calls.length} tool calls`)
