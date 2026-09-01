# ACTA — Estado consolidado

> **Nota de incorporación.** Este documento es el mapa de *qué hay*, redactado la mañana del
> 31-ago-2026, **antes** de la `Directiva de construcción` y de los tres intercambios que
> produjeron `roadmap/E0-protocolo-y-enmiendas.md`. Se incorpora íntegro y literal. Las
> marcas `[Enmendado…]` en blockquote señalan los puntos superados ese mismo día; donde este
> texto y las enmiendas E-1..E-8 difieran, mandan las enmiendas. El cuerpo no está alterado.
> Resuelve las dos referencias cruzadas pendientes: la **escala de vocabulario** vive en su
> §1.2 y las **decisiones de canonicalización** (incluida la decisión 3 que cita E-7) en su
> §3.1.

> **[Enmendado por E-9 — Accepted 31-ago-2026]** La postura estratégica vigente es EVM/Base
> first con multi-anclaje, Cardano en catálogo, `agent_commerce` como siguiente perfil
> comercial y OpenWorker/OmegaClaw como suelo público/techo técnico. AML se conserva como
> referencia interna. La enmienda completa, ratificada dentro del paquete E-9 + E-1 + E-2 +
> E-6 sobre el commit `503aefbd0636ba3ef52b782c675b1a901e27466a`, está en
> `roadmap/E-9-reposicionamiento-acta-economia-agentes.md`.

**Fecha:** 31 de agosto de 2026
**Tipo:** mapa interno de trabajo
**Autoridad:** subordinado a `constitution/ACTA_Foundations_v1.2_consolidado.pdf`

> Este documento no es normativo. Consolida en un solo sitio qué es ACTA, qué está construido, qué está decidido pero no escrito, y qué sigue abierto. Donde hay discrepancia con las Foundations o con `protocol/ACTA_Protocol_v0.md`, mandan esos.

---

## 1. Qué es ACTA

**El problema.** En los sistemas automatizados con impacto real, quien controla la infraestructura controla también la narrativa de lo ocurrido: puede reconstruir los logs, elegir qué conservar y presentar una versión favorable del proceso.

**La tesis.** ACTA crea registros constituidos en el momento del acto —canónicos, secuenciados, atestiguables, portables y anclables— que reducen el espacio donde un operador puede reescribir u ocultar actuaciones sin dejar señal.

**El límite honesto.** ACTA no hace responsables a los sistemas. Hace que la evidencia para exigirles responsabilidad exista y no pueda ser silenciada. La responsabilidad la ejercen instituciones; ACTA garantiza que no puedan ignorar la evidencia.

**La formulación operativa.** ACTA no es un tribunal. Verifica si las partes cumplieron lo que ellas mismas definieron como justo. Nunca produce veredictos.

### 1.1 La distinción que ordena todo

| | |
|---|---|
| **Verificabilidad** | Gradual, acumulativa, técnica. Es lo que ACTA produce. |
| **Responsabilidad** | Institucional, externa, no técnica. Es lo que otros ejercen sobre esa evidencia. |

Confundirlas es el principal riesgo de comunicación del proyecto.

### 1.2 Cronología del vocabulario

El término "notarial" solo es honesto en la última fase. Escala propuesta:

1. **v0 — paquete probatorio portable.** Forma + secuencia + prueba de inclusión. *(Estado actual.)*
2. **v2+ — dossier atestiguado.** Cuando hay firma independiente del productor.
3. **v5+ — registro anclado externamente.** Cuando el `epoch_root` se verifica contra infraestructura externa.
4. **Madurez — fe computacional delegada.** Attestors independientes + resolución de identidad + commitments abribles + perfiles normativos + reconocimiento institucional.

Usar "acta notarial digital" para describir un `BundleV0` hoy sería un error ante un auditor o regulador.

> **[Enmendado por Directiva §1.1 y E-3]** La escala se reinterpreta como **escala de
> vulnerabilidades retiradas**: cada fase elimina una línea del registro de *condiciones
> estructurales de la versión* del informe de confianza residual. Ver
> `roadmap/E0-protocolo-y-enmiendas.md` (E-3).

