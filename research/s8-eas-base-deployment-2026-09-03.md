# S8 — despliegue real de anclaje EAS en Base Sepolia

**Fecha de ejecución:** 3 de septiembre de 2026
**Naturaleza:** registro de ejecución no normativo. Documenta lo ocurrido; no acepta ni
modifica ADR-012/013, ratificadas el 2-sep-2026 contra sus tablas de hashes completas.
**Autoridad:** operador bajo ADR-001, decisión de alcance de 3-sep-2026.

## 1. Alcance de lo demostrado

Lo cerrado es el **mecanismo de anclaje**, no la custodia del Artefacto 1.

- El mecanismo funciona de punta a punta y es verificable por un tercero contra una atestación
  real en cadena.
- **Testnet demuestra el mecanismo ante un tercero; no es custodia de producción.** Un anclaje
  en red de pruebas prueba que el camino funciona; no equivale a anclaje en mainnet.
- **La clave firmante circuló fuera del canal previsto.** La atestación acredita que el camino
  funciona, pero **no quién firmó**. Coherente con que `TR-SIGNER-SELF` siga puesto.
- **La custodia del Artefacto 1 NO queda cerrada** y la compuerta de E-9.6 sigue abierta: los
  bundles de la corrida real del 1-sep-2026 no son recuperables (ver riesgo en
  `roadmap/risk-register.md`, sección «S8 / demonstration evidence custody»). Sin bundles no
  hay a qué adjuntar la referencia de anclaje que exige ADR-012 §5.

## 2. Transacciones

Cuenta firmante dedicada de testnet: `0xb0d8bd5c0183d72626c65c11a39fcc5c6701aa15`, nonce
inicial 0, financiada desde el faucet de Coinbase Developer Platform.

| Paso | Transacción | Bloque |
|---|---|---|
| Registro de esquema | `0xd2b442d25c12c4ed7f418ec69d660f6ade8f5a5691a5526c1170fd77dcc9d6ab` | 46343470 |
| Atestación de la raíz | `0x63d805480cda9ebaa61e30dcbdfa1db23ac39c3cf7c85b7baae61ed2950a48fc` | 46343480 |

El UID de esquema resultante coincide exactamente con el declarado en ADR-012 §2:
`0x1fbe4ca64e41bb8503eafb480385306db0f8d18aa67c152839e4b50cd4325f71`. `already_registered`
devolvió `false`: ACTA lo registró, no reutilizó uno preexistente.

UID de atestación: `0x3cc34239d3f1f46907e45bdd15082dbe112e58fbf5745546b7642a276696c907`.

Raíz anclada: `bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba`, la raíz de
época de la corrida real de C2 del 1-sep-2026 registrada en
`demos/c2-omegaclaw/artifact1/e0-real-history-run.md`.

## 3. Sidecar `acta.eas-anchor-evidence.v1`

```json
{
  "evidence_version": "acta.eas-anchor-evidence.v1",
  "anchor_ref": {
    "substrate": "eas",
    "network": "eip155:84532",
    "tx_id": "0x63d805480cda9ebaa61e30dcbdfa1db23ac39c3cf7c85b7baae61ed2950a48fc",
    "slot": 46343480,
    "epoch_root": "bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba"
  },
  "eas": {
    "chain_id": 84532,
    "contract_address": "0x4200000000000000000000000000000000000021",
    "schema_registry_address": "0x4200000000000000000000000000000000000020",
    "schema_uid": "0x1fbe4ca64e41bb8503eafb480385306db0f8d18aa67c152839e4b50cd4325f71",
    "attestation_uid": "0x3cc34239d3f1f46907e45bdd15082dbe112e58fbf5745546b7642a276696c907",
    "attester": "0xb0d8bd5c0183d72626c65c11a39fcc5c6701aa15",
    "schema": "bytes32 epochRoot",
    "resolver": "0x0000000000000000000000000000000000000000",
    "revocable": false
  }
}
```

Se transcribe íntegro aquí a propósito: dejarlo solo en almacenamiento efímero repetiría el
fallo registrado en el riesgo de S8.

## 4. Verificación por el camino del tercero

Ejecutada con el verificador Rust contra Ethereum JSON-RPC crudo
(`https://sepolia.base.org`), sin el publicador, sin indexador, sin GraphQL, sin explorador,
sin API de EAS y sin endpoint de ACTA. `EasAnchorBackend::verifier` construye el backend con
`publisher = None`, de modo que el camino de verificación no puede invocar al publicador ni
por accidente. La variable de la clave se retiró explícitamente del entorno del proceso
verificador.

**Predicado de ADR-012 §4: PASS, las siete comprobaciones.**

Matiz declarado: sin los bundles reales de S6 no existe un `AnchorRefV0` independiente contra
el que contrastar el sidecar, así que el verificador tomó el del propio sidecar. Eso deja
**vacua** la comprobación adicional bundle↔sidecar de `validate_evidence_shape`. Esa
comprobación **no es ninguna de las siete**: las siete del predicado son sidecar↔cadena y se
ejecutaron todas de verdad.

### Control negativo

Para descartar que el PASS fuera vacuo se verificaron tres sidecars manipulados. Los tres
fueron rechazados:

| Campo alterado | Resultado |
|---|---|
| `epoch_root` (un byte) | FAIL — `Mismatch("on-chain attestation record")` |
| `attester` | FAIL — `Mismatch("on-chain attestation record")` |
| `slot` (bloque +1) | FAIL — `Mismatch("transaction receipt")` |

## 5. Coste real medido

Base es un L2: el coste total suma la componente de ejecución L2 y la de datos L1.

| Transacción | Gas L2 | Coste L2 (wei) | Coste L1 (wei) | Total ETH | USD |
|---|---|---|---|---|---|
| Registro de esquema | 79.150 | 474.900.000.000 | 5.781.815.189 | 0,000000480682 | 0,0021631 |
| Atestación | 190.201 | 1.141.206.000.000 | 7.048.807.873 | 0,000001148255 | 0,0051671 |
| **Total** | | | | **0,000001628937** | **0,0073302** |

Precio de gas efectivo 0,006 gwei en ambas; USD a 4.500 USD/ETH.

**La estimación de ADR-012 (0,002 USD para las dos transacciones) queda corregida: el coste
medido es 0,0073 USD, factor 3,67×.** La componente L1 es despreciable (0,8% del total); casi
todo es ejecución L2, y la atestación cuesta 2,4× el registro. Sigue siendo ruido económico,
pero la cifra que se publique debe ser la medida.

Saldo restante de la cuenta: 0,000298371 ETH.

## 6. Estado tras la ejecución

- S8: el mecanismo de anclaje queda demostrado y verificado por el camino del tercero.
- Compuerta de custodia de E-9.6: **sigue abierta**.
- Notas de no-publicabilidad del Artefacto 1 en `docs/agent-context/S6_C2_HANDOFF.md`,
  `docs/agent-context/S7_HANDOFF.md` y `roadmap/current-state.md`: **intactas, sin tocar**.
- El anclaje publicado sigue siendo válido para la raíz que ancla. Si la corrida real se
  recupera o se regenera con la misma raíz, `attach` cierra el §4 sin republicar; si produce
  otra raíz, se ancla esa.
