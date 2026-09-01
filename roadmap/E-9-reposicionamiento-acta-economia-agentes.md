# E-9 — Reposicionamiento: ACTA como capa de confianza de la economía entre agentes

**Fecha:** 31 de agosto de 2026
**Tipo:** enmienda estratégica a la Directiva de construcción
**Estado:** **Accepted — ratificada por el operador bajo ADR-001**
**Revisión:** corregida tras la revisión paralela del 31 de agosto de 2026
**Ratificación:** paquete E-9 + E-1 + E-2 + E-6, 31 de agosto de 2026, sobre el commit
`503aefbd0636ba3ef52b782c675b1a901e27466a`
**Autoridad:** enmienda a `roadmap/directiva-construccion-2026-08-31.md` (§1.3, §2, §4, §9 y
la postura de sustrato del Estado consolidado §6.7). Subordinada a las Foundations y al
Protocol. **No toca el Core ni el Protocol v0; no contradice las ADR aceptadas 003–006, y
E-1/E-2 se materializan sin alterarlas en ADR-007/008.**

> **Entrada en vigor.** La ratificación del paquete levantó la congelación de
> Cardano/agentes/EVM y sustituyó el rumbo anterior en los puntos enumerados por esta
> enmienda. No ratifica por anticipado los detalles normativos de E-9.1, E-9.2 o E-9.4: sus
> ADRs deben entrar como *Proposed* y alcanzar *Accepted* antes de que la implementación fije
> esos detalles.

---

## 0. Por qué (base de evidencia)

La validación del §9 de la Directiva se ejecutó en dos vías: investigación de señales duras
observables (desk research multi-fuente, 31-ago-2026) y el guion de conversaciones (en
`research/guion-validacion-tres-conversaciones.md`, pendiente de ejecutarse con interlocutores
redefinidos por esta enmienda). Los hallazgos que motivan el giro:

> **Estado de fuentes.** El operador anunció que entregará aparte el informe de investigación
> de mercado del 31-ago-2026. Hasta incorporarlo bajo `research/` y enlazar sus fuentes
> fechadas, las cifras y afirmaciones externas de este §0 son la base declarada de la decisión
> estratégica, no hechos independientemente reproducibles desde este repositorio. La
> ratificación fija el rumbo; no convierte la evidencia pendiente en evidencia presente.

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
  una transacción reflejada contra los mandatos de ambas partes simultáneamente. Mecanismo:
  `ActaEventV0` porta un solo `policy_snapshot`, así que el multi-mandato compromete un
  **manifiesto canónico de N mandatos** cuyo hash es el `policy_hash` — el mismo patrón
  manifiesto-como-política que ADR-004 estableció para el lockfile. La alternativa (eventos
  separados por mandato) queda disponible para el perfil; el formato exacto se decide en la
  ADR del perfil cuando se construya.

Capa Profile pura. El Core no aprende qué es un mandate, igual que no sabe qué es AML ni
Hyperon.

## E-9.2 — Atestación cruzada (Protocol, forma nombrada sobre lo existente)

En una transacción entre dos agentes, la contraparte puede atestiguar. Dos formas, con
significados distintos que el perfil debe definir — no confundirlas:

- **Co-firma del mismo receipt**: A y B firman el mismo cuerpo. Prueba **acuerdo sobre la
  misma afirmación comprometida** — no prueba por sí sola "recibido" ni "entregado". El
  split body/full ya soporta multi-firma sin cambiar el payload (decisión 2 del §3.1); esto
  es darle forma nombrada, no cambiar el protocolo.
