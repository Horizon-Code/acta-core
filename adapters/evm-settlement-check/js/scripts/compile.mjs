// Compiles contracts/TestToken.sol into the retained artifact. The artifact is committed so
// the tests do not need the compiler; re-running this must reproduce it byte for byte.
import solc from 'solc'
import { readFileSync, writeFileSync } from 'node:fs'

const source = readFileSync(new URL('../../contracts/TestToken.sol', import.meta.url), 'utf8')
const input = {
  language: 'Solidity',
  sources: { 'TestToken.sol': { content: source } },
  settings: { optimizer: { enabled: true, runs: 200 }, evmVersion: 'paris', outputSelection: { '*': { '*': ['abi', 'evm.bytecode.object'] } } }
}
const output = JSON.parse(solc.compile(JSON.stringify(input)))
for (const error of output.errors ?? []) {
  if (error.severity === 'error') throw new Error(error.formattedMessage)
}
const contract = output.contracts['TestToken.sol'].TestToken
const artifact = { compiler: solc.version(), settings: input.settings, abi: contract.abi, bytecode: '0x' + contract.evm.bytecode.object }
writeFileSync(new URL('../../contracts/TestToken.json', import.meta.url), JSON.stringify(artifact, null, 2) + '\n')
console.log(`compiled with ${artifact.compiler}: ${(artifact.bytecode.length - 2) / 2} bytes`)
