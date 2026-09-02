# S8 — verificación de fuentes EAS/Base

**Fecha de comprobación:** 2 de septiembre de 2026

**Naturaleza:** research no normativo; sustenta ADR-012/013, no las acepta.

## Hechos comprobados en fuentes primarias

1. El repositorio oficial de contratos EAS define `SchemaRegistry.register/getSchema`,
   `EAS.attest/getAttestation/isAttestationValid` y el evento `Attested`. Se inspeccionó el
   commit `e6e970286ff18bbdfc5d8eff2742c5ece46040e4` de
   [ethereum-attestation-service/eas-contracts](https://github.com/ethereum-attestation-service/eas-contracts).
2. Los artefactos oficiales de despliegue del mismo commit fijan para Base Sepolia chain id
   `84532`, EAS `0x4200000000000000000000000000000000000021` y Schema Registry
   `0x4200000000000000000000000000000000000020`.
3. La documentación oficial de Base publica `https://sepolia.base.org` como RPC estándar de
   Base Sepolia y `84532` como chain id: [RPC endpoints](https://docs.base.org/base-chain/api-reference/rpc-overview)
   y [`eth_chainId`](https://docs.base.org/base-chain/api-reference/ethereum-json-rpc-api/eth_chainId).
4. Una consulta read-only realizada el 2-sep-2026 devolvió `0x14a34` y bytecode no vacío en
   las dos direcciones oficiales. El test ignorado `base_sepolia_live.rs` reproduce esa
   comprobación sin credenciales.
5. El SDK oficial inspeccionado fue
   [eas-sdk](https://github.com/ethereum-attestation-service/eas-sdk) commit
   `9ebc43fc0345679c8a521a1f497592015b9deb17`, paquete `2.10.0`. Su API construye la
   transacción y solo la emite al ejecutar `wait()`; `SchemaRegistry.getSchemaUID` usa el
   esquema/resolver/revocabilidad comprometidos. Sin embargo, `npm audit --omit=dev` sobre su
   grafo 2.10.0 devolvió 29 avisos (7 altos), en gran parte por Hardhat arrastrado por el
   paquete de contratos, sin arreglo disponible para esa cadena. Por ello no se coloca ese
   SDK en el camino de la clave: el candidato usa `ethers` y las dos ABI oficiales mínimas.
6. El UID calculado para `bytes32 epochRoot`, resolver cero y `revocable=false` es
   `0x1fbe4ca64e41bb8503eafb480385306db0f8d18aa67c152839e4b50cd4325f71`. La consulta
   read-only devolvió `Schema not found`: aún hace falta registrar el esquema antes de la
   primera atestación real.

## Inferencias de diseño (no hechos de la red)

- Consultar contrato y receipt directamente retira la dependencia de un indexador alojado;
  no retira la dependencia operativa del RPC escogido.
- `AnchorRefV0.tx_id` debe conservar el hash real de transacción. Como EAS añade un UID de
  atestación distinto, un sidecar versionado del adaptador evita sobrecargar el campo o tocar
  Protocol v0.
- Esquema sin resolver, atestación no revocable, recipient/refUID/expiration cero y datos de
  exactamente 32 bytes minimizan semántica ajena al único hecho anclado: el `epoch_root`.
- Un backend mock valida código; no constituye custodia. Solo una transacción confirmada y
  verificada abre la compuerta de publicación del Artefacto 1 de S6.

## Trabajo externo no realizado

No se creó ni financió una cuenta, no se registró el esquema y no se envió ninguna
transacción. Hacerlo exige una clave dedicada y gas de testnet, y solo procede tras aceptar la
ADR que fija los bytes y predicados del submission.