---

## 2. Arquitectura: cinco jurisdicciones

El principio fundacional es que el Core define **qué es un hecho computacional, cómo se representa y cómo se verifica**. No define quién gobierna, quién valida, cómo se incentiva ni qué token existe.

| Capa | Responsabilidad | Ubicación |
|---|---|---|
| **Core** | Primitivas y verificación estructural | `core/rust/acta-core/` |
| **Protocol** | Formatos interoperables y versionado | `protocol/ACTA_Protocol_v0.md` |
| **Profiles** | Semántica de dominio y reglas de ciclo de vida | `profiles/aml/rust` |
| **Adapters** | Ledgers externos, identidad, almacenamiento, registros, sistemas de prueba | `adapters/cardano-anchor/` |
| **Instituciones** | Adjudicación, ejecución, sanciones, remedios | Fuera del repo |

**Regla de asignación** (de `architecture/foundations-to-modules.md`):

- Si valida bytes/hash/forma → Core.
- Si valida semántica de dominio → Profile.
- Si necesita consulta externa → Adapter.
- Si decide consecuencia → Institución/Auditor.
- Si afecta a los bytes canónicos o al hash → debe versionarse.
- Si hace constitutivo un sustrato concreto → se rechaza.

### 2.1 Qué puede y qué no puede responder el Core

**Puede:** ¿es este evento estructuralmente válido? ¿está canónicamente codificado? ¿coincide este hash con este evento? ¿este receipt liga a este evento y a esta referencia Chronos? ¿este bundle liga internamente evento, receipt, prueba Merkle, epoch root y anchor opcional? ¿esta secuencia local encadena bien? ¿este commitment es sintácticamente válido?

**No puede:** ¿fue justa la decisión? ¿se cumplió la ley? ¿estaba el actor institucionalmente autorizado? ¿debe congelarse la cuenta? ¿debe sancionarse a alguien? ¿incluyó Cardano realmente esa tx? ¿era materialmente correcta la política AML?

### 2.2 Los tres planos que no deben fundirse

1. **El hecho** (`ActaEventV0`) responde *qué se afirma*.
2. **La atestación** (`ReceiptV0`) responde *quién lo atestigua*.
3. **El anclaje** (`AnchorRefV0`) responde *desde cuándo no puede reescribirse sin contradecir una referencia externa*.

Un evento no es verdadero por estar anclado. Una firma no prueba independencia si el firmante es el mismo operador. Un anclaje externo no valida la semántica del contenido.

---

## 3. Modelo de objetos v0

| Objeto | Qué liga | Qué no prueba |
|---|---|---|
| `ActaEventV0` | protocolo, id, `issued_at`, `process_ref`, `event_kind`, `commitments`, `policy_snapshot`, `actor_ref` | corrección legal, autorización institucional, verdad de dominio |
| `ProcessRefV0` | `process_id`, `process_type` | que el tipo esté permitido por un perfil |
| `EventKindRefV0` | `namespace`, `kind`, `version` | corrección semántica del kind |
| `ActorRefV0` | `actor_id`, `actor_type` | identidad real, resolución DID, autoridad |
| `PolicySnapshotV0` | `policy_id`, `policy_hash`, `policy_type`, `jurisdiction`, `effective_from`, `effective_to?` | validez legal ni exigibilidad actual |
| `CommitmentsV0` | inputs, outputs, artifact | que lo comprometido sea verdadero, lícito o completo |
| `ChronosRefV0` | `epoch_id`, `prev_event_hash?` | anti-omisión global por sí solo |
| `ReceiptV0` | `event_hash`, `chronos_ref`, `issued_at`, `signatures` | validez criptográfica de la firma, identidad del attestor |
| `MerkleProofV0` | `leaf_index` + siblings direccionales | verdad del anclaje externo |
| `BundleV0` | todo lo anterior + `epoch_root` + `anchor?` | inclusión real en ledger externo |
| `AnchorRefV0` | `substrate`, `network?`, `tx_id?`, `slot?`, `epoch_root` | existencia de la tx, validez del slot |