- **Eventos recíprocos**: cada parte emite su propio evento sobre lo que observó ("entrega
  recibida", "pago constatado"), con su propio receipt, referenciándose mutuamente. Las
  afirmaciones de entrega y recepción viven aquí, como event kinds propios de
  `agent_commerce` (E-9.1), no como interpretación implícita de una firma.

Sobre la línea del informe — con precisión, porque el predicado vigente es un artefacto
ratificado:

- **Hoy**, `TR-SIGNER-SELF` avisa ante cualquier autofirma, aunque exista un firmante
  adicional. La contrafirma **no la borra** bajo el predicado vigente.
- **La retirada requiere un predicado nuevo** ("existe atestador con identidad
  independiente verificada"), que entra como ADR *Proposed* cuando se construya el perfil.
  Esta enmienda fija el rumbo; no reescribe predicados de pasada.
- **Y la independencia es tan fuerte como la vinculación de identidad**: con claves inline
  autoafirmadas (A3 v0), "firmante independiente" es a su vez afirmación del productor
  (interacción con `TR-KEY-SELF-ASSERTED`). Sin vinculación externa (el DID de E-9.5), la
  contrafirma **rebaja** la condición, no la borra. El borrado pleno llega con
  contrafirma + identidad vinculada externamente. E-9.2 y E-9.5 son, por tanto, un par.

La promesa estructural se mantiene, correctamente formulada: la economía de agentes hace la
atestación independiente **barata y natural** (cada transacción la produce), y el registro
estructural del informe mengua conforme predicado e identidad maduran — la escala del §1.2
funcionando, sin atajos.

## E-9.3 — Informe máquina-primero y perfil de confianza negociable (B2, cambio de prioridad)

Entre agentes, el consumidor del informe no es un auditor humano: es el agente contraparte
decidiendo si transacciona. Por tanto:

- **JSON estructurado y versionado como salida primaria; texto humano como segunda
  representación.** La Directiva ya enumera JSON antes que texto; el cambio real es el
  consumidor principal y el criterio de cierre, que pasan de humano a máquina.
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
  sustratos independientes. Mecánica en v0, sin cambio de protocolo: el multi-anclaje es
  propiedad del **epoch**, no del bundle — `BundleV0` porta un solo `AnchorRefV0`, así que
  N anclas se presentan como N bundles idénticos salvo el anchor, o como lista de
  `AnchorRefV0` externa al bundle referida al mismo `epoch_root`. El formato de cable
  definitivo (¿envoltorio? ¿versión nueva?) se decide en la ADR del adaptador cuando se
  construya. Es el argumento que ninguna plataforma puede copiar (Google anclará en lo
  suyo; la evidencia de ACTA no muere con ninguna cadena). "Anclado en N cadenas" es mejor
  línea de informe que "anclado en una".
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
  **Condición de diseño obligatoria de la demo**: la rotura solo es probatoria ante un
  tercero si el compromiso se conserva **fuera del control del operador** — raíz anclada,
  o receipt/epoch_root retenido por una parte independiente. El guion de la demo incluye
  esa referencia externa explícitamente; sin ella, la demo afirmaría más de lo que prueba,
  que es exactamente lo que el Estado consolidado §11 prohíbe.
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
| Calendario de códigos | `TR-SIGNER-SELF` retirable solo con predicado nuevo + identidad vinculada externamente (E-9.2); resto igual |

## Secuencia de sesiones (sustituye a la del plan director desde S6)

| Sesión | Contenido |
|---|---|
| S6 — **implementación completada; cierre pendiente** | Profile propuesto y ambos artefactos C2 ejecutados localmente; requiere aprobación específica de ADR-009/Profile y custodia externa real del compromiso del Artefacto 1 |
| S7 | Especificación del perfil `agent_commerce` (E-9.1) + forma nombrada de atestación cruzada (E-9.2) como ADR *Proposed* |
| S8 | Solo tras validar una contraparte real y aceptar las ADR aplicables: adaptador `AnchorBackend` EVM/EAS con mock→Base testnet (E-9.4) + informe JSON-primero (E-9.3) |
| S9 | Conector de emisión OpenWorker + Artefacto 1 sobre OpenWorker (E-9.6) |
| S10 | Propuesta de extensión a x402/AP2 (E-9.7) — **solo con demo funcionando** |
| Paralelo [RUB] | Conversaciones E-9.8; ratificaciones; contactos de comunidad |
| Paralelo [EJEC] | Deuda documental restante. C1: solo congelar en estado actual y documentar su régimen de referencia interna — sin sesión dedicada |

## Ratificación

El operador ratificó explícitamente bajo ADR-001, el 31 de agosto de 2026 y contra el commit
`503aefbd0636ba3ef52b782c675b1a901e27466a`, el **paquete consolidado** formado por:

1. **Esta E-9** (versión corregida tras la revisión paralela del 31-ago: multi-mandato por
   manifiesto, mecánica v0 del multi-anclaje, semántica de la atestación cruzada con
   retirada de `TR-SIGNER-SELF` condicionada a predicado nuevo + identidad vinculada, y
   requisito de referencia externa en la demo OpenWorker).
2. **E-1** tal como quedó enmendada por E0: los cuatro códigos, con `TR-IMPORT-UNPINNED`
   reformulado desde el atajo "componente cuyo pin vive fuera del artefacto" a la condición
   evaluable **"componente sin raíz de procedencia inmutable registrada en el manifiesto
   conforme a ADR-004"**. Incluye una referencia mutable de build no resuelta o un pin
   externo no absorbido por commit/digest; no dispara solo porque `.git` no sobreviva si el
   digest inmutable de imagen sí consta como raíz (evidencia: captura 2, §2.6 de
   E0-resultados).
3. **E-2** tal como quedó enmendada por la captura 3: predicado desplazado
   `bytes(premisa) == bytes(T(C))` con `T` declarada del arnés, más las condiciones de
   `process_ref` y precedencia Chronos, y el descargo de selección inseparable de la cifra.
4. **E-6**: el rediseño de C2 (auditoría eslabón a eslabón; 5/5 solo como techo bajo
   instrucción de copia; t1/t6 como material narrativo).

Los puntos 2 y 3 se materializan como decisiones vinculantes en ADR-007 y ADR-008. E-6 queda
ratificada como decisión de roadmap, con el alcance medido y los límites descritos arriba.
C2 queda desbloqueado por el paquete, sujeto a formalizar y ratificar primero el Cognitive
Forensics Profile y a cualquier nueva decisión normativa que aparezca durante S6.

La ratificación: (1) levanta la congelación de Cardano/agentes/EVM; (2) pone en vigor la
secuencia S6–S10 de esta enmienda; y (3) mantiene como tarea documental pendiente la
incorporación del **informe de investigación de mercado del 31-ago** bajo `research/`, seguida
de la anotación de este §0 con referencias fechadas que separen hechos medidos de inferencias
estratégicas. No se inventa ni se reconstruye ese informe antes de que el operador lo entregue.

El mensaje de ratificación hizo explícita la entrega posterior del informe (“lo enviaré
aparte”). Esa instrucción posterior dispensa la condición de incorporación previa o atómica
que figuraba en el baseline ratificado, sin convertir las afirmaciones del §0 en evidencia ya
presente. El estado de fuentes de este documento sigue siendo vinculante hasta la entrega.

Las piezas con forma de decisión de protocolo o perfil (E-9.1, E-9.2 y E-9.4) entrarán como
ADRs *Proposed* individuales y deberán alcanzar `Accepted` antes de que su implementación fije
detalles normativos. Esta enmienda fija el rumbo, no esos detalles.
