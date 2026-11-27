// One real x402 v2 exchange on localhost: a WDK wallet signs a real EIP-3009 authorization,
// the real x402 client and Express middleware carry it, and both ACTA recorders capture it.
//
// The facilitator is a stand-in. It accepts every payment and returns a made-up transaction
// hash, so NOTHING is verified or settled on any chain. The records are real protocol bytes
// around a settlement that did not happen.

import express from 'express'
import WalletManagerEvm from '@tetherto/wdk-wallet-evm'
import { x402Client, wrapFetchWithPayment } from '@x402/fetch'
import { registerExactEvmScheme } from '@x402/evm/exact/client'
import { paymentMiddleware, x402ResourceServer } from '@x402/express'
import { ExactEvmScheme } from '@x402/evm/exact/server'
import { createBuyerRecorder, createSellerRecorder } from '../src/recorder.mjs'

export const NETWORK = 'eip155:9745' // Plasma
export const USDT0 = '0xB8CE59FC3717ada4C02eaDF9682A9e934F625ebb'
export const SELLER_ADDRESS = '0x000000000000000000000000000000000000dEaD'
export const STAND_IN_TRANSACTION = '0x' + 'ab'.repeat(32)
// The public test mnemonic every EVM toolchain ships with. It holds nothing.
const PUBLIC_TEST_SEED = 'test test test test test test test test test test test junk'

export async function runLocalExchange () {
  const account = await new WalletManagerEvm(PUBLIC_TEST_SEED, { provider: 'http://127.0.0.1:1' }).getAccount()
  const buyerAddress = await account.getAddress()
  const buyerRef = `${NETWORK}:${buyerAddress}`
  const sellerRef = `${NETWORK}:${SELLER_ADDRESS}`

  const standInFacilitator = {
    async verify () { return { isValid: true, payer: buyerAddress } },
    async settle () { return { success: true, payer: buyerAddress, transaction: STAND_IN_TRANSACTION, network: NETWORK } },
    async getSupported () { return { kinds: [{ x402Version: 2, scheme: 'exact', network: NETWORK }], extensions: [], signers: {} } }
  }

  const sellerRecorder = createSellerRecorder({ recorderRef: sellerRef, counterpartyRef: buyerRef })
  const buyerRecorder = createBuyerRecorder({ recorderRef: buyerRef, counterpartyRef: sellerRef })

  const app = express()
  app.use(sellerRecorder.middleware)
  app.use(paymentMiddleware({
    'GET /weather': {
      accepts: [{
        scheme: 'exact',
        network: NETWORK,
        price: { amount: '1000', asset: USDT0, extra: { name: 'USDT0', version: '1', decimals: 6 } },
        payTo: SELLER_ADDRESS
      }],
      description: 'Weather data',
      mimeType: 'application/json'
    }
  }, new x402ResourceServer(standInFacilitator).register(NETWORK, new ExactEvmScheme())))
  app.get('/weather', (req, res) => res.json({ weather: 'sunny', temperature: 70 }))

  const server = await new Promise((resolve) => { const s = app.listen(0, '127.0.0.1', () => resolve(s)) })
  try {
    const client = new x402Client()
    client.setSpendControls({ allowedAssets: [{ network: NETWORK, asset: USDT0, maxAmountPerPayment: '1000000' }] })
    registerExactEvmScheme(client, { signer: { address: buyerAddress, signTypedData: (data) => account.signTypedData(data) } })
    const fetchWithPayment = wrapFetchWithPayment(buyerRecorder.wrap(fetch), client)
    const response = await fetchWithPayment(`http://127.0.0.1:${server.address().port}/weather`)
    const body = await response.text()
    return { status: response.status, body, buyer: buyerRecorder.records, seller: sellerRecorder.records }
  } finally {
    await new Promise((resolve) => server.close(resolve))
  }
}