> **[Enmendado por Directiva §1.1]** La columna "Qué no prueba" pasa de mapa de carencias a
> **esquema de salida del informe de confianza residual**: cada carencia es una condición
> nombrada y acotada que el verificador sabe detectar (Directiva §3, ampliada por E-1).

### 3.1 Las cuatro decisiones técnicas que sostienen el protocolo

**Canonicalización por arrays CBOR posicionales, no mapas.** Los mapas CBOR o JSON no son canónicos entre implementaciones: distintos lenguajes ordenan claves de forma distinta. Arrays posicionales resuelven el problema de raíz. Es lo que permite que un verificador escrito en Python o Go recompute el mismo `event_hash` años después. Es la garantía más importante del protocolo.

**Separación body/full en el receipt.** El attestor firma el cuerpo (`event_hash` + `chronos_ref` + `issued_at`), no el receipt completo. Evita la circularidad de firmar las propias firmas y permite multi-firma sin cambiar el payload — la evolución a N-of-M no rompe v0. El test `receipt_body_hash_does_not_depend_on_signatures` lo fija.

**Hojas Merkle en orden, sin ordenar.** Ordenar destruiría la propiedad anti-omisión: un atacante podría omitir eventos del medio y reordenar los restantes para obtener la misma raíz. El orden posicional hace que la raíz dependa de la secuencia completa, no del conjunto.

**`ActaEventV0` separado de `ChronosStampedEventV0`.** El `event_hash` no incluye `prev_event_hash`. El hecho computacional existe con independencia de su posición; la posición es atributo del proceso. Lo que el receipt ancla bajo firma es el par (hecho, posición).

> **[Complementado por E-7]** La decisión 3 (hojas en orden) forma par con su regla inversa
> para conjuntos de resultados no deterministas, que **sí** se normalizan antes de
> comprometer. Hojas: el orden es la evidencia. Conjuntos: el orden es ruido del motor.
> Ver E-7 en `roadmap/E0-protocolo-y-enmiendas.md`; la regla del par es capa Profile.

### 3.2 Formato de commitment v0

Formato exigido: `sha256:<64 caracteres hex minúscula>`.

Rechaza: placeholders sin prefijo, digests de longitud incorrecta, no-hex, mayúsculas, espacios circundantes, y en particular `sha256:manual_review_inputs`.

> **Nota de estado.** Esto cierra la tensión más señalada del análisis de abril de 2026, que apuntaba que el protocolo aceptaba cualquier string como commitment. Ya no. El commitment liga integridad, no verdad material — pero al menos ahora es sintácticamente un hash.

---

## 4. Flujos

**Emisión (productor):**
```
Evento → Canonicalizar → Hash → Crear Receipt → Firmar → Validar forma → Agrupar en epoch → Anclar
```

**Verificación (auditor):**
```
Recuperar Evento+Receipt → Rehashear evento → Validar forma del receipt →
Verificar firmas → Verificar cadena Chronos → Verificar prueba Merkle
```

Reparto: `acta-core` valida forma, determinismo y hashing. Los módulos resuelven verificación de firmas (requiere resolución de claves, fuera del core). Los servicios cubren red, almacenamiento y gobernanza.

---

## 5. Estado real, fase por fase

Estado de las 10 fases del MVP definidas en `roadmap/acta-mvp-fases-tecnicas.txt`, cruzado con `docs/core-v0-alpha-readiness.md`.

