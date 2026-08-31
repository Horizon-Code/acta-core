# Current State

- Authority-based repository structure is in place.
- Core Rust crate is located at `core/rust/acta-core`.
- AML profile helpers are separated under `profiles/aml/rust`.
- Architecture/profile/research/roadmap artifacts have been relocated by authority.
- E0 gate is **open, 4/4 complete**: all captures have measured data and a written decision.
  Capture 3 measured a 5/5 ceiling in isomorphic tasks under explicit byte-copy instructions;
  it is not a general mediator rate. Auxiliary attempts recorded omission and recomputation.
  C2 is unblocked by E0 but remains unstarted pending E-1/E-2 ratification and its profile.
- ADR-003, ADR-004 and ADR-005 were ratified by the operator on 2026-08-31. Their equality,
  lockfile-shape and raw-commitment-point decisions are binding; E-1/E-2 remain separate.
- A1 and A3 are implemented outside Core in `adapters/attestation-single-signer/rust`:
  Ed25519 signatures verify offline against public keys carried inline at the bundle JSON root.
  A manipulated signature is rejected, and the external report emits `TR-KEY-SELF-ASSERTED`
  structurally plus `TR-SIGNER-SELF` when detected. The portable reference vector is
  `protocol/test-vectors/verifiable-bundle-v0.json`.
- B1 is implemented at `tools/acta-verifier/rust`: the standalone binary accepts the portable
  bundle without network access and produces the same deterministic report as the library.
  Its JSON/text report already carries `TR-NO-ANCHOR`, `TR-ANCHOR-UNVERIFIED`, and
  `TR-TIME-DECLARED`; B2 remains incremental because its human-readability criterion still
  requires review by a non-technical reader.
- The full documentary triad is in the repo: `acta-estado-consolidado.md`,
  `directiva-construccion-2026-08-31.md`, and `E0-protocolo-y-enmiendas.md`. The §1.2
  vocabulary-scale and §3.1 canonicalization references now resolve.
