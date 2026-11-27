// One real MCP session against Tether's wallet server, through the ACTA proxy.
//
// Real: `wdk-mcp` 1.0.0-beta.9, its daemon, a freshly generated wallet, and transfers that
// execute. Local: the chain is an in-process Ganache with made-up funds, so nothing here has
// any value and no public network is touched. The wallet is created in a temporary home and
// thrown away; its seed phrase is never read by this script.

import { spawn, execFileSync } from 'node:child_process'
import { mkdtempSync, writeFileSync, readFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import ganache from 'ganache'

const here = (path) => fileURLToPath(new URL(path, import.meta.url))
const WDK = here('../node_modules/@tetherto/wdk-cli/bin/wdk.mjs')
const WDK_MCP = here('../node_modules/@tetherto/wdk-cli/bin/wdk-mcp.mjs')
const PROXY = here('../bin/acta-mcp-proxy.mjs')
export const RECIPIENT = '0x000000000000000000000000000000000000dEaD'
export const PROXY_CONFIG = {
  session_id: 's11-local-session-0001',
  agent_ref: 'ref:agent:s11-demo-wallet-agent',
  deployed_by_ref: 'ref:operator:s11-demo',
  standing_instruction: 'Operate the demo wallet on the local network. Preview every transfer before executing it.',
  significant_tools: ['send_token', 'swap_token', 'bridge_token', 'sign_message', 'call_method'],
  preview_arguments: {
    send_token: { argument: 'dryRun', default: true },
    swap_token: { argument: 'dryRun', default: true },
    bridge_token: { argument: 'dryRun', default: true }
  },
  denial_codes: ['RULE_DENIED', 'NO_APPLICABLE_RULE', 'GOVERNED_BUT_UNMATCHED']
}

export async function runLocalSession () {
  const work = mkdtempSync(join(tmpdir(), 'acta-s11-'))
  const env = { ...process.env, HOME: join(work, 'home'), XDG_CONFIG_HOME: join(work, 'home', '.config'), WDK_PASSPHRASE: 'throwaway-demo-passphrase' }
  const wdk = (...args) => JSON.parse(execFileSync('node', [WDK, ...args, '--json'], { env, encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).trim().split('\n').pop())

  const chain = ganache.server({ chain: { chainId: 1337 }, logging: { quiet: true } })
  await chain.listen(0, '127.0.0.1')
  const rpcUrl = `http://127.0.0.1:${chain.address().port}`

  let proxy
  try {
    // The create command prints the seed phrase; its output is deliberately discarded.
    execFileSync('node', [WDK, 'wallet', 'create', '--name', 'agent', '--json'], { env, stdio: 'ignore' })
    wdk('wallet', 'unlock', '--name', 'agent', '--ttl', '10')
    wdk('network', 'create', JSON.stringify({ network: 'localdev', displayName: 'Local dev', module: '@tetherto/wdk-wallet-evm', testnet: true, config: { chainId: 1337, provider: rpcUrl } }))
    wdk('token', 'add', JSON.stringify({ network: 'localdev', token: 'eth', symbol: 'ETH', decimals: 18, isNative: true }))
    const { address } = wdk('get', 'address', '--network', 'localdev')
    await chain.provider.request({ method: 'evm_setAccountBalance', params: [address, '0x8AC7230489E80000'] }) // 10 ETH of nothing

    const configPath = join(work, 'proxy.json')
    const outPath = join(work, 'session.json')
    writeFileSync(configPath, JSON.stringify(PROXY_CONFIG))
    proxy = spawn('node', [PROXY, '--config', configPath, '--out', outPath, '--', 'node', WDK_MCP], { env, stdio: ['pipe', 'pipe', 'ignore'] })

    let buffer = ''
    const waiting = new Map()
    proxy.stdout.on('data', (chunk) => {
      buffer += chunk
      let end
      while ((end = buffer.indexOf('\n')) >= 0) {
        const line = buffer.slice(0, end)
        buffer = buffer.slice(end + 1)
        if (line.trim() === '') continue
        const message = JSON.parse(line)
        if (message.id !== undefined && waiting.has(message.id)) { waiting.get(message.id)(message); waiting.delete(message.id) }
      }
    })
    let id = 0
    const rpc = (method, params) => new Promise((resolve) => { const n = ++id; waiting.set(n, resolve); proxy.stdin.write(JSON.stringify({ jsonrpc: '2.0', id: n, method, params }) + '\n') })
    const call = async (name, args) => {
      const { result } = await rpc('tools/call', { name, arguments: args })
      return { isError: result.isError === true, body: JSON.parse(result.content[0].text) }
    }

    await rpc('initialize', { protocolVersion: '2025-06-18', capabilities: {}, clientInfo: { name: 'acta-s11-driver', version: '0.1.0' } })
    proxy.stdin.write(JSON.stringify({ jsonrpc: '2.0', method: 'notifications/initialized' }) + '\n')
    const catalog = await rpc('tools/list', {})

    const results = {
      tools: catalog.result.tools.map((tool) => tool.name),
      balance: await call('get_balance', { network: 'localdev' }),
      preview: await call('send_token', { network: 'localdev', to: RECIPIENT, amount: '0.01' }),
      previewed: await call('send_token', { network: 'localdev', to: RECIPIENT, amount: '0.01', dryRun: false }),
      // No preview was requested for this amount. The server executes it all the same.
      unpreviewed: await call('send_token', { network: 'localdev', to: RECIPIENT, amount: '0.02', dryRun: false }),
      tooMuch: await call('send_token', { network: 'localdev', to: RECIPIENT, amount: '1000', dryRun: false }),
      signed: await call('sign_message', { network: 'localdev', message: 'acta s11 demo' })
    }

    const closed = new Promise((resolve) => proxy.on('exit', resolve))
    proxy.stdin.end()
    await closed
    proxy = null
    return { results, address, session: JSON.parse(readFileSync(outPath, 'utf8')) }
  } finally {
    if (proxy) proxy.kill()
    try { wdk('wallet', 'lock', '--all') } catch {}
    await chain.close()
  }
}
