#!/usr/bin/env node

const {
  AbiCoder,
  Contract,
  JsonRpcProvider,
  Wallet,
  ZeroAddress: ZERO_ADDRESS,
  ZeroHash: ZERO_BYTES32,
  solidityPackedKeccak256
} = require('ethers');
const fs = require('node:fs');

const SCHEMA_REGISTRY_ABI = [
  'function register(string schema,address resolver,bool revocable) returns (bytes32)',
  'function getSchema(bytes32 uid) view returns ((bytes32 uid,address resolver,bool revocable,string schema))'
];
const EAS_ABI = [
  'function attest((bytes32 schema,(address recipient,uint64 expirationTime,bool revocable,bytes32 refUID,bytes data,uint256 value) data) request) payable returns (bytes32)',
  'event Attested(address indexed recipient,address indexed attester,bytes32 uid,bytes32 indexed schemaUID)'
];

const CHAIN_ID = 84532n;
const NETWORK = 'eip155:84532';
const EAS_ADDRESS = '0x4200000000000000000000000000000000000021';
const SCHEMA_REGISTRY_ADDRESS = '0x4200000000000000000000000000000000000020';
const SCHEMA = 'bytes32 epochRoot';
const EVIDENCE_VERSION = 'acta.eas-anchor-evidence.v1';

function normalizeEpochRoot(value) {
  if (!/^[0-9a-f]{64}$/.test(value)) {
    throw new Error('epoch_root must be exactly 64 lowercase hexadecimal characters');
  }
  return value;
}

function schemaUID() {
  return solidityPackedKeccak256(
    ['string', 'address', 'bool'],
    [SCHEMA, ZERO_ADDRESS, false]
  );
}

function evidenceFromReceipt({ epochRoot, uid, attester, receipt }) {
  if (!receipt || receipt.status !== 1) {
    throw new Error('EAS attestation transaction was not successful');
  }
  return {
    evidence_version: EVIDENCE_VERSION,
    anchor_ref: {
      substrate: 'eas',
      network: NETWORK,
      tx_id: receipt.hash.toLowerCase(),
      slot: Number(receipt.blockNumber),
      epoch_root: normalizeEpochRoot(epochRoot)
    },
    eas: {
      chain_id: Number(CHAIN_ID),
      contract_address: EAS_ADDRESS,
      schema_registry_address: SCHEMA_REGISTRY_ADDRESS,
      schema_uid: schemaUID().toLowerCase(),
      attestation_uid: uid.toLowerCase(),
      attester: attester.toLowerCase(),
      schema: SCHEMA,
      resolver: ZERO_ADDRESS,
      revocable: false
    }
  };
}

function attachAnchor(bundle, evidence) {
  if (evidence?.evidence_version !== EVIDENCE_VERSION) {
    throw new Error(`anchor evidence must use ${EVIDENCE_VERSION}`);
  }
  if (!bundle || bundle.epoch_root !== evidence.anchor_ref?.epoch_root) {
    throw new Error('bundle epoch_root does not match anchor evidence');
  }
  if (bundle.anchor !== null && bundle.anchor !== undefined) {
    throw new Error('refusing to replace an existing bundle anchor');
  }
  return { ...bundle, anchor: evidence.anchor_ref };
}

function attachFiles(evidencePath, bundlePath, outputPath) {
  const evidence = JSON.parse(fs.readFileSync(evidencePath, 'utf8'));
  const bundle = JSON.parse(fs.readFileSync(bundlePath, 'utf8'));
  const attached = attachAnchor(bundle, evidence);
  fs.writeFileSync(outputPath, `${JSON.stringify(attached, null, 2)}\n`, {
    encoding: 'utf8',
    flag: 'wx'
  });
  return { output: outputPath, epoch_root: attached.epoch_root };
}

function signerFromEnvironment() {
  const rpcUrl = process.env.ACTA_EAS_RPC_URL ?? 'https://sepolia.base.org';
  const privateKey = process.env.ACTA_EVM_ANCHOR_PRIVATE_KEY;
  if (!privateKey) {
    throw new Error(
      'ACTA_EVM_ANCHOR_PRIVATE_KEY is not set; provide it out of band through the environment'
    );
  }
  const provider = new JsonRpcProvider(rpcUrl);
  return { provider, signer: new Wallet(privateKey, provider) };
}