| Fase | Objetivo | Estado |
|---|---|---|
| **0** — Congelación del protocolo | Definir y congelar ACTA v0 | **Cerrada.** Spec en `protocol/ACTA_Protocol_v0.md` (draft v0). Todos los tipos definidos. |
| **1** — `acta-core` | Núcleo criptográfico y verificador puro | **Cerrada.** Canonicalización, hashing validado, Chronos, Merkle posicional, bundle, vectores de test de protocolo, test e2e. `cargo fmt` y `cargo test --workspace` pasan. |
| **2** — Attestation single-signer | Firmar receipts y verificar firmas | **Parcial.** Las primitivas de receipt y el split de validación de body están cerrados en Core. La verificación criptográfica vive fuera del Core y no consta implementada. |
| **3** — Recorder + Chronos (servicio) | API, persistencia, epoch en curso | **No iniciada.** No hay servicio. Chronos existe como primitiva local, no como servicio. |
| **4** — Epoch builder | Agrupar, Merkle, epoch_root, pruebas | **Cerrada en local.** Local Epoch Builder existe y está en el checklist go/no-go. |
| **5** — Anchor module (Cardano) | Publicar epoch_root en Cardano | **No iniciada.** `AnchorRefV0` existe como referencia neutra respecto al sustrato y el Core valida su forma. `adapters/cardano-anchor/` es un README placeholder. Readiness lo lista explícitamente: *Cardano adapter absent*. |
| **6** — Verificador independiente | CLI/lib verificable por terceros | **Parcial.** La verificación de bundle funciona dentro del Core. El "Local Verification Report v0" está marcado como capa explicativa post-freeze. No hay CLI ni informe para humano/regulador/auditor. |
| **7** — Policy reference | Vincular hechos a obligaciones previas | **Parcial y transitoria.** `PolicySnapshotV0` existe inline, pero `policy_hash` solo se comprueba no-vacío. `policy_commitment` y `NormativeRefV0` no implementados. |
| **8** — Demo probatoria | Caso ancla end-to-end con UI | **No iniciada.** El ejemplo AML vive temporalmente bajo `acta-core/examples`. Demo de AI Agents ausente. UI/producto ausente. |
| **9** — Hardening | Claves, observabilidad, Docker, CI | **No iniciada.** |

> **[Enmendado — estado posterior el mismo día]** Fase 6 avanzó: `report.rs` incorpora la
> **estructura** del informe (partición en dos registros de E-3 e invariante de
> códigos-como-datos de la regla de extensión del §3 de la Directiva), sin códigos `TR-*`
> todavía — cada código entra con la funcionalidad que hace su condición detectable. Fase 2
> (A1) y la resolución de claves (A3) quedan como siguientes; el adaptador Cardano (A2)
> queda además condicionado a las tres conversaciones de validación (Directiva §9).

### 5.1 Titular

**Core v0-alpha está en freeze candidato y el checklist go/no-go está completo.** Todo lo que hay por encima del Core —servicio, anclaje real, verificador de terceros, demo— está por construir.

Lo que ACTA demuestra hoy: verificabilidad local por un tercero del flujo `evento → hash → receipt → epoch → prueba Merkle → bundle → verificar`. Nada más y nada menos.

### 5.2 Deuda aceptada (declarada, no bloqueante)

Perfil AML no es crate independiente. Ejemplo AML temporalmente en `acta-core/examples`. Falta informe para humano/regulador/auditor. `account_released` / `process_closed` / `transfer_flagged` incompletos. `policy_hash` transitorio. Sin `NormativeRefV0`. Sin adapter Cardano ni Midnight. Sin DID/TAP real. Sin ZK ni disclosure selectivo. Sin verificación de almacenamiento externo. Sin demo de agentes. Sin UI.

> **[Enmendado por Directiva §1.1]** Esta lista deja de ser deuda pendiente y pasa a ser
> parte del **esquema de salida del informe**: categorías de vulnerabilidad que el
> verificador detecta y nombra.

### 5.3 Prohibido en el Core

Semántica AML. Semántica de agentes IA. Cardano/Midnight/DID/ZK reales. Verificación de almacenamiento externo. Juicio legal, sanciones, ejecución institucional, gobernanza, tokenomics, mercado de disputas, lógica de adopción o marketing.

---

## 6. Decisiones cerradas que NO están en el repo

Todo lo siguiente se ha resuelto en conversación y no tiene reflejo documental. **Es la brecha principal entre lo pensado y lo escrito.**

### 6.1 Los dos niveles forenses

