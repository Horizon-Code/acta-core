# E-9 — Reposicionamiento: ACTA como capa de confianza de la economía entre agentes

**Fecha:** 31 de agosto de 2026
**Tipo:** enmienda estratégica a la Directiva de construcción
**Estado:** *Proposed — pendiente de ratificación del operador (ADR-001)*
**Autoridad:** enmienda a `roadmap/directiva-construccion-2026-08-31.md` (§1.3, §2, §4, §9 y
la postura de sustrato del Estado consolidado §6.7). Subordinada a las Foundations y al
Protocol. **No toca el Core, el Protocol v0, ni ninguna ADR aceptada (003–006).**

> Regla de entrada en vigor: mientras esta enmienda esté *Proposed*, la postura vigente en el
> repo sigue siendo la escrita (Cardano-first, AML como suelo comercial, §9 con comprador de
> cumplimiento). La congelación de Cardano/agentes/EVM comunicada al ejecutor se levanta
> **solo** con la ratificación de este documento.

---

## 0. Por qué (base de evidencia)

La validación del §9 de la Directiva se ejecutó en dos vías: investigación de señales duras
observables (desk research multi-fuente, 31-ago-2026) y el guion de conversaciones (en
`research/guion-validacion-tres-conversaciones.md`, pendiente de ejecutarse con interlocutores
redefinidos por esta enmienda). Los hallazgos que motivan el giro:

1. **El comprador de cumplimiento compra atestación, no verificabilidad.** El mercado de AI
   governance/assurance es real y creciente (~$3,1bn TRiSM 2025; £1,01bn UK), pero el gasto
   compra el modelo "logs del operador + procedimiento firmado". Los supervisores (EBA, BCE
   jul-2025, FDA) exigen "trazabilidad" y la aceptan auto-atestada. Ninguna sanción ha citado
   jamás la integridad del log como el fallo.
2. **La propuesta exacta de ACTA ya se probó en ese mercado y quedó en nicho.** Guardtime
   vende verificabilidad-sin-confiar-en-el-operador a banca y sanidad desde 2012; catorce
   años después sigue siendo nicho. Amazon cerró QLDB (su ledger criptográficamente
   verificable) en 2025, presumiblemente por baja demanda.
3. **El disparador del mercado de cumplimiento es externo y llega en 2027**: que un
   estándar armonizado (CEN-CENELEC JTC21 definiendo "logging" del AI Act) o un supervisor
   exija integridad verificable por terceros. Obligaciones de logging de alto riesgo:
   2-dic-2027 (Anexo III) / 2-ago-2028 (Anexo I).
4. **En la economía de agentes la distinción de ACTA no es prima: es prerequisito.** El
   stack en formación (Google AP2 con mandates firmados como W3C Verifiable Credentials;
   Coinbase x402 con Visa, AWS, Anthropic, Stripe; Visa TAP; PayPal Agent Ready) resuelve
   **autorización** (el humano consintió) y **liquidación** (el dinero se movió, final y sin
   chargebacks). No resuelve **ejecución probada**: qué hizo el agente entre el mandato y el
   cobro. Sin chargebacks, la evidencia de ejecución es el único mecanismo de disputa que
   queda. Y entre agentes no hay operador humano cuya palabra aceptar: la verificabilidad
   sin confianza deja de ser lujo.
5. **La demanda de la categoría "evidencia de agente" ya es visible del lado mainstream:**
   OpenWorker (Andrew Ng, 17k⭐, MIT) vende como titular "governance is the architecture" y
   un audit trail con procedencia de aprobación por tool call — implementado como registro
   local auto-atestado, sin encadenamiento ni anclaje. El mercado quiere el cuaderno; nadie
   vende el candado.

**Tesis de la enmienda:** ACTA se posiciona como **la capa de evidencia y disputa de la
economía entre agentes** — la tercera pieza del stack (autorización → **ejecución probada**
→ liquidación) — con el cumplimiento humano como segundo mercado que madura en 2027 con
disparador supervisor. Esto no es un cambio de producto: la arquitectura ya estaba apuntada
aquí (separación de planos, verificación sin confianza, señal pre-acto, modelo de cuatro
piezas, asunción fundacional de la economía de agentes). Es un cambio de **primer comprador
y de orden de construcción**.

---

## E-9.1 — Perfil `agent_commerce` (capa Profile, nueva)

Perfil de dominio para transacciones entre agentes, donde el mandato de autorización **es**
la norma comprometida:

- El Mandate de AP2 (o equivalente firmado) se compromete como `PolicySnapshotV0`:
  `policy_type: "agent_mandate"`, `policy_hash` = hash del mandate. La evidencia de ACTA se
  produce *contra* ese mandato — el modelo de cuatro piezas hecho literal:
  Norma (mandate) → Evidencia (ACTA) → Adjudicación → Ejecución (liquidación o su reversa).
- Event kinds del ciclo transaccional (namespace `agent_commerce.*`, ordenados por fuerza de
  la afirmación forense, como en el Cognitive Forensics Profile): mandato recibido, acción
  ejecutada contra mandato, entrega comprometida, liquidación solicitada, disputa abierta.
- Los tres tipos de disputa ya mapeados aplican directamente: de forma (¿bundle íntegro?
  — totalmente automatizable), de proceso (contra el mandate comprometido — parcialmente
  automatizable), sustantiva (dossier para adjudicador).
- La constancia multinormativa (§6.6 del Estado consolidado) se relee como **multi-mandato**:
  una transacción reflejada contra los mandatos de ambas partes simultáneamente.

Capa Profile pura. El Core no aprende qué es un mandate, igual que no sabe qué es AML ni
Hyperon.

## E-9.2 — Atestación cruzada (Protocol, forma nombrada sobre lo existente)

En una transacción entre dos agentes, **la contraparte co-firma el receipt**: A emite, B
atestigua lo que recibió, y viceversa. Consecuencias:

- La asimetría del single-signer — la debilidad estructural del caso de cumplimiento —
  desaparece gratis: atestación independiente sin red de attestors, sin registro central,
  sin coste marginal. Cada transacción produce evidencia bilateral.
- El split body/full del receipt ya soporta multi-firma sin cambiar el payload (decisión 2
  del §3.1); esto es darle **forma nombrada** (receipt de atestación cruzada), no cambiar el
  protocolo.
- Línea del informe: `TR-SIGNER-SELF` es la primera condición que la economía de agentes
  **borra por estructura** — desaparece cuando existe contrafirma de contraparte con
  identidad distinta. El registro estructural del informe mengua exactamente como la escala
  del §1.2 prometía.

## E-9.3 — Informe máquina-primero y perfil de confianza negociable (B2, inversión de orden)

Entre agentes, el consumidor del informe no es un auditor humano: es el agente contraparte
decidiendo si transacciona. Por tanto:

- **JSON estructurado y versionado primero; texto humano segundo** (inversión del orden de
  B2 en la Directiva).