async function requireBaseSepolia(provider) {
  const network = await provider.getNetwork();
  if (network.chainId !== CHAIN_ID) {
    throw new Error(`wrong chain id: expected ${CHAIN_ID}, received ${network.chainId}`);
  }
  const easCode = await provider.getCode(EAS_ADDRESS);
  const registryCode = await provider.getCode(SCHEMA_REGISTRY_ADDRESS);
  if (easCode === '0x' || registryCode === '0x') {
    throw new Error('official EAS contracts are not deployed at the pinned addresses');
  }
}

async function registerSchema() {
  const { provider, signer } = signerFromEnvironment();
  await requireBaseSepolia(provider);
  const registry = new Contract(SCHEMA_REGISTRY_ADDRESS, SCHEMA_REGISTRY_ABI, signer);
  const uid = schemaUID();
  const existing = await registry.getSchema(uid);
  if (existing.uid !== ZERO_BYTES32) {
    requireExactSchema(existing);
    return { schema_uid: uid.toLowerCase(), already_registered: true };
  }
  const transaction = await registry.register(SCHEMA, ZERO_ADDRESS, false);
  const receipt = await transaction.wait(2);
  return {
    schema_uid: uid.toLowerCase(),
    already_registered: false,
    transaction_id: receipt.hash.toLowerCase(),
    block_number: Number(receipt.blockNumber)
  };
}

async function publish(epochRoot) {
  normalizeEpochRoot(epochRoot);
  const { provider, signer } = signerFromEnvironment();
  await requireBaseSepolia(provider);
  const registry = new Contract(SCHEMA_REGISTRY_ADDRESS, SCHEMA_REGISTRY_ABI, provider);
  const uid = schemaUID();
  const record = await registry.getSchema(uid);
  if (record.uid === ZERO_BYTES32) {
    throw new Error('ACTA EAS schema is absent; run register-schema');
  }
  requireExactSchema(record);

  const data = AbiCoder.defaultAbiCoder().encode(['bytes32'], [`0x${epochRoot}`]);
  const eas = new Contract(EAS_ADDRESS, EAS_ABI, signer);
  const transaction = await eas.attest({
    schema: uid,
    data: {
      recipient: ZERO_ADDRESS,
      expirationTime: 0n,
      revocable: false,
      refUID: ZERO_BYTES32,
      data,
      value: 0n
    }
  });
  const receipt = await transaction.wait(2);
  const attested = receipt.logs
    .map((log) => {
      try {
        return eas.interface.parseLog(log);
      } catch {
        return null;
      }
    })
    .find((event) => event?.name === 'Attested');
  if (!attested) {
    throw new Error('confirmed transaction does not contain an EAS Attested event');
  }
  return evidenceFromReceipt({
    epochRoot,
    uid: attested.args.uid,
    attester: signer.address,
    receipt
  });
}

function requireExactSchema(record) {
  if (
    record.schema !== SCHEMA ||
    record.resolver.toLowerCase() !== ZERO_ADDRESS ||
    record.revocable
  ) {
    throw new Error('existing schema UID does not match the ACTA schema profile');
  }
}

async function main(argv) {
  const [command, ...arguments_] = argv;
  if (command === 'register-schema' && arguments_.length === 0) {
    return registerSchema();
  }
  if (command === 'publish' && arguments_.length === 1) {
    return publish(arguments_[0]);
  }
  if (command === 'attach' && arguments_.length === 3) {
    return attachFiles(...arguments_);
  }
  throw new Error(
    'usage: eas-anchor.cjs register-schema | publish <epoch-root> | attach <evidence.json> <bundle.json> <output.json>'
  );
}

module.exports = {
  CHAIN_ID,
  NETWORK,
  EAS_ADDRESS,
  SCHEMA_REGISTRY_ADDRESS,
  SCHEMA,
  EVIDENCE_VERSION,
  normalizeEpochRoot,
  schemaUID,
  evidenceFromReceipt,
  attachAnchor,
  main
};

if (require.main === module) {
  main(process.argv.slice(2))
    .then((result) => process.stdout.write(`${JSON.stringify(result, null, 2)}\n`))
    .catch((error) => {
      process.stderr.write(`eas-anchor: ${error.message}\n`);
      process.exitCode = 1;
    });
}
