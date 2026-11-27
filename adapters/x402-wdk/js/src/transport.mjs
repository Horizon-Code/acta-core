// ACTA evidence transport over Hypercore (implementation candidate, Proposed ADR-020).
//
// x402 gives a seller no way to hand its evidence to the buyer. This carries the seller's
// bundles to the buyer as blocks of a Hypercore: one bundle per block, bytes untouched.
//
// Measured before building (research/hypercore-mini-e0-2026-11-27.md): a Hypercore proves who
// wrote a block, but its writer can truncate the log and write a different history, and a
// reader that keeps following the log adopts the new history silently — only the `fork`
// counter moves. So this module does not treat the log as evidence of anything. The buyer
// takes a checkpoint when it reads, and `compareWithCheckpoint` is what notices a rewrite.
//
// The caller supplies the Hypercore instances and connects them. Discovering peers over a
// network (Hyperswarm) is outside this module and has not been exercised.

export const EVIDENCE_HEADER = 'acta-evidence-core'

// Bare has no TextDecoder (measured); Hypercore blocks are Buffers there.
const utf8 = (block) => typeof TextDecoder !== 'undefined' ? new TextDecoder().decode(block) : block.toString('utf8')
const hex = (bytes) => Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('')

/** Seller: append bundles, one per block, exactly as given. */
export async function publishBundles (core, bundles) {
  if (!core.writable) throw new Error('acta transport: this peer cannot write to the core')
  for (const bundle of bundles) {
    if (!(bundle instanceof Uint8Array)) throw new TypeError('acta transport: bundles must be bytes')
  }
  await core.append(bundles)
  return checkpoint(core)
}

/** What a reader must keep to notice a later rewrite. */
export async function checkpoint (core) {
  await core.ready()
  return {
    schema: 'acta.transport.checkpoint.v0',
    core_key: hex(core.key),
    length: core.length,
    fork: core.fork,
    tree_hash: hex(await core.treeHash(core.length))
  }
}

/**
 * Buyer: read every block and keep those that belong to one transaction. Blocks are parsed
 * only to read `event.process_ref.process_id` (the portable bundle keeps the Core bundle flattened at its root); the bytes returned are the bytes read.
 * Returns the checkpoint taken at the moment of reading.
 */
export async function readBundles (core, { transactionId, timeout = 10000 } = {}) {
  if (typeof transactionId !== 'string' || transactionId === '') {
    throw new TypeError('acta transport: transactionId is required')
  }
  await core.update({ wait: true })
  const at = await checkpoint(core)
  const bundles = []
  for (let index = 0; index < at.length; index++) {
    const block = await core.get(index, { timeout })
    let processId = null
    try {
      processId = JSON.parse(utf8(block))?.event?.process_ref?.process_id ?? null
    } catch {
      // Not a bundle. Left out, not repaired.
    }
    if (processId === transactionId) bundles.push(new Uint8Array(block))
  }
  return { bundles, checkpoint: at }
}

/**
 * Compare the core as it is now with a checkpoint taken earlier. An empty list means the
 * history up to the checkpoint is the one that was read.
 */
export async function compareWithCheckpoint (core, retained) {
  await core.ready()
  const findings = []
  if (hex(core.key) !== retained.core_key) {
    return [{ finding: 'different_core', retained: retained.core_key, current: hex(core.key) }]
  }
  if (core.fork !== retained.fork) {
    findings.push({ finding: 'history_rewritten', retained_fork: retained.fork, current_fork: core.fork })
  }
  if (core.length < retained.length) {
    findings.push({ finding: 'history_shortened', retained_length: retained.length, current_length: core.length })
  } else {
    const current = hex(await core.treeHash(retained.length))
    if (current !== retained.tree_hash) {
      findings.push({ finding: 'tree_hash_changed', at_length: retained.length, retained: retained.tree_hash, current })
    }
  }
  return findings
}

/**
 * Seller: Express-style middleware that tells a paying buyer where the evidence will be, by
 * adding one response header to paid requests. Mount it before the payment middleware.
 */
export function createEvidenceAdvertiser ({ coreKey }) {
  const value = typeof coreKey === 'string' ? coreKey : hex(coreKey)
  if (!/^[0-9a-f]{64}$/.test(value)) throw new TypeError('acta transport: coreKey must be 32 bytes')
  return function advertiseEvidence (req, res, next) {
    const headers = req.headers || {}
    if (Object.keys(headers).some((name) => name.toLowerCase() === 'payment-signature')) {
      res.setHeader(EVIDENCE_HEADER, value)
    }
    next()
  }
}