**Nivel 1 — forense de forma.** Chronos detecta el borrado silencioso de eventos emitidos. Aplicable a cualquier agente, incluidos LLMs. Es lo que el Core ya hace.

**Nivel 2 — forense de contenido.** El concepto de *testigo* captura el subgrafo mínimo de premisas, reglas y valores de verdad de sistemas reproducibles y deterministas (Hyperon/PLN). Permite recomputación por un tercero y convierte la omisión en **perjurio falsable**.

**Fuera de alcance:** el tercer nivel, "lo no pensado" (hipótesis nunca exploradas). Explícitamente descartado.

> **[Enmendado por E-5]** El nivel 2 se reformula por **reproducibilidad, no por
> transporte**: con (premisas literales, lockfile del ruleset, motor+versión, conclusión) un
> tercero reejecuta y comprueba; la identidad de la regla se recupera por recomputación. El
> alcance exacto sobre el sistema objetivo: **un salto es recomputable; una cadena es un
> conjunto de saltos individualmente recomputables unidos por eslabones mediados por el
> LLM** (`TR-CHAIN-MEDIATED`, con partición fiel/libre por eslabón — E-2). El AtomSpace del
> sistema objetivo se destruye por llamada.

### 6.2 Cognitive Forensics Profile

Event kinds ordenados por fuerza de la afirmación forense, incluyendo `ai_agent.tool_call_executed` y `ai_agent.significant_action_executed`.

> Nota de coherencia: el readiness prohíbe semántica de agentes IA **en el Core**. Este perfil es capa Profile, no Core. La prohibición no lo bloquea.

### 6.3 Modelo de cuatro piezas

```
Norma → Evidencia (ACTA) → Adjudicación → Ejecución (smart contract)
```

El smart contract **ejecuta** veredictos; nunca los produce.

### 6.4 Tres tipos de disputa

| Tipo | Automatización |
|---|---|
| **De forma** | Totalmente automatizable |
| **De proceso** | Parcialmente automatizable contra una norma comprometida |
| **Sustantiva** | Requiere adjudicador humano; ACTA aporta el mejor dossier posible |

### 6.5 Modo señal pre-acto

ACTA avisa antes de una acción si cae fuera de una referencia normativa verificable, pero **nunca bloquea**. El operador configura la señal como informativa, confirmatoria o bloqueante. *ACTA es el semáforo, no el volante.*

### 6.6 Constancia notarial multinormativa

Para interacciones de agentes entre jurisdicciones, ACTA refleja la interacción contra todos los marcos relevantes simultáneamente. El acto de reflejar constituye adhesión sin imposición.

> **[Enmendado por E-9.1]** En `agent_commerce`, esta idea se relee como multi-mandato. Como
> `ActaEventV0` porta un solo `policy_snapshot`, el perfil podrá comprometer un manifiesto
> canónico de N mandatos mediante un único `policy_hash`, o usar eventos separados. El formato
> exacto no queda decidido aquí: requiere la ADR del perfil antes de implementarse.

### 6.7 Postura de sustrato

Cardano-first, con arquitectura de adaptador (`AnchorBackend`) que hace de cualquier cadena un backend enchufable sin tocar el núcleo. El adaptador ASI:Chain se difiere: ASI:Chain está en DevNet/TestNet, con mainnet apuntado a finales de 2026–2027.

> **[Enmendado por E-9.4 — postura vigente]** EVM/Base-first para el primer comprador, con
> EAS como vía preferente por especificar; multi-anclaje por diseño; Cardano y ASI:Chain en el
> catálogo. Ninguna cadena se vuelve constitutiva. `BundleV0` sigue portando un solo
> `AnchorRefV0`: el multi-anclaje v0 usa bundles equivalentes o referencias externas al mismo
> `epoch_root` hasta que una ADR decida cualquier formato nuevo.

---

## 7. Contexto de ecosistema

### 7.1 Por qué Hyperon importa

Los cuatro análisis del ecosistema (`analisis-hyperon-experimental`, `analisis-mork`, `analisis-petta`, `analisis-pln-chaining`) sostienen una proposición concreta:

