const assert = require('node:assert/strict');
const test = require('node:test');
const {
  EAS_ADDRESS,
  EVIDENCE_VERSION,
  attachAnchor,
  evidenceFromReceipt,
  normalizeEpochRoot,
  schemaUID
} = require('../src/eas-anchor.cjs');

const ROOT = 'aa'.repeat(32);

test('epoch root syntax is strict and schema UID is stable', () => {
  assert.equal(normalizeEpochRoot(ROOT), ROOT);
  assert.throws(() => normalizeEpochRoot(ROOT.toUpperCase()));
  assert.match(schemaUID(), /^0x[0-9a-f]{64}$/);
  assert.equal(schemaUID(), schemaUID());
});

test('confirmed receipt maps to an ACTA sidecar without hiding the transaction hash', () => {
  const result = evidenceFromReceipt({
    epochRoot: ROOT,
    uid: `0x${'bb'.repeat(32)}`,
    attester: `0x${'11'.repeat(20)}`,
    receipt: {
      status: 1,
      hash: `0x${'cc'.repeat(32)}`,
      blockNumber: 123n
    }
  });
  assert.equal(result.evidence_version, EVIDENCE_VERSION);
  assert.equal(result.anchor_ref.tx_id, `0x${'cc'.repeat(32)}`);
  assert.equal(result.anchor_ref.slot, 123);
  assert.equal(result.anchor_ref.epoch_root, ROOT);
  assert.equal(result.eas.contract_address, EAS_ADDRESS);
});

test('failed receipt cannot be represented as external custody', () => {
  assert.throws(() =>
    evidenceFromReceipt({
      epochRoot: ROOT,
      uid: `0x${'bb'.repeat(32)}`,
      attester: `0x${'11'.repeat(20)}`,
      receipt: { status: 0, hash: `0x${'cc'.repeat(32)}`, blockNumber: 123n }
    })
  );
});

test('anchor attachment preserves bundle fields and refuses root mismatch', () => {
  const evidence = evidenceFromReceipt({
    epochRoot: ROOT,
    uid: `0x${'bb'.repeat(32)}`,
    attester: `0x${'11'.repeat(20)}`,
    receipt: { status: 1, hash: `0x${'cc'.repeat(32)}`, blockNumber: 123n }
  });
  const bundle = { protocol: 'acta.v0', epoch_root: ROOT, anchor: null };
  const attached = attachAnchor(bundle, evidence);
  assert.equal(attached.protocol, bundle.protocol);
  assert.deepEqual(attached.anchor, evidence.anchor_ref);
  assert.equal(bundle.anchor, null);
  assert.throws(() =>
    attachAnchor({ ...bundle, epoch_root: 'dd'.repeat(32) }, evidence)
  );
  assert.throws(() => attachAnchor(attached, evidence));
});
