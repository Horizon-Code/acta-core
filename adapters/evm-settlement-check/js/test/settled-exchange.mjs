// One x402 payment that really settles, on a throwaway local chain.
//
// Real: a WDK wallet signing the EIP-3009 authorization, the x402 client and Express
// middleware, the x402 facilitator code verifying and submitting it, and a token transfer
// mined in a block. Local: the chain is an in-process Ganache and the token is the test
// contract in ../../contracts, minted from nothing. No public network is touched and nothing
// of value moves.
//
// Both sides are captured with the ratified ACTA recorder, and the two JSON-RPC response
// bodies a verifier needs are kept exactly as the node sent them over HTTP.

import express from 'express'
import ganache from 'ganache'
import { readFileSync } from 'node:fs'
import { createWalletClient, custom, publicActions, defineChain } from 'viem'
import { privateKeyToAccount, generatePrivateKey } from 'viem/accounts'
import WalletManagerEvm from '@tetherto/wdk-wallet-evm'
import { x402Client, wrapFetchWithPayment } from '@x402/fetch'
import { registerExactEvmScheme } from '@x402/evm/exact/client'
import { paymentMiddleware, x402ResourceServer } from '@x402/express'
import { ExactEvmScheme as ServerScheme } from '@x402/evm/exact/server'
import { ExactEvmScheme as FacilitatorScheme } from '@x402/evm/exact/facilitator'
import { toFacilitatorEvmSigner } from '@x402/evm'
import { x402Facilitator } from '@x402/core/facilitator'
import { createBuyerRecorder, createSellerRecorder } from '../../../x402-wdk/js/src/recorder.mjs'

export const CHAIN_ID = 1337
export const NETWORK = `eip155:${CHAIN_ID}`
export const SELLER_ADDRESS = '0x000000000000000000000000000000000000dEaD'
export const PRICE = '1000'
// The public test mnemonic every EVM toolchain ships with. It holds nothing.
const PUBLIC_TEST_SEED = 'test test test test test test test test test test test junk'
const token = JSON.parse(readFileSync(new URL('../../contracts/TestToken.json', import.meta.url), 'utf8'))

export async function runSettledExchange ({ whileChainIsUp } = {}) {
  const node = ganache.server({ chain: { chainId: CHAIN_ID }, logging: { quiet: true } })
  await node.listen(0, '127.0.0.1')
  const rpcUrl = `http://127.0.0.1:${node.address().port}`
  const chain = defineChain({ id: CHAIN_ID, name: 'local', nativeCurrency: { name: 'ETH', symbol: 'ETH', decimals: 18 }, rpcUrls: { default: { http: [rpcUrl] } } })
  let server
  try {
    const account = await new WalletManagerEvm(PUBLIC_TEST_SEED, { provider: rpcUrl }).getAccount()
    const buyerAddress = await account.getAddress()

    // The facilitator pays the gas. Its key is generated here and discarded.
    const facilitatorAccount = privateKeyToAccount(generatePrivateKey())
    await node.provider.request({ method: 'evm_setAccountBalance', params: [facilitatorAccount.address, '0x8AC7230489E80000'] })
    const facilitatorClient = createWalletClient({ account: facilitatorAccount, chain, transport: custom(node.provider) }).extend(publicActions)
    const deployHash = await facilitatorClient.deployContract({ abi: token.abi, bytecode: token.bytecode, args: [buyerAddress, 5_000_000n] })
    const asset = (await facilitatorClient.waitForTransactionReceipt({ hash: deployHash })).contractAddress

    const facilitator = new x402Facilitator().register(NETWORK, new FacilitatorScheme(toFacilitatorEvmSigner(facilitatorClient)))
    const inProcessFacilitator = {
      verify: (payload, requirements) => facilitator.verify(payload, requirements),
      settle: (payload, requirements) => facilitator.settle(payload, requirements),
      getSupported: async () => facilitator.getSupported()
    }

    const buyerRef = `${NETWORK}:${buyerAddress}`
    const sellerRef = `${NETWORK}:${SELLER_ADDRESS}`
    const sellerRecorder = createSellerRecorder({ recorderRef: sellerRef, counterpartyRef: buyerRef })
    const buyerRecorder = createBuyerRecorder({ recorderRef: buyerRef, counterpartyRef: sellerRef })

    const app = express()
    app.use(sellerRecorder.middleware)
    app.use(paymentMiddleware({
      'GET /weather': {
        accepts: [{ scheme: 'exact', network: NETWORK, price: { amount: PRICE, asset, extra: { name: 'USDT0', version: '1', decimals: 6 } }, payTo: SELLER_ADDRESS }],
        description: 'Weather data',
        mimeType: 'application/json'
      }
    }, new x402ResourceServer(inProcessFacilitator).register(NETWORK, new ServerScheme())))
    app.get('/weather', (req, res) => res.json({ weather: 'sunny', temperature: 70 }))
    server = await new Promise((resolve) => { const s = app.listen(0, '127.0.0.1', () => resolve(s)) })

    const client = new x402Client()
    client.setSpendControls({ allowedAssets: [{ network: NETWORK, asset, maxAmountPerPayment: '1000000' }] })
    registerExactEvmScheme(client, { signer: { address: buyerAddress, signTypedData: (data) => account.signTypedData(data) } })
    const response = await wrapFetchWithPayment(buyerRecorder.wrap(fetch), client)(`http://127.0.0.1:${server.address().port}/weather`)
    const body = await response.text()
    const receipt = JSON.parse(Buffer.from(response.headers.get('payment-response'), 'base64').toString())

    // The node's answers over HTTP JSON-RPC, kept as the response bodies it sent.
    const ask = async (method, params) => {
      const answer = await fetch(rpcUrl, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }) })
      return await answer.text()
    }
    const rpc = {
      eth_chainId: await ask('eth_chainId', []),
      eth_getTransactionReceipt: await ask('eth_getTransactionReceipt', [receipt.transaction])
    }
    const sellerBalance = await facilitatorClient.readContract({ address: asset, abi: token.abi, functionName: 'balanceOf', args: [SELLER_ADDRESS] })

    const run = { status: response.status, body, receipt, asset, buyerAddress, sellerBalance, buyer: buyerRecorder.records[0], seller: sellerRecorder.records[0], rpc }
    if (whileChainIsUp) run.live = await whileChainIsUp(run, rpcUrl)
    return run
  } finally {
    if (server) await new Promise((resolve) => server.close(resolve))
    await node.close()
  }
}
