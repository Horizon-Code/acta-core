# Mini-E0 SCITT — compatibilidad medida, no razonada

**Fecha:** 3 de septiembre de 2026
**Naturaleza:** research no normativo. Mide; no acepta SCITT ni lo añade al catálogo de anclaje.
**Motivo:** la compatibilidad SCITT estaba razonada por lectura, no medida, y va a sostener una
decisión de catálogo y material público. Criterio: se mide antes de afirmar nada.

## 0. Corrección de fuente — hallazgo previo a la medición

El encargo citaba **RFC 9942** como base de la compatibilidad SCITT. **RFC 9942 no es la
arquitectura SCITT**: es *CBOR Object Signing and Encryption (COSE) Receipts*, y define los
parámetros de cabecera `receipts` (394), `vds` (395) y `vdp` (396) para pruebas sobre
Verifiable Data Structures. No contiene requisitos de Signed Statement ni menciona los claims
CWT `iss`/`sub`.

La arquitectura SCITT es **RFC 9943**, *An Architecture for Trustworthy and Transparent Digital
Supply Chains*. Toda la medición de abajo se hizo contra RFC 9943; RFC 9942 sigue siendo
relevante, pero para el **recibo** que devuelve el Transparency Service, no para el sobre que se
le envía.

Consecuencia: cualquier material que cite RFC 9942 como fuente de la forma del Signed Statement
debe corregirse antes de publicarse.

## 1. Envolver una raíz de época real como Signed Statement — **PASA**

Se envolvió la raíz real de S6 `bd60c5e6…38fba` (la misma anclada en Base Sepolia) como
`COSE_Sign1` etiquetado, en modo sobre, con los 32 bytes canónicos de la raíz como **payload
opaco** — exactamente los mismos bytes que EAS ancla.

Requisitos normativos de RFC 9943 satisfechos en el protected header:

| Label | Parámetro | Valor puesto |
|---|---|---|
| 1 | `alg` | `-8` (EdDSA), el algoritmo que ACTA ya usa |
| 3 | content type | `application/acta-epoch-root` |
| 4 | `kid` | clave pública Ed25519 (obligatorio si no hay `x5t` ni `x5chain`) |
| 15 | `CWT_Claims` | **obligatorio**, con `iss` (1) y `sub` (2) |

- `iss`: `https://acta.example/issuer/c2-omegaclaw`
- `sub`: `urn:acta:epoch-root:bd60c5e6…38fba`

Sobre resultante: **305 bytes**, SHA-256
`fa8ee3f95445bbf80847086a53154de403753a3dfc511bec6eee9832e6a87522`.

Productor: crate `coset` 0.3.8 (Google), firma Ed25519 con `ed25519-dalek`. Semilla
determinista `[7u8; 32]`: es un ejercicio de medida, no una clave de producción.

Bytes completos, para que la medición sea reproducible sin este scratchpad:

```
d28458c8a4012703781b6170706c69636174696f6e2f616374612d65706f63682d726f6f74045820ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c0fa201782868747470733a2f2f616374612e6578616d706c652f6973737565722f63322d6f6d656761636c617702785475726e3a616374613a65706f63682d726f6f743a62643630633565366662646434323530343733383765396438376631613365326233333037656437313864323239623139313431616664383433323338666261a05820bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba58409c88a4d159451668bf19e0fb2e1355089ef2538f0df47d97cdc85195a4aed9a186b802a71f4972ca6973a471c2dc5df5c3c398857f79c2544713f70e02938b0c
```

## 2. Round-trip byte a byte — **PASA**

El sobre se guardó en un sidecar JSON como base64, se recuperó y se comparó:

| | Valor |
|---|---|
| Longitud original / recuperada | 305 / 305 |
| SHA-256 original / recuperado | `fa8ee3f9…7522` / `fa8ee3f9…7522` |
| Idéntico byte a byte | **sí** |

Tras el round-trip, la librería ajena vuelve a verificar la firma y recupera el payload exacto.

**Matiz medido, que conviene no sobreinterpretar.** Decodificar el sobre y volver a serializarlo
con la *otra* implementación produjo también los mismos 305 bytes. Es buena señal de
interoperabilidad, pero **no autoriza a recomputar**: el protected header se firma como `bstr`,
así que cualquier implementación que reordene el mapa o cambie la codificación de longitudes
rompería la firma. La regla sigue siendo **preservar el sobre, nunca recomputarlo**, igual que
las firmas del receipt FULL y que el sidecar de EAS. La coincidencia observada es una propiedad
de estas dos implementaciones sobre este sobre, no una garantía del formato.

## 3. Verificación por librería ajena — **PASA**, y es el punto que importa

Verificar con código propio habría sido la carpeta hermana otra vez. Se usó un cruce real de
implementaciones:

- **Produce:** `coset` (Rust, Google).
- **Verifica:** `@auth0/cose` + `cbor2` (Node, Auth0/Okta). Lenguaje distinto, autores
  distintos, base de código sin relación.

El verificador **no sabe qué es ACTA**: solo lee COSE y comprueba los requisitos de RFC 9943.

```json
{
  "library": "@auth0/cose + cbor2 (Node)",
  "bytes": 305,
  "alg": -8,
  "content_type": "application/acta-epoch-root",
  "cwt_claims_present": true,
  "iss": "https://acta.example/issuer/c2-omegaclaw",
  "sub": "urn:acta:epoch-root:bd60c5e6…38fba",
  "required_headers_ok": true,
  "signature_valid": true,
  "payload_hex": "bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba"
}
```

