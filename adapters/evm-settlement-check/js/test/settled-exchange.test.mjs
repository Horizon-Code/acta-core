import test from 'node:test'
import assert from 'node:assert/strict'
import { runSettledExchange, NETWORK, PRICE, SELLER_ADDRESS } from './settled-exchange.mjs'

test('an x402 payment signed by a WDK wallet settles on a local chain', async () => {
  const run = await runSettledExchange()
  assert.equal(run.status, 200)
  assert.equal(run.receipt.success, true)
  assert.equal(run.receipt.network, NETWORK)
  assert.equal(run.receipt.payer.toLowerCase(), run.buyerAddress.toLowerCase())
  assert.equal(run.sellerBalance, BigInt(PRICE), 'the seller holds exactly what was paid')

  // Both sides recorded the same receipt, and it names a mined transaction.
  assert.equal(run.buyer.payment_response_header, run.seller.payment_response_header)
  const mined = JSON.parse(run.rpc.eth_getTransactionReceipt).result
  assert.equal(mined.transactionHash, run.receipt.transaction)
  assert.equal(mined.status, '0x1')
  assert.equal(mined.to.toLowerCase(), run.asset.toLowerCase())
  assert.equal(mined.logs.length, 2)
  assert.ok(mined.logs.every((log) => log.address.toLowerCase() === run.asset.toLowerCase()))
  assert.equal(JSON.parse(run.rpc.eth_chainId).result, '0x539')
  assert.ok(SELLER_ADDRESS)
})