**Hyperon produce artefactos forenses pero no los preserva.** PLN genera `EvidentialTruthValue` (valor de verdad atado al conjunto concreto de evidencias que lo sostienen), proof-trees, propagación de confianza con erosión formalizada, y condiciones de consistencia que se niegan a deducir de premisas incoherentes. Pero el Space se reescribe constantemente.

**ACTA llena exactamente ese hueco: Hyperon produce, ACTA preserva, un verificador separado recomputa.**

Lectura de madurez del ecosistema: el sustrato (hyperon-experimental, MORK, PeTTa) está bien construido pero es pre-alfa con TODOs en la semántica central. La capa cognitiva (PLN, chaining) es la más temprana de todo: todo bajo `experimental/`, port en curso del PLN clásico de OpenCog a MeTTa. *Fundación sólida, pisos superiores en obra.*

> **[Enmendado por E-5 y Directiva §5]** Verificación de agosto sobre el sistema objetivo
> (OmegaClaw-Core): lo anterior es cierto del ecosistema y **falso del sistema en
> producción** — su `lib_pln` de 309 líneas no contiene maquinaria de stamps, y el AtomSpace
> no se reescribe: **se destruye al terminar cada inferencia**. Los cuatro análisis son de
> abril y no cubren OmegaClaw; el §5 de la Directiva es la actualización.

### 7.2 Actores

**ASI Alliance** (SingularityNET + Fetch.ai + Ocean Protocol, $FET/$ASI, Ben Goertzel). **Cardano/IOHK** (Charles Hoskinson; socio histórico de migración de SingularityNET). **Hyperon/OpenCog** (MeTTa, PLN, MORK, Petta).

### 7.3 Demos

Proyecto compañero: red/IA descentralizada que mantiene las leyes del mundo actualizadas y verificables, consumible por ACTA de forma opcional. Aragón como primer perímetro medible. Comparte el ADN de separación de planos: *quién construye* (votable) ≠ *qué es verdad* (no votable, validado contra fuente oficial) ≠ *quién paga* (si la verdad pasa).

No tiene representación en el repositorio de ACTA. Decisión pendiente sobre si comparten una constitución ligera o integración estructural más pesada.

> **[Enmendado por E-9.6]** El suelo público pasa a OpenWorker para forense de forma y el
> techo técnico permanece en OmegaClaw para recomputación. AML deja la comunicación externa y
> queda como perfil de referencia interna. Una demo de borrado exige un compromiso retenido
> fuera del control del operador para que la rotura sea probatoria ante terceros.

---

## 8. Lo que sigue abierto

### 8.1 Preguntas técnicas sin resolver

**Regla de frontera de epoch en Chronos.** ¿Puede el primer evento de un epoch apuntar al último del anterior, o cada epoch abre su propia cadena génesis? Sin especificar.

**Modelo de confianza del verificador independiente.** Para verificar sin confiar en ACTA hacen falta tres cosas: el bundle (resuelto), resolución `attestor_id → clave pública` (**no resuelto**) y verificación `epoch_root ∈ Cardano[slot]` (no implementada). Si la resolución de claves depende de un endpoint de ACTA, la independencia del verificador es ficticia. Decisión pendiente: ¿claves inline en el bundle, o registro público con garantías de disponibilidad independiente?

**Asimetría del single-signer.** Si el attestor es el mismo sistema que produce el evento, el receipt no añade valor probatorio externo: es la misma entidad atestiguando sus propios actos. Debe documentarse como limitación explícita del MVP.

**Coherencia temporal.** `issued_at` es una afirmación del productor. Chronos garantiza orden relativo, no absoluto. Un `account_frozen` con `issued_at` anterior al `aml_scored` que lo causó pasa la validación de cadena siendo temporalmente incoherente. Requiere reglas de perfil o un módulo de validación temporal.

