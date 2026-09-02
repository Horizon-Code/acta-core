# Current State

- Authority-based repository structure is in place.
- Core Rust crate is located at `core/rust/acta-core`.
- AML profile helpers are separated under `profiles/aml/rust`.
- Architecture/profile/research/roadmap artifacts have been relocated by authority.
- E0 gate is **open, 4/4 complete**: all captures have measured data and a written decision.
  Capture 3 measured a 5/5 ceiling in isomorphic tasks under explicit byte-copy instructions;
  it is not a general mediator rate. Auxiliary attempts recorded omission and recomputation.
  C2 is unblocked by E0 and the ratified E-1/E-2/E-6 package. S6 is closed in executable work
  as of 2026-09-01. ADR-009 was Accepted on 2026-09-02 with `provisional` lifecycle status;
  Artefact 1 custody/anchoring remains the sole S6 deployment gate and is deferred to S8.
- ADR-003, ADR-004 and ADR-005 were ratified by the operator on 2026-08-31. Their equality,
  lockfile-shape and raw-commitment-point decisions are binding.
- ADR-007 and ADR-008 materialize ratified E-1 and E-2: the four report codes use mechanically
  evaluable conditions, and mediated-link fidelity is byte equality against declared `T(C)`
  with same `process_ref`, Chronos precedence and an inseparable selection disclaimer.
- ADR-006 was ratified on 2026-08-31: Chronos continuity does not reset at epoch boundaries.
  Its implementation and future `TR-CHRONOS-BOUNDARY-UNVERIFIED` report line remain
  unstarted pending an explicit execution order.
- ADR-009 was ratified by the operator on 2026-09-02 against commit
  `19dde9f4143e4ea61102db5647cc8d2698c266c6` and its exact eight-file hash set. The
  Cognitive Forensics Profile is Accepted with `provisional` lifecycle status; the eight
  submitted files remain byte-identical to the ratified set.
- E-9, “Reposicionamiento: ACTA como capa de confianza de la economía entre agentes”, is
  **Accepted** as part of the E-9 + E-1 + E-2 + E-6 package ratified by the operator on
  2026-08-31 against commit `503aefbd0636ba3ef52b782c675b1a901e27466a`. The
  Cardano/agents/EVM freeze is lifted. The active
  direction is EVM/Base-first with multi-anchor neutrality, Cardano in the catalog,
  `agent_commerce` as the next commercial Profile, OpenWorker as the public level-1 target,
  OmegaClaw as the level-2 ceiling, and AML retained only as an internal reference.
- S7 produced the exact Agent Commerce Profile v1.0 candidate and reference validator under
  `profiles/agent-commerce/`. ADR-010 proposes the Profile with `experimental` lifecycle;
  ADR-011 separately proposes cross-attestation semantics and the independent-signer
  predicate. Both remain non-binding pending operator ratification. The current verifier
  still emits `TR-SIGNER-SELF` for any producer self-signature.
- E-9.4 still requires its own Proposed-to-Accepted ADR before implementation fixes adapter
  wire details. S8 is not authorized, and a real anchor adapter remains gated on a real
  counterparty.
- C2 Artefact 1 seals exact OmegaClaw history records into signed Chronos/Merkle bundles and
  detects a silent deletion against a witness placed in a separate local domain. It passed on
  its fixture and on a read-only copy of the real 100-record E0 history; the original volume
  was untouched. That run is a topology simulation, not independent custody.
- El Artefacto 1 está **implementado y verificado, no demostrable en público**. La detección
  del borrado es probatoria ante un tercero solo cuando el compromiso se conserva fuera del
  control del operador. La custodia independiente o el anclaje externo llegan en **S8**
  (adaptador EVM/EAS). Hasta entonces, la demo no se presenta a terceros ni se publica.
- C2 Artefact 2 applies ADR-008 to the real E0 strings, emits
  `TR-CHAIN-MEDIATED`/`TR-NONDET-INPUT`, preserves the selection disclaimer and provides the
  pinned source patch at the raw `(eval $s)` boundary. The historical E0 process/Chronos fields
  are explicitly test envelopes, not retroactive ACTA receipts.
- A1 and A3 are implemented outside Core in `adapters/attestation-single-signer/rust`:
  Ed25519 signatures verify offline against public keys carried inline at the bundle JSON root.
  A manipulated signature is rejected, and the external report emits `TR-KEY-SELF-ASSERTED`
  structurally plus `TR-SIGNER-SELF` when detected. The portable reference vector is
  `protocol/test-vectors/verifiable-bundle-v0.json`.
- B1 is implemented at `tools/acta-verifier/rust`: the standalone binary accepts the portable
  bundle without network access and produces the same deterministic report as the library.
  Its JSON/text report already carries `TR-NO-ANCHOR`, `TR-ANCHOR-UNVERIFIED`, and
  `TR-TIME-DECLARED`; B2 remains incremental. Under E-9.3 its primary consumer and closure
  priority are machine-first JSON, while human-readable text remains the second
  representation.
- The 31-Aug market-research report cited by E-9 was incorporated literally at
  `research/investigacion-validacion-mercado-2026-08-31.md` on 2026-09-02. E-9 §0 now maps
  every motivating claim to dated report findings and labels the strategic inference
  separately.
- The full documentary triad is in the repo: `acta-estado-consolidado.md`,
  `directiva-construccion-2026-08-31.md`, and `E0-protocolo-y-enmiendas.md`. The §1.2
  vocabulary-scale and §3.1 canonicalization references now resolve.