- Las líneas `TR-*` dejan de ser solo diagnóstico post-hoc y pasan a ser **términos de la
  transacción**: un agente declara en el handshake su perfil de evidencia ("emito bajo
  `agent_commerce` v1, con anclaje, con atestación cruzada") y la política de la contraparte
  exige, acepta o rebaja. El informe de confianza residual se convierte en el lenguaje común
  para *cotizar* confianza entre máquinas — sin emitir veredictos, como siempre.
- **Finalidad probatoria progresiva** como propiedad nombrada: receipt inmediato a velocidad
  de transacción, anclaje por epoch después, con la garantía subiendo de nivel conforme el
  epoch ancla (`TR-ANCHOR-UNVERIFIED` → verificado). Es la escala del §1.2 aplicada a cada
  transacción individual. El diseño ya lo soporta; esta enmienda lo nombra y lo vende.

## E-9.4 — Giro de anclaje: EVM/Base-first, multi-anclaje por diseño

- **Primer anclaje comercial: EVM/Base** — donde liquida x402 y vive la infraestructura de
  la contraparte. Vía de implementación preferente: **EAS (Ethereum Attestation Service)**
  ya desplegado en Base — el `AnchorBackend` EVM puede ser poco más que publicar el
  `epoch_root` como atestación EAS o evento de contrato mínimo. Coste estimado: inferior al
  adaptador Blockfrost que planeaba A2.
- **Multi-anclaje como argumento de neutralidad**: el mismo `epoch_root` anclable en N
  sustratos independientes. Es el argumento que ninguna plataforma puede copiar (Google
  anclará en lo suyo; la evidencia de ACTA no muere con ninguna cadena). "Anclado en N
  cadenas" es mejor línea de informe que "anclado en una".
- **Cardano: de first a catálogo**, como segunda ancla natural del multi-anclaje (estable,
  barato para metadata, ajeno al stack de pagos — independencia real) y por el valor de
  ecosistema (ASI/Deep Funding, demo Hyperon). Sin urgencia, sin borrarlo.
- **ASI:Chain: catálogo, si el ecosistema lo pide** (baja un escalón desde "diferido").
- **Criterio de cierre de A2 (enmienda al §8 de la Directiva):** de "epoch_root publicado en
  Cardano testnet" a *"epoch_root publicado en el sustrato del primer comprador, con
  `AnchorRefV0` conteniendo referencias reales"*. La compuerta de A2 se mantiene en
  espíritu — adaptador real cuando haya contraparte real — pero su coste con EAS permite
  convivencia de mock + anclaje en Base testnet casi sin inversión.
- Invariante que no cambia: **ninguna cadena es constitutiva**. Este giro es la prueba de
  que la regla funciona — cambiar de primer anclaje no toca ni una línea de Core ni de
  Protocol.

## E-9.5 — Identidad: adoptar DID/VC del stack, no inventar

A3 inline sigue correcto para v0. La evolución del registro de claves declarada en A3 se
concreta: **resolución DID / W3C Verifiable Credentials**, el estándar que AP2 ya usa. Se
sube de post-MVP a evolución nombrada de A3. No se implementa hasta que un integrador lo
necesite.

## E-9.6 — Demostradores: OpenWorker como suelo público, OmegaClaw como techo técnico

- **Nuevo objetivo de nivel 1 (forense de forma): OpenWorker** (`andrewyng/openworker`,
  MIT). Su audit trail con procedencia de aprobación por tool call es un registro local
  auto-atestado — el `memory/history.metta` con mejor marketing. El Artefacto 1 (borrado
  silencioso: editar el registro, nada lo detecta; misma traza bajo ACTA, la edición rompe
  Chronos+Merkle) se replica ahí, sobre un sistema mainstream — esquivando el riesgo del
  §4.3 de la Directiva (quedar etiquetados como "capa forense de Hyperon").
- **Adaptador de emisión OpenWorker**: conector vía MCP (soportado nativamente por ellos),
  sin PR upstream (su política de contribuciones lo desaconseja). Su procedencia de
  aprobación (auto-aprobado / aprobado-por-humano / denegado, con razonamiento del revisor)
  mapea a event kinds del Cognitive Forensics Profile con fuerzas forenses distintas — es
  mejor materia prima que la de OmegaClaw para el perfil. Las ~200 líneas commodity del
  §1.3.
- **OmegaClaw conserva el techo (nivel 2)**: recomputación de inferencias, C2 rediseñado
  según captura 3 (auditoría eslabón a eslabón, cifra + descargo, t1/t6 como material
  narrativo). Nada de C2 cambia con esta enmienda.
- **AML (C1) se degrada a perfil de referencia interno.** Deja de ser "el suelo que se
  vende" (§4.1 de la Directiva, enmendado): ese suelo está ocupado por vendedores de
  atestación y su comprador espera al disparador de 2027. Régimen concreto:
  - Se cierra **en su estado actual**: sin sesión dedicada, sin completar eventos más allá
    de lo ya hecho, y **sin anclaje end-to-end** — el criterio de C1 del §8 de la Directiva
    ("flujo completo con anclaje real") **se retira**.
  - **Desaparece de toda comunicación externa**: ningún pitch, demo pública, documento de
    posicionamiento ni material de la era agentes lo menciona. La historia externa es
    `agent_commerce` + OpenWorker + OmegaClaw, exclusivamente.
  - Se conserva en el repo por tres motivos internos: es el único perfil que **prueba la
    generalidad del Core** (no-cognitivo, no-agéntico — el testigo de que "el Core verifica
    gramática probatoria, no significado de dominio" es verdad y no eslogan); su coste de
    conservación es ~cero; y es la **opción sobre el mercado de 2027** — cuando el
    disparador supervisor llegue, reactivar un perfil existente cuesta una semana, empezar
    de cero cuesta meses.
- La maquinaria legal pesada (`NormativeRefV0`, constancia multijurisdiccional plena) sale
  de la ruta crítica sin borrarse: es para el comprador de 2027.

## E-9.7 — Carrera de estandarización contra la captura de plataforma

**La amenaza al primer puesto no es un competidor: es que la plataforma llene el hueco**
(Google metiendo evidencia de ejecución en AP2; Coinbase en x402). Defensa, por este orden:

1. **La neutralidad es el foso**: la capa de evidencia de una plataforma tiene el problema
   del operador (Google atestiguando agentes de Google no es evidencia — es el "quién audita
   al auditor" que ACTA existe para resolver). Este argumento solo funciona si ACTA llega
   **antes** como estándar abierto.
2. **Movimiento concreto**: proponer el bundle de ACTA como *extensión de evidencia* a las
   comunidades x402 (la Foundation tiene proceso abierto) y AP2 (publica extensiones) — la
   jugada C2PA: colocarse como el estándar de procedencia antes de que el hueco se llene.
3. **Reparto de capas del §1.3, ahora con sentido competitivo**: formato y verificador
   regalados (estándar abierto); negocio en el registro de perfiles y la red de atestación.
4. **Estilo**: constructivo siempre. Con OpenWorker: el conector como regalo a su ecosistema
   ("ACTA hace vuestro audit trail verificable por terceros"), la demo habla sola, nadie es
   acusado. Una corrección técnica concreta vale más que cualquier propuesta genérica —
   doctrina OProW, §5.7.

## E-9.8 — Conversaciones de validación: interlocutores redefinidos

El guion se conserva (estructura, ficha, regla de decisión anti-cortesía). Cambian los
perfiles y la pregunta central:

1. **Constructor sobre x402/AP2**: ¿cómo resolvéis hoy una disputa entre agentes? ¿qué
   evidencia acepta el facilitador/adjudicador?
2. **Operador de agentes con dinero real en juego**: ¿qué te haría confiar en el agente de
   otro? ¿qué registro exigirías tras un fallo caro?
3. *(Opcional, para calibrar el mercado 2027)* **Cumplimiento**, con la pregunta invertida:
   ¿qué haría que dejaras de aceptar la palabra del operador? — su respuesta fecha la
   maduración del segundo mercado.

La regla de decisión sobre A2 se re-liga a estas conversaciones (el sustrato ahora es EVM,
pero la compuerta "adaptador real cuando haya contraparte real" se conserva).

---

## Tabla de impacto — qué toca y qué no

| Pieza | Efecto |
|---|---|
| Core / Protocol v0 / codificación canónica | **Intacto** (invariante del giro) |
| ADR-003/004/005/006 | **Intactas** — nada de esta enmienda las contradice |
| Compuerta E0 y decisiones derivadas | **Intactas** — C2 se ejecuta como quedó rediseñado |
| Directiva §1.3 (reparto de capas) | Confirmado y reforzado (E-9.7) |
| Directiva §2 (orden de construcción) | A2 → sustrato EVM/EAS, criterio §8 enmendado |
| Directiva §4 (demostradores) | C1 → referencia interna (criterio §8 retirado, fuera de comunicación externa); nivel 1 público → OpenWorker; C2 sin cambios |
| Directiva §9 (validación) | Respondida por desk research; conversaciones redefinidas (E-9.8) |
| Estado consolidado §6.7 (postura de sustrato) | Cardano-first → EVM-first + multi-anclaje + catálogo |
| Estado consolidado §6.6 (multinormativa) | Relectura multi-mandato (E-9.1), sin borrar la original |
| B2 (informe) | Máquina-primero; perfil de confianza negociable (E-9.3) |
| A3 (claves) | Evolución nombrada a DID/VC (E-9.5), sin implementación inmediata |
| Calendario de códigos | `TR-SIGNER-SELF` retirable por contrafirma (E-9.2); resto igual |

## Secuencia de sesiones (sustituye a la del plan director desde S6)

| Sesión | Contenido |
|---|---|
| S6 | C2 (los dos artefactos, sobre OmegaClaw) — ya desbloqueado por ratificaciones |
| S7 | Especificación del perfil `agent_commerce` (E-9.1) + forma nombrada de atestación cruzada (E-9.2) como ADR *Proposed* |
| S8 | Adaptador `AnchorBackend` EVM/EAS con mock→Base testnet (E-9.4) + informe JSON-primero (E-9.3) |
| S9 | Conector de emisión OpenWorker + Artefacto 1 sobre OpenWorker (E-9.6) |
| S10 | Propuesta de extensión a x402/AP2 (E-9.7) — **solo con demo funcionando** |
| Paralelo [RUB] | Conversaciones E-9.8; ratificaciones; contactos de comunidad |
| Paralelo [EJEC] | Deuda documental restante. C1: solo congelar en estado actual y documentar su régimen de referencia interna — sin sesión dedicada |

## Ratificación

Aceptar esta enmienda requiere aprobación explícita del operador bajo ADR-001. Al
ratificarse: (1) se levanta la congelación de Cardano/agentes/EVM; (2) el ejecutor refleja
los cambios en `next-milestones.md`, `current-state.md` y las marcas de enmienda de la
Directiva y el Estado consolidado; (3) las piezas con forma de decisión de protocolo o
perfil (E-9.1, E-9.2, E-9.4) entran como ADRs *Proposed* individuales cuando les toque
construcción — esta enmienda fija el rumbo, no los detalles normativos de cada pieza.