La librería ajena recupera la raíz de ACTA **exacta** como payload opaco, sin conocer el
formato.

### Control negativo

Un PASS sin control negativo no dice nada. Dos manipulaciones, dos rechazos:

| Manipulación | Resultado |
|---|---|
| Un byte del payload alterado dentro del sobre | `signature verification failed` |
| Clave pública distinta | `signature verification failed` |

## 4. Política de registro — medida en dos servicios reales

RFC 9943 confirma lo que se sospechaba: **la autenticación del emisor está fuera de alcance**.
«Authentication and authorization are implementation specific and out of scope of the SCITT
architecture». El Transparency Service sí autentica criptográficamente el Signed Statement, pero
*quién puede registrar* lo decide cada operador. La fricción es por servicio, no por estándar.

**DataTrails** (comercial, en preview):

- Requiere **cuenta con suscripción** y credenciales OAuth client-credentials
  (`DATATRAILS_CLIENT_ID` / `DATATRAILS_CLIENT_SECRET`).
- Identidad de emisor como cadena tipo dominio (`sample.synsation.io` en su ejemplo).
- Su quickstart genera la clave con **prime256v1** (ES256).
- La API está declarada **en preview y sujeta a cambio**.

**scitt-ccf-ledger** (Microsoft, open source, autoalojable):

- Política de registro como **script JavaScript o Rego** configurable por el administrador; el
  ejemplo comprueba `phdr.cwt.iss` contra un `did:x509` concreto.
- Algoritmos aceptados por defecto:
  `["ES256","ES384","ES512","PS256","PS384","PS512","EDDSA"]`, sensibles a mayúsculas, y el
  administrador puede sobreescribir la lista.
- Esquemas de identidad citados: `did:x509` y `did:attestedsvc`.

**Hallazgo con consecuencia práctica.** ACTA firma **Ed25519**. scitt-ccf lo acepta por defecto
(`EDDSA` está en la lista); el quickstart de DataTrails usa ES256. Es decir: **el algoritmo de
ACTA no es universalmente seguro entre Transparency Services**, y la lista es configurable por
el administrador. Presupuestar un TS concreto implica comprobar su lista de algoritmos, no
asumirla.

**Y el hallazgo estructural, que es el que pesa:** un Transparency Service es un operador al que
hay que pedir permiso y en quien hay que confiar para el registro. Eso no es un defecto de
SCITT — es su modelo — pero contrasta de forma directa con el anclaje contra sustrato público,
donde no hay a quién pedir permiso. Es exactamente el argumento que S9 necesitará preparado.

## 5. Coste sobre lo construido — medido sobre el código, no estimado

**Core: cero cambios.** `AnchorRefV0` ya es agnóstico de sustrato —
`substrate`/`network`/`tx_id`/`slot`/`epoch_root`— y `tx_id` y `slot` son
`Option`. SCITT puede dejarlos a `None` y llevar sus identificadores en el sidecar. Esto
respeta además la disciplina que ADR-012 §3 ya fijó al negarse a sobrecargar `tx_id` con el UID
de atestación: **el identificador de entrada del TS va al sidecar, no al campo de Core.**

**A1 (`attestation-single-signer`): cero cambios.** Firma receipts y verifica bundles; el sobre
COSE envuelve la **raíz de época**, que está una capa por encima. A1 solo cambiaría si se
quisiera convertir el propio receipt en Signed Statement, que es otro diseño y no éste.

**B1 (`acta-verifier`): cambio acotado pero real.** En
`tools/acta-verifier/rust/src/main.rs:83` la evidencia está **tipada a EAS**:

```rust
let evidence: EasAnchorEvidenceV1 = read_json(Path::new(&path), "anchor evidence");
```

Añadir SCITT exige despachar por `evidence_version` en vez de asumir el tipo. Es el punto
exacto donde el segundo sustrato toca el verificador, y es pequeño.

**ADR-013: aquí está el coste de verdad.** `MachineAnchorV1` lleva campos **específicos de
EAS** —`attestation_uid`, `schema_uid`, `attester`— en una envolvente **ya ratificada**.
Meter SCITT obliga a una de dos: añadir más campos opcionales por sustrato, que degrada la
envolvente a un cajón, o rediseñar a una forma común por sustrato. Cualquiera de las dos es
**una ADR nueva sobre una envolvente ratificada**, no un parche.

Esa es precisamente la decisión que ADR-014 propone resolver una sola vez para los tres
sustratos, antes de que haya dos formas incompatibles que mantener.

## Criterio de cierre

Los cinco puntos con resultado escrito: **1 PASA · 2 PASA · 3 PASA · 4 medido en dos servicios ·
5 medido sobre el código**. Más una corrección de fuente (§0) que afecta a material futuro.

El modo sobre funciona y es interoperable con implementaciones ajenas. Lo que **no** está
resuelto y no se afirma: ningún registro real contra un Transparency Service —eso exige cuenta,
credenciales y, en el caso comercial, suscripción—, y ningún recibo COSE (RFC 9942) recibido ni
verificado. El mini-E0 midió el sobre que se envía, no el recibo que se devuelve.

## Reproducir

Productor y verificador quedaron en scratchpad, fuera del repo, por ser exploratorios. Los bytes
del sobre están transcritos en §1 a propósito: bastan para reproducir la verificación ajena con
cualquier librería COSE, sin depender de este entorno.