**Fragmentación entre perfiles.** Si `aml/process_closed` y `credit/process_closed` tienen semánticas incompatibles, el Core no lo detecta. La solución no es meter semántica en el Core, sino un **contrato de compatibilidad** documental que todo perfil registrado deba respetar.

> **[Enmendado por Directiva §1.1, A3 y §3]** Esta lista es hoy, en parte, esquema del
> informe: claves → **resuelto** (inline en bundle para v0, Directiva A3, con
> `TR-KEY-SELF-ASSERTED` como línea de la limitación); single-signer → `TR-SIGNER-SELF`;
> coherencia temporal → `TR-TIME-DECLARED` / `TR-TIME-INCOHERENT`; anclaje no comprobado →
> `TR-ANCHOR-UNVERIFIED`. Siguen abiertas sin código asociado: la frontera de epoch en
> Chronos y el contrato de compatibilidad de perfiles.

### 8.2 Decisiones abiertas de Demos

Prevención de colusión (aportador ≠ validador sobre la misma norma). Recompensas de mantenimiento de vigencia, para evitar el "verde caducado".

### 8.3 Trabajo documental pendiente

1. Llevar al repo las decisiones de la sección 6. Ninguna está escrita.
2. Definir el Cognitive Forensics Profile como perfil formal bajo `profiles/`.
3. Escribir el contrato de compatibilidad de perfiles.
4. Reclasificar `research/imported-notes/analisis-filosofico-claude-codex.md`: los 13 pilares consolidados merecen ser normativos, no research.
5. Convertir `roadmap/acta-mvp-fases-tecnicas.txt` en `.md` y añadirle columna de estado.

> **[Enmendado — estado posterior el mismo día]** El punto 1 avanzó: la sección 6 tiene ya
> reflejo documental parcial vía `roadmap/directiva-construccion-2026-08-31.md` (§6 de
> deuda documental) y `roadmap/E0-protocolo-y-enmiendas.md`. El estado por ítem lo lleva
> `next-milestones.md`, no este documento.

---

## 9. Los trece pilares

Formulación consolidada tras el intercambio Claude ↔ Codex de abril de 2026.

| Pilar | Formulación |
|---|---|
| Evidencia constituida | Lo que nace con el acto pesa más que la narrativa ex post |
| Tiempo procesal | La posición verificable en la secuencia es más robusta que el timestamp declarado |
| Verificabilidad acumulada | ACTA suma capas de prueba; la responsabilidad la ejercen instituciones |
| Confianza explícita | Toda dependencia de confianza debe quedar nombrada y acotada |
| Registro constituido portable | La evidencia debe poder salir del sistema que la produjo |
| Core austero | El núcleo verifica gramática probatoria, no significado de dominio |
| Perfil gobernado | Cada dominio debe declarar semántica, cobertura, transiciones y obligaciones |
| Compromiso sin exposición | La evidencia puede quedar fijada sin revelación total inmediata |
| Regla ligada al acto | La política aplicable forma parte de la inteligibilidad del evento |
| Anti-omisión | Antes de ver todo, hay que impedir que lo inscrito desaparezca sin señal |
| Completitud gobernada | ACTA protege lo emitido; la obligación de emitir todo viene de fuera |
| Planos separados | Hecho, atestación y anclaje responden preguntas distintas |
| Cierre dual | ACTA cierra criptográficamente para habilitar cierre normativo |

---

## 10. Mapa de documentos del repositorio

