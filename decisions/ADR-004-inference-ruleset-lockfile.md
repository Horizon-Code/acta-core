# ADR-004: Inference Ruleset Lockfile Shape

- Status: Proposed — pending operator ratification
- Date: 2026-08-31
- Evidence: `research/E0-resultados.md`, captures 2 and 3

## Context

An inference result is not reproducible from one declared rule file when that file imports a
transitive closure of MeTTa and Python code, the runtime semantics depend on an engine, and the
agent harness transforms results before the next hop.

E0 enumerated 34 deployed source files (21 `.metta`, 13 `.py`) and matched them 34/34 against
the image. The closure remained unchanged after execution. Provenance is recoverable in two
different ways: some components retain `.git`; OmegaClaw deliberately removes it but remains
fixed by the immutable image digest, with a declared version/tag available as an optional
public cross-check.

## Proposed decision

A `PolicySnapshotV0` whose `policy_type` is `inference_ruleset` commits to a **static manifest
generated before execution**, not to one rule blob.

The manifest contains at least:

1. every source entry in the transitive executable closure, with normalized path and SHA-256;
2. a provenance root for each component:
   - when `.git` survives: the commit read from the artifact;
   - without `.git`: immutable image digest plus declared version, with verification against a
     public tag optional and subordinate to the digest;
3. inference engine identity and version;
4. harness source/version and configuration that affects the declared link transformation
   `T`, including `maxFeedback`, `maxHistory`, `string-safe` and `normalize_string`.

The canonical manifest hash is the `policy_hash` of the `PolicySnapshotV0`. The exact wire
schema and canonical serialization are specified by the Profile that implements this ADR and
must be versioned.

The manifest covers executable closure regardless of language. A `git-import!` call that finds
an already populated directory is not itself the pin; the manifest records what actually
entered the artifact.

Static import expressions that cannot be resolved at generation time are recorded explicitly
as unresolved entries, including their source location and literal expression. A generator
must not silently omit them or claim a complete closure. The measured OmegaClaw case includes
the dynamic `src/context` expression as the initial negative reference vector for this rule.

This mechanism belongs to Profile/verifier code. ACTA Core only carries and commits the neutral
`PolicySnapshotV0` fields.

## Consequences

- A third party can identify which file, engine or harness parameter diverged instead of seeing
  only an opaque policy-hash mismatch.
- Rebuilding from mutable tags or branches is detectable because the resulting hashes/digest
  differ.
- The image digest remains the provenance root when a public tag disappears or moves.
- Harness changes cannot alter faithful/free classification invisibly.
- The OmegaClaw v0.1.19 closure measured by E0 becomes the first reference vector for the
  generator and its tests.

## Ratification

Accepting this ADR requires the operator's explicit approval under ADR-001. Until then it is a
proposal and does not ratify E-1, E-2 or E-4 by itself.
