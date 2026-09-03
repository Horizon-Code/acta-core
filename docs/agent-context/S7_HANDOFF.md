# S7 Agent Commerce Handoff

**Completion date:** 2026-09-02
**Authority:** operator acceptance of ADR-009 and explicit S7 authorization against commit
`19dde9f4143e4ea61102db5647cc8d2698c266c6`
**Ratification:** ADR-010 and ADR-011 Accepted by the operator on 2026-09-02 against commit
`ed5cd658d80c55a4780dbe41985010874ae76378` and their fixed hash tables.

## Outcome

S7 prepared one exact Profile submission and two separate decisions, now Accepted:

- `profiles/agent-commerce/` specifies Agent Commerce Profile v1.0 with `experimental`
  lifecycle status through ADR-010.
- ADR-011 names co-signature and reciprocal-event forms and binds the externally resolved
  independent-signer predicate.
- Acceptance does not rewrite the byte-identical seven-file submission or pretend its report
  integration/external identity dependency already exists. S8 was authorized separately.

## Profile scope

The candidate uses namespace `agent_commerce`, process type
`agent_commerce_transaction`, and seven event kinds:

1. `mandate_received`
2. `action_executed_against_mandate`
3. `delivery_committed`
4. `delivery_received`
5. `settlement_requested`
6. `settlement_observed`
7. `dispute_opened`

One canonical CBOR positional-array manifest commits N mandates. Its SHA-256 is the
`PolicySnapshotV0.policy_hash` for `policy_type: agent_mandate`. Manifest entries normalize
set order; transaction events retain Chronos order. Both prepaid and postpaid settlement order
are valid.

The Profile maps form/process/substantive disputes without adjudicating them and adds no Core
or Protocol fields.

## Cross-attestation boundary

- Co-signing one receipt proves agreement over the same committed receipt body and Chronos
  position. It does not prove delivery, receipt, satisfaction or settlement.
- Reciprocal claims are new `delivery_received` or `settlement_observed` events with their own
  receipts and source-event hashes.
- A counter-signature using inline keys is classified as identity unresolved and does not
  remove `TR-SIGNER-SELF`.
- ADR-011 permits full retirement only after cryptographic verification and externally
  resolved bindings show producer and counter-attestor are distinct subjects. That makes
  E-9.2 dependent on E-9.5.
- The accepted reference assessment is not yet wired into the report because external identity
  resolution is absent. Existing report behavior remains unchanged.

## Verification

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo fmt --all -- --check
PATH="$HOME/.cargo/bin:$PATH" cargo test -p acta-agent-commerce-profile
PATH="$HOME/.cargo/bin:$PATH" cargo test --workspace
```

The focused suite contains eight positive/negative tests covering canonical manifest order,
policy binding, duplicate rejection, all seven event mappings, predecessor and transaction
checks, x402-style prepayment, reciprocal delivery binding and the three cross-attestation
outcomes/identity boundary.

## Closure and following gate

- ADR-010 and ADR-011 are Accepted on the exact hashes recorded in them.
- S8 was explicitly authorized by the operator on 2026-09-02. Its adapter/report wire remains
  Proposed under ADR-012/013 until separately ratified.
- S8 treated independent custody/anchoring of the S6 Artefact 1 commitment as a first-class
  requirement and closed it on 2026-09-03 for the demonstration scope stated below.

El Artefacto 1 es **demostrable ante un tercero, con alcance declarado**. El 3-sept-2026 su
raíz de época `bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba` quedó anclada
en Base Sepolia como atestación EAS irrevocable, y la verificación pasó las siete comprobaciones
del predicado de ADR-012 §4 por el camino del tercero: verificador Rust contra Ethereum JSON-RPC
crudo, sin publicador, sin indexador y sin endpoint de ACTA. Registro completo en
`research/s8-eas-base-deployment-2026-09-03.md`.

Cuatro precisiones acompañan siempre a esa afirmación. Presentar la demo omitiendo cualquiera de
ellas la convierte en una afirmación falsa:

- **Alcance.** Lo cerrado es la **custodia de demostración**, y el mecanismo de anclaje está
  verificado de punta a punta.
- **Testnet.** Base Sepolia demuestra el mecanismo ante un tercero; **no es custodia de
  producción**. Un anclaje en red de pruebas prueba que el camino funciona; no equivale a un
  anclaje en mainnet.
- **Attester.** La clave firmante circuló fuera del canal previsto, así que la atestación
  acredita que **el camino funciona, no quién firmó**. `TR-SIGNER-SELF` y
  `TR-KEY-SELF-ASSERTED` siguen puestos.
- **Procedencia.** Los bundles que reproducen la raíz anclada se regeneran de forma determinista
  desde la historia fuente retenida en `fixtures/e0-real-history.metta`. **No se afirma que sean
  los bundles originales del 1-sept.** La raíz se computa sobre los eventos, no sobre los
  receipts: la coincidencia del witness prueba identidad de eventos y de orden, no de firmas.