| Ruta | Qué contiene | Naturaleza |
|---|---|---|
| `constitution/ACTA_Foundations_v1.2_consolidado.pdf` | Referencia canónica | **Normativo** |
| `protocol/ACTA_Protocol_v0.md` | Formato técnico y reglas de verificación v0 | Draft v0 |
| `architecture/core-boundaries.md` | Qué puede y no puede responder el Core | Arquitectura |
| `architecture/foundations-to-modules.md` | Mapeo jurisdicciones → módulos, con TODOs | Arquitectura |
| `docs/core-v0-alpha-readiness.md` | Estado de freeze, deuda, checklist go/no-go | **Estado** |
| `roadmap/acta-mvp-fases-tecnicas.txt` | Las 10 fases del MVP | Plan, sin estado |
| `roadmap/acta-roadmap-mejoras-futuras.md` | HAP, firma humana, context bundle, disputa, ZK | Post-MVP |
| `research/imported-notes/analisis-filosofico-claude-codex.md` | Análisis filosófico y técnico + 13 pilares | Research (debería promoverse) |
| `research/imported-notes/roadmap-analisis-flujo-acta.txt` | Recorrido del pipeline archivo por archivo | Research |
| `.github/copilot-instructions.md` | Arquitectura esencial y convenciones | Operativo |
| `.github/copilot-instructions-aml-process.md` | Perfil AML y su Definition of Done | Operativo |
| `analisis-hyperon-experimental.md` | El sustrato: motor de reescritura de hipergrafos | Ecosistema |
| `analisis-mork.md` | Rendimiento del sustrato | Ecosistema |
| `analisis-petta.md` | Sustrato | Ecosistema |
| `analisis-pln-chaining.md` | Capa cognitiva: PLN, ETV, proof-trees, chaining | Ecosistema |
| `adapters/cardano-anchor/README.md` | Placeholder | Vacío |

> **[Enmendado — estado posterior el mismo día]** Este mapa es anterior a la incorporación
> de `roadmap/directiva-construccion-2026-08-31.md`, `roadmap/E0-protocolo-y-enmiendas.md`,
> `research/E0-resultados.md`, `roadmap/next-milestones.md`, `roadmap/risk-register.md`,
> `research/open-questions.md` y `docs/local-build-environment.md`. Los índices `README.md`
> de cada directorio son la fuente autoritativa del mapa actual.

---

## 11. Resumen en una página

**Qué es.** Infraestructura probatoria que convierte actuaciones técnicas en evidencia portable, secuenciada y verificable por terceros.

**Qué no es.** Un tribunal, una máquina de verdad, un sistema de explicabilidad, una autoridad.

**Dónde está.** Core v0-alpha en freeze candidato: el flujo completo `evento → hash → receipt → epoch → Merkle → bundle → verificar` funciona y es verificable localmente por un tercero. Encima del Core no hay nada construido: sin servicio recorder, sin anclaje real a Cardano, sin verificador CLI, sin demo.

**Qué lo bloquea.** Nada técnicamente. La ruta crítica es Fase 5 (anclaje Cardano) + Fase 6 (verificador independiente), porque juntas son las que convierten "integridad estructural interna" en "fe notarial externa" — el salto que la propuesta de valor necesita para ser demostrable, no solo argumentable.

**Cuál es el riesgo principal.** No es técnico, es de comunicación: presentar ACTA v0 como si la responsabilidad computacional plena ya estuviera implementada. La formulación honesta es que ACTA v0 **hace que mentir sea más costoso y detectable**; las fases siguientes hacen que sea **imposible mentir de forma no detectable**. Esa progresión gradual, documentada con precisión, es la propuesta de valor real.

**Cuál es la brecha más grande.** Entre lo decidido y lo escrito. Los dos niveles forenses, el Cognitive Forensics Profile, el modelo de cuatro piezas, los tres tipos de disputa, la señal pre-acto y la constancia multinormativa son las decisiones estratégicamente más valiosas del proyecto y ninguna existe en el repositorio.

> **[Enmendado — estado posterior el mismo día]** El §9 de la Directiva añade el riesgo que
> este resumen no recoge: **no consta comprador**. Las tres conversaciones de validación con
> quien firma cumplimiento en banca o sanidad van en paralelo al Bloque A y condicionan A2.

> **[Enmendado por E-9.8]** Las conversaciones vigentes se dirigen primero a un constructor
> sobre x402/AP2 y a un operador de agentes con dinero real en juego; cumplimiento queda como
> calibración opcional del mercado de 2027. A2 conserva la compuerta “adaptador real cuando
> exista contraparte real”, ahora con el sustrato del primer comprador. El informe de mercado
> del 31-ago sigue pendiente de entrega e incorporación bajo `research/`.
