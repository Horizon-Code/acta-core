# ACTA — Directiva de construcción

> **Nota de incorporación (T1).** Este documento se incorpora al repo íntegro y literal, tal
> como se redactó el 31-ago-2026, **antes** de los tres intercambios de análisis que
> produjeron `roadmap/E0-protocolo-y-enmiendas.md`. Las enmiendas E-1..E-8 de ese documento
> corrigen o amplían secciones concretas de este — en particular: **E-5 corrige el §5.2**
> (el `lib_pln` de OmegaClaw no contiene stamps; el testigo se reformula por
> reproducibilidad, no por transporte), **E-6 rediseña el Artefacto 2 del §4.2**, **E-1/E-3
> amplían la tabla del §3**, y **E-8 precisa la nota regulatoria del §9**. Donde este texto
> y las enmiendas difieran, mandan las enmiendas. No se ha alterado ni una línea del cuerpo.

> **[Enmendado por E-9 — Accepted 31-ago-2026]** El paquete E-9 + E-1 + E-2 + E-6,
> ratificado por el operador bajo ADR-001 sobre el commit
> `503aefbd0636ba3ef52b782c675b1a901e27466a`, sustituye el primer
> comprador, el primer sustrato y el orden desde S6. E-1 y E-2 son vinculantes en ADR-007 y
> ADR-008. Las notas de enmienda de §4, §5.9, §8 y §9 resumen los puntos desplazados;
> el texto completo y sus límites están en
> `roadmap/E-9-reposicionamiento-acta-economia-agentes.md`.

**Fecha:** 31 de agosto de 2026
**Tipo:** directiva de ejecución
**Autoridad:** subordinado a `constitution/ACTA_Foundations_v1.2_consolidado.pdf` y a
`protocol/ACTA_Protocol_v0.md`. Complementa a `acta-estado-consolidado.md`, que describe
*qué hay*; este documento describe *qué construir y en qué orden*.

> Este documento no redefine ACTA. Fija una decisión de posicionamiento, un orden de
> construcción con sus razones, dos demostradores concretos, y los datos verificados del
> ecosistema Hyperon que el ejecutor necesita para no perder tiempo. Donde contradiga a las
> Foundations o al Protocol, mandan esos.

---

## 1. La decisión que ordena todo lo demás

**El entregable de ACTA no es el `BundleV0`. Es el informe de confianza residual que se
deriva de él.**

El bundle es el insumo. Lo que se entrega a un humano es la respuesta a una pregunta
distinta: *dado este dossier, ¿en quién hay que seguir confiando para que se sostenga?*

Esto ya está en el diseño sin nombre. La columna **"Qué no prueba"** de la tabla del §3 de
`acta-estado-consolidado.md` es un mapa de vulnerabilidad enumerado objeto por objeto, y el
pilar de **confianza explícita** lo formula como principio: toda dependencia de confianza
debe quedar nombrada y acotada.

### 1.1 Consecuencias operativas

**La deuda deja de ser deuda.** Las listas del §5.2 (deuda aceptada) y del §8.1 (preguntas
sin resolver) dejan de ser carencias pendientes y pasan a ser **el esquema de salida del
informe**: son las categorías de vulnerabilidad que el verificador sabe detectar y nombrar.
Eso convierte la lista de lo que falta en la especificación funcional de lo que hay que
construir.

**La escala de vocabulario del §1.2 se reinterpreta.** Paquete probatorio → dossier
atestiguado → registro anclado → fe computacional delegada no es una escala de madurez del
producto. Es **una escala de vulnerabilidades retiradas**: cada fase elimina una línea del
informe. Es más honesto y más defendible ante un auditor que decir "estamos en la v0".

**El riesgo de comunicación del §11 se resuelve estructuralmente.** Un producto cuyo output
*es* la lista de lo que aún no garantiza no puede presentarse como si la responsabilidad
plena estuviera implementada. La honestidad deja de ser una disciplina y pasa a ser una
propiedad del artefacto.

### 1.2 Forma comercial

Marco afirmativo, no acusatorio. El informe no dice "eres vulnerable aquí"; dice **"esto es
lo que este dossier soporta, y esto es lo que requeriría un paso adicional"**. Misma
información, decisión de producto consciente.

### 1.3 Reparto de capas

| Capa | Naturaleza | Decisión |
|---|---|---|
| **Formato** (spec, vectores, canonicalización) | Estándar | Abierto. Forzado por el pilar de registro portable: una evidencia que solo puede verificar una empresa no es evidencia, es un servicio |
| **Adaptadores** de emisión | Commodity | Abiertos. Un plugin son ~200 líneas; no hay foso |
| **Verificador** | Producto | Código ejecutable por cualquiera, sin llamar a casa. Un verificador que solo ACTA puede ejecutar es una autoridad, y el §11 dice que ACTA no lo es |
| **Registro de perfiles y red de attestors** | Negocio | Reputación del formato, no control del cómputo |

**ACTA es un protocolo con SDK, no un framework.** El sistema que produce los eventos ya
existe y no se va a reorganizar alrededor de ACTA. Referencia mental: OpenTelemetry o C2PA.

---

## 2. Orden de construcción

La regla de precedencia es: **nada de lo que hay encima del Core vale si el epoch todavía se
puede reescribir antes de publicarse.** Sin anclaje externo, la integridad es interna y por
tanto autoafirmada; sin verificador ejecutable por un tercero, no hay quien lo compruebe.
Fase 5 y Fase 6 son la ruta crítica y todo lo demás va después.

### Bloque A — Cierre de la cadena de confianza (bloqueante)

**A1. Verificación criptográfica de firmas (cierra Fase 2).**
Está fuera del Core por diseño y no consta implementada. Sin ella, el `ReceiptV0` es forma
sin contenido. Va primero porque es prerrequisito de todo informe.

**A2. Adaptador Cardano (Fase 5).**
`adapters/cardano-anchor/` es hoy un README vacío. `AnchorRefV0` ya existe como referencia
neutra y el Core valida su forma, así que la interfaz está fijada. Construir contra el
`AnchorBackend` de §6.7, con backend mock primero y Blockfrost después.

> **Referencia útil:** `openwater_mk/cardano.py` en `singnet/watermarks_PoC` implementa
> exactamente esto — `AnchorRecord`, anclaje bajo metadata label 40961, `MockCardanoBackend`
> funcional y `BlockfrostCardanoBackend` esbozado, con tests. Apache 2.0. No copiar; leer
> para no repetir decisiones ya tomadas sobre el esquema de metadatos.

**A3. Resolución `attestor_id → clave pública` (§8.1, no resuelto).**
Decisión pendiente y bloqueante: claves inline en el bundle, o registro público con garantías
de disponibilidad independiente. **Si la resolución depende de un endpoint de ACTA, la
independencia del verificador es ficticia y el producto entero se cae.** Recomendación:
inline en el bundle para v0 (autocontenido, verificable offline), registro público como
evolución. Documentar la limitación en el informe mientras tanto.

### Bloque B — El producto (Fase 6 + el informe)

**B1. Verificador CLI independiente.**
Binario que toma un bundle y no llama a ningún servicio. Debe poder ejecutarlo un tercero sin
relación con ACTA.

**B2. Informe de confianza residual.**
El entregable de §1. Formato doble: JSON estructurado (máquina) y texto legible por alguien
que no sabe qué es un Merkle (auditor). Esquema de salida en §3 de este documento.

> **[Enmendado por E-9.3]** No cambia el orden tipográfico ya escrito: cambia la prioridad y
> el consumidor principal. El JSON estructurado y versionado es la salida primaria para que
> una contraparte máquina negocie su perfil de confianza; el texto humano permanece como
> segunda representación.

### Bloque C — Demostradores

**C1. Demo AML** (el suelo que se vende). Ya existe el ejemplo en `acta-core/examples`;
promoverlo a crate de perfil independiente y llevarlo end-to-end con anclaje real.

**C2. Demo Hyperon** (el techo que se demuestra). Especificada en §4.

> **[Enmendado por E-9.6]** AML queda cerrado en su estado actual como perfil de referencia
> interna, sin sesión dedicada ni criterio de anclaje end-to-end, y sale de la comunicación
> externa. OpenWorker pasa a ser el objetivo público de nivel 1; OmegaClaw conserva el techo
> técnico de nivel 2. C2 mantiene sus dos artefactos y se ejecuta en S6 con el rediseño E-6.

> **[Estado S6 — implementación completa, cierre pendiente 1-sep-2026]** El Cognitive
> Forensics Profile está formalizado como propuesta en `profiles/ai-agent/README.md` y
> pendiente de aprobación específica mediante ADR-009. Los dos artefactos C2 están en
> `demos/c2-omegaclaw/`: el primero detectó el borrado sobre una copia de la historia real E0
> contra una raíz situada en una topología local separada; el segundo aplica ADR-008 a los
> strings reales E0 y aporta el parche fijado de `src/loop.metta` antes de
> `normalize_string`. Falta custodia realmente independiente o anclaje para el primero. Sus
> README conservan los límites probatorios; ningún concepto Hyperon entró en Core.

### Bloque D — Deuda documental

Va en paralelo, no después. Detalle en §6.

### Lo que NO se construye todavía

Servicio recorder (Fase 3), hardening (Fase 9), UI. El recorder solo tiene sentido cuando hay
alguien emitiendo en volumen; hasta entonces la emisión puede ser una librería. La UI es
irrelevante hasta que el informe esté definido.

---

## 3. Especificación del informe de confianza residual

Cada línea del informe es una **dependencia de confianza nombrada y acotada**. El verificador
la emite cuando detecta la condición; cuando la condición se retira, la línea desaparece.

| Código | Condición detectada | Texto (marco afirmativo) |
|---|---|---|
| `TR-SIGNER-SELF` | `actor_ref.actor_id` == identidad del attestor del receipt | El productor y el atestiguador son la misma entidad; una atestación independiente reforzaría este dossier |
| `TR-NO-ANCHOR` | `bundle.anchor` ausente | La secuencia es internamente consistente; el anclaje externo fijaría el momento a partir del cual no puede reescribirse |
| `TR-ANCHOR-UNVERIFIED` | `anchor` presente pero no comprobado contra el ledger | El anclaje está declarado; su inclusión real requiere consulta al sustrato |
| `TR-POLICY-WEAK` | `policy_hash` presente pero no verificado contra artefacto | La norma aplicable está nombrada; verificar su contenido requiere resolver el artefacto |
| `TR-RULESET-UNPINNED` | Evento con contenido inferencial sin `PolicySnapshotV0` de tipo `inference_ruleset` | La conclusión no es recomputable sin fijar el conjunto de reglas bajo el que se derivó |
| `TR-NONDET-INPUT` | Premisa marcada como producida por proceso no determinista | La formulación de la premisa no es reproducible; su fiabilidad es afirmación del productor |
| `TR-TIME-DECLARED` | Siempre, en v0 | `issued_at` es afirmación del productor; Chronos garantiza orden relativo, no absoluto |
| `TR-TIME-INCOHERENT` | Evento con `issued_at` anterior al evento causante en la cadena | La secuencia es válida pero temporalmente incoherente según el perfil |
| `TR-KEYS-DEPENDENT` | Resolución de claves vía endpoint controlado por el emisor | La verificación de firmas depende de una fuente controlada por el productor |

> **[Enmendado por E-9.2]** La contrafirma no retira hoy `TR-SIGNER-SELF`: el predicado
> vigente avisa ante cualquier autofirma. Su retirada futura exige un predicado nuevo
> ratificado que detecte al menos un atestador independiente **y** una vinculación externa de
> identidad; con claves inline autoafirmadas, la contrafirma solo rebaja la condición. E-9 no
> reescribe ese predicado por sí sola.

**Regla de diseño:** el verificador nunca emite un veredicto. Emite qué está soportado y qué
requeriría un paso adicional. Si no puede determinar una condición, lo dice; no la omite.

**Regla de extensión:** añadir un código nuevo no puede requerir tocar `acta-core`. Los
códigos que dependen de semántica de dominio pertenecen al Profile.

---

## 4. Los dos demostradores

> **[Enmendado por E-9.6 — régimen vigente]** La presentación externa ya no tiene dos
> demostradores AML/Hyperon. OpenWorker es el suelo público de forense de forma y OmegaClaw el
> techo técnico de recomputación. AML se conserva solo como referencia interna que prueba la
> generalidad no cognitiva del Core y como opción sobre el mercado de cumplimiento de 2027.
> En cualquier demo de borrado, la rotura solo es probatoria si una raíz, receipt o
> `epoch_root` quedó fuera del control del operador.

### 4.1 AML — el suelo

Qué demuestra: forense de **forma** en un dominio donde existen personas que responden
legalmente por decisiones automatizadas. Técnicamente aburrido; es lo que se parece a algo
que alguien compraría.

Trabajo: promover el perfil AML a crate independiente (hoy en `acta-core/examples`, listado
como deuda aceptada), completar `account_released` / `process_closed` / `transfer_flagged`, y
llevarlo end-to-end con anclaje real.

### 4.2 Hyperon — el techo

Qué demuestra: forense de **contenido** (nivel 2). Es el único sitio donde puede demostrarse,
porque exige determinismo, semántica versionada y acceso al motor. Ventaja práctica decisiva:
**no requiere permiso de nadie** — código Apache 2.0, API de plugin documentada, Docker, y un
agente corriendo en público desde abril.

Dos artefactos cortos y específicos. Nada de "ACTA integrado con OmegaClaw", que no demuestra
nada.

**Artefacto 1 — borrado silencioso.**
Tomar `memory/history.metta` de un agente OmegaClaw, que es su traza episódica real. Editarlo
y mostrar que nada lo detecta. Después, la misma ejecución bajo ACTA: la edición rompe la
cadena Chronos y la prueba Merkle. Dos minutos, sistema real, resultado incontestable.

**Artefacto 2 — recomputación imposible.**
Tomar un ETV producido por OmegaClaw e intentar recomputarlo bajo `trueagi-io/PLN`. No sale el
mismo resultado, porque existen tres `lib_pln.metta` divergentes sin versionar (datos en §5.2).
Después, el mismo caso con `PolicySnapshotV0` de tipo `inference_ruleset`: el verificador sabe
qué reglas usar y la recomputación cierra.

Este segundo es el más fuerte que ACTA puede producir, porque no enseña una capacidad propia:
enseña **un agujero que existe hoy en un sistema que otros mantienen, y que la primitiva de
ACTA cierra**.

### 4.3 Reparto y orden de presentación

**AML demuestra el suelo que se vende; Hyperon demuestra el techo del protocolo.** A un
evaluador técnico o de Deep Funding, Hyperon primero. A alguien que firma informes de
cumplimiento, AML primero y Hyperon como prueba de que el diseño aguanta el caso difícil.

**Riesgo a evitar:** si ACTA se presenta como "la capa forense de Hyperon", hereda el tamaño
de mercado de Hyperon y su problema reputacional. Pasa de infraestructura probatoria a
proyecto de ecosistema, y de esa caja cuesta salir.

---

## 5. Hechos verificados del ecosistema

Datos obtenidos por inspección directa de código y de historial git a 31 de agosto de 2026.
**Los análisis existentes en el repo (`analisis-hyperon-experimental`, `analisis-mork`,
`analisis-petta`, `analisis-pln-chaining`) son de abril de 2026 y están desactualizados en los
puntos siguientes.**

### 5.1 El sistema objetivo: OmegaClaw-Core

`github.com/asi-alliance/OmegaClaw-Core` · Apache 2.0 · 1.170 commits desde el 21-feb-2026 ·
15+ autores · 20.377 líneas Python + 1.582 MeTTa · Docker · 28 documentos de referencia.
Corre sobre PeTTa vía `janus-swi`. Es el repositorio con más desarrollo real del ecosistema
Hyperon.

**Bucle de turno** (`reference-internals-loop.md`), donde el paso 5 es el `event_kind` que
importa:

```
1. receive()          mensaje del canal
2. getContext()       PROMPT + SKILLS + LAST_SKILL_USE_RESULTS + HISTORY + TIME
3. llamada LLM        Anthropic / OpenAI / ASICloud / ASI:One
4. sread / balance    parsear respuesta en s-expresiones de skill
5. eval each skill    ← ai_agent.tool_call_executed
6. addToHistory       append a memory/history.metta
7. sleep / recurse
```

**El hallazgo que justifica el nivel 2.** De `reference-internals-memory-store.md`, sobre la
tercera capa de memoria:

> Cada llamada `(metta (|- ...))` **empieza con un AtomSpace nuevo. El conocimiento no
> persiste entre invocaciones.**

No es que el Space se reescriba: **se destruye al terminar cada inferencia**. El ETV y su
derivación existen solo dentro de la llamada. Lo que sobrevive es que el LLM reescriba la
conclusión a mano en la siguiente vuelta. La prueba no se pierde por descuido: no se conserva
nunca. Es el argumento más fuerte de ACTA y está en la documentación del propio sistema.

**El fichero vulnerable.** `memory/history.metta`, descrito como *"episodic trace (written at
runtime)"*. Fichero plano de append, escrito por el operador, sin secuenciación atestiguada,
sin encadenamiento, sin anclaje.

**API de plugin** (`reference-plugin-api.md`): un plugin es un módulo MeTTa o Python que
expone `loadOmegaClawPlugin`, se registra en `config/plugins.yaml`, y puede añadir canales,
proveedores LLM y skills — `(add-skill $function $description $arguments)` /
`(remove-skill $function)`. Todos los canales y proveedores del propio OmegaClaw están
implementados con esta API, así que está probada.

Aviso: la documentación reconoce que la API está en construcción y que ejecutar plugins dentro
de Docker requiere reconstruir la imagen o montar el código. **Antes de comprometerse con la
integración, verificar que existe un punto de intercepción en el paso 5 del bucle**; si no lo
hay, la alternativa es un canal `wschat` intermediario, no un plugin.

**Límites declarados por OmegaClaw** (`introduction.md`, sección *Honest limits*), que ACTA
debe reflejar y no ignorar:

- Errores de formulación de premisas del LLM de hasta ~16,6% en relaciones asimétricas.
- Sobreestimación de confianza de ~15 puntos porcentuales en valores autoasignados.
- Decaimiento de confianza de ~10% por salto; al tercer salto `c` suele caer bajo 0,5.
- El motor formal **no filtra la basura, la amplifica**, prestándole autoridad matemática a
  conclusiones derivadas de premisas defectuosas.

### 5.2 El testigo ya existe como tipo: ETV

De `trueagi-io/pln-experimental`, `common/truthvalue/EvidentialTruthValue.metta`:

```metta
;; Evidential truth value type. Represent a truth value alongside its evidence.
(: ETV (-> (OrderedSet $a) TruthValue EvidentialTruthValue))
```

Un valor de verdad **es** un par (conjunto de evidencias, valor). La procedencia es
constitutiva del tipo, no metadato adjunto.

De `trueagi-io/PLN/lib_pln.metta`, maquinaria de *stamps*:

```metta
;Whether evidence was just counted once
(= (StampDisjoint $Ev1 $Ev2) ...)
;Concat stamp with sorting
(= (StampConcat $stamp $addition) (msort (append $stamp $addition)))
```

Cada afirmación lleva un `EvidenceID` y el sistema comprueba solapamiento para no contar dos
veces la misma evidencia. **El subgrafo de premisas del "testigo" del §6.1 ya está serializado
dentro de cada ETV.** No hay que reconstruirlo desde fuera: hay que capturarlo y comprometerlo.

> **[Enmendado por E-5]** Verificación posterior sobre el sistema objetivo: el `lib_pln` de
> 309 líneas que OmegaClaw ejecuta **no contiene** maquinaria de stamps (grep de `stamp`,
> `evidence`, `EvID`: cero coincidencias). Lo anterior es cierto del ecosistema y falso del
> sistema objetivo. Ver E-5 en `roadmap/E0-protocolo-y-enmiendas.md`.

### 5.3 El defecto que ACTA cierra: tres PLN divergentes

```
trueagi-io/PLN/lib_pln.metta               430 líneas   md5 671daefbc87009ea5e7477ba15752010
trueagi-io/PeTTa/lib/lib_pln.metta         466 líneas   md5 17d649e8a318c4fa69a2a0c060b733ad
asi-alliance/OmegaClaw/lib_pln.metta       309 líneas   md5 834a369c91fc82206d7f464593d3ebd7
```

Tres ficheros distintos, mismo nombre, mismo propósito, ninguno declara versión.

**Esto rompe el nivel 2 tal como está formulado hoy.** Un ETV computado bajo un `lib_pln` no
es recomputable bajo otro. El testigo preserva premisas y resultado pero no la regla que los
une, y sin la regla la recomputación por un tercero es imposible. La omisión no se convierte
en perjurio falsable si el verificador no puede reproducir el cálculo.

**La corrección, y es de coste cero en Core:** usar `PolicySnapshotV0` —que ya existe— no solo
para la norma legal sino para el **conjunto de reglas de inferencia**:

```
policy_type:  "inference_ruleset"
policy_id:    "pln/omegaclaw"
policy_hash:  sha256:<hash del lib_pln.metta concreto>
```

No toca la gramática probatoria; es un uso nuevo de un tipo existente. Convierte el ETV en
recomputable. **Es la contribución técnica más concreta que ACTA puede hacer hacia el
ecosistema**, y resuelve un problema que ese ecosistema tiene y no ha visto.

> **[Enmendado por E-1/E-4]** El par (ruleset, motor) tampoco basta: el cierre de
> importaciones completo debe fijarse como manifiesto (lockfile), incluido el `git-import!`
> sin revisión. Ver E-4 en `roadmap/E0-protocolo-y-enmiendas.md`.

### 5.4 Semántica de referencia estable

`hyperon-experimental` (⭐274, MIT) está congelado desde el 11-feb-2026 en v0.2.10. **Para ACTA
eso es un activo, no un síntoma de inmadurez**: una recomputación por un tercero exige una
semántica de referencia estable, y un intérprete que se mueve rompe la reproducibilidad de
cualquier traza pasada.

Además hay tres implementaciones convergiendo sobre esa especificación:

| Implementación | Estado | Relación con la espec |
|---|---|---|
| `hyperon-experimental` (Rust) | Congelado v0.2.10 | Es la especificación |
| `PeTTa` (Prolog) | Vivo, 963 commits | Sustrato de OmegaClaw |
| `JeTTa` (Kotlin → bytecode JVM) | Vivo, v0.9.0, 35.352 líneas, MIT | Objetivo declarado: respuestas **byte a byte idénticas** sobre la suite de tests de hyperon-experimental |

Eso habilita **verificación diferencial**: un verificador que recompute la misma traza en dos
motores independientes y obtenga el mismo resultado tiene una garantía cualitativamente más
fuerte que uno que confía en un único intérprete. Es una línea de trabajo posterior, pero
condiciona el diseño del verificador: **no acoplar el verificador a un intérprete concreto.**

### 5.5 Metodología de recomputación ya escrita

`trueagi-io/MORK` tiene un corpus diferencial: `differential/run.py` pasa cada programa `.mm2`
por los dos motores de consulta y compara los espacios resultantes **byte a byte**; los
programas que fijan un espacio esperado funcionan como suite de regresión. 104 ficheros de
corpus.

Es la metodología que la Fase 6 necesita, escrita para validar dos motores internos. Aplicarla
entre implementaciones en vez de dentro de una es un cambio de alcance, no de diseño.

### 5.6 Merkleización nativa en el sustrato

`Adam-Vandervorst/PathMap` (MIT, 56.321 líneas Rust) es la estructura de datos sobre la que
corre MORK. Tiene `src/merkleization.rs`:

```rust
pub struct MerkleizeResult {
    pub hash: u128,        // hash de todo el trie bajo la raíz
    pub reused: usize,     // referencias compartidas que reemplazaron copias idénticas
    ...
}
```

**Matiz decisivo:** usa `gxhash`, un hash **no criptográfico** de 128 bits, y su propósito es
**deduplicación estructural**, no evidencia de manipulación. Es un Merkle para compartir
memoria, no para detectar reescritura. Misma forma, función hash equivocada para lo forense.

La estructura existe y los comentarios internos ya discuten "merkleización gradual" como
proceso de fondo. Sustituir `gxhash` por SHA-256 en un modo opcional daría un atomspace
direccionable por contenido con raíz estable, y convertiría la anti-omisión de "ACTA encadena
lo que el productor emite" a "el estado del atomspace tiene raíz verificable en cada punto".

**Esto es exploración a largo plazo, no ruta crítica.** Antes de construir nada encima:
resolver §5.8.

### 5.7 Diseño gemelo: la capa de confianza de OProW

El SDK `oprow`, entregado por Ben Goertzel a SingularityNET como archivo ZIP y vendorizado en
`singnet/watermarks_PoC`, contiene en su Step 11 una capa de confianza con esta forma:

```
AnchorRecord · AnchorReceipt · TrustBackend (protocol)
MemoryTrustBackend · MultiTrustBackend
ASIChainTrustBackend + plantillas de anclaje en Rholang
```

Es el `AnchorBackend` de ACTA diseñado independientemente.

Dos coincidencias más finas:

**El split body/full.** OProW Step 1 separa `ManifestCore` (objeto semántico firmado, sin
locator ni firmas) de `SignedManifest` (core + firmas) de `ManifestEnvelope` (transporte), y
justifica el split así: *evita el hashing autorreferencial, porque el locator se deriva de
bytes estables de `SignedManifest` y no se reinserta en ellos*. Es la decisión 2 del §3.1 de
ACTA. Dos equipos que no se conocen, mismo problema, misma solución estructural.

**La frontera de seguridad.** OProW: *el backend de confianza ancla solo compromisos
compactos; no debe publicar medios en bruto, logs de consulta, claims privados cifrados ni
manifiestos completos*. Es el pilar de compromiso sin exposición.

**Dónde ACTA es mejor, y es lo que hay que llevar a una conversación:** OProW usa CBOR
canónico con **mapas**. La decisión 1 del §3.1 de ACTA identifica por qué los mapas no son
canónicos entre implementaciones y lo resuelve con **arrays posicionales**. Es una corrección
técnica concreta sobre un diseño de Goertzel, y vale más que cualquier propuesta genérica de
colaboración.

### 5.8 Riesgo legal del sustrato — comprobar antes de depender

| Componente | Licencia | Nota |
|---|---|---|
| `OmegaClaw-Core` | **Apache 2.0** ✓ | Limpio. Es la integración recomendada |
| `PeTTa` | **MIT** ✓ | Limpio |
| `JeTTa` | **MIT** ✓ | Limpio |
| `hyperon-experimental` | **MIT** ✓ | Limpio |
| `PLN` (trueagi-io) | **MIT** ✓ | Limpio |
| `pln-experimental` | dual MIT / GPL-3.0 | Elegible MIT |
| `PathMap` | **MIT** ✓ | Limpio, pero en **cuenta personal** |
| **`MORK`** | ⚠️ **NINGUNA** | No hay fichero LICENSE. **No depender** |
| `CAIRN` | ⚠️ ninguna | Además exige `metta-attention`, que no existe públicamente |
| `hyperon-miner` | ⚠️ ninguna en repo | El listado dice AGPL |
| `oprow` (en watermarks_PoC) | ⚠️ ninguna + nota de no distribuir en repo público | Leer, **no reutilizar código** |

**Regla:** en este ecosistema la ausencia de licencia es el estado por defecto. La
comprobación de licencia va **antes** que la de calidad en cualquier decisión de dependencia.

### 5.9 Sustrato de anclaje

La postura del §6.7 (Cardano-first, ASI:Chain diferido) es correcta y se confirma:
`asi-alliance/asi-chain` está en **v0.1.0 BETA**, todos los endpoints apuntan a
`*.dev.asichain.io`, y aunque tiene equipo grande y desarrollo activo (219 commits, activo en
agosto), sigue siendo testnet. Mantener Cardano-first.

> **[Enmendado por E-9.4]** El primer anclaje comercial pasa a EVM/Base, preferentemente EAS,
> condicionado a una contraparte real y a las ADRs aplicables. Cardano pasa de *first* a
> catálogo como segunda ancla natural; ASI:Chain queda también en catálogo si el ecosistema lo
> pide. El diseño sigue siendo neutral: ningún sustrato es constitutivo. En v0, N anclas se
> representan mediante N bundles con el mismo `epoch_root` o referencias externas; cualquier
> nuevo formato de cable requiere su propia decisión versionada.

---

## 6. Deuda documental — llevar al repo lo decidido

La brecha entre lo decidido y lo escrito es, según el propio §11, la mayor del proyecto. Va en
paralelo a la construcción, no después.

1. **Escribir el §6 completo en el repo.** Los dos niveles forenses, el Cognitive Forensics
   Profile, el modelo de cuatro piezas, los tres tipos de disputa, el modo señal pre-acto y la
   constancia multinormativa. Ninguno existe documentalmente.
2. **Formalizar el Cognitive Forensics Profile** bajo `profiles/`, con `ai_agent.*` como
   namespace y los event kinds ordenados por fuerza de la afirmación forense. Es capa Profile,
   no Core; la prohibición del readiness no lo bloquea.
3. **Añadir el límite de no determinismo al nivel 2.** El §6.1 asume "sistemas reproducibles y
   deterministas", pero la fase de atomización no lo es (§5.1). Formulación recomendada: *ACTA
   preserva con integridad la fase determinista del razonamiento; la formulación de premisas
   es no determinista y su fiabilidad es afirmación del productor, que debe declararla.* Debe
   documentarse igual que ya se documenta la asimetría del single-signer.
4. **Escribir el contrato de compatibilidad de perfiles** (§8.1, fragmentación entre perfiles).
5. **Especificar la regla de frontera de epoch en Chronos** (§8.1, sin especificar).
6. **Promover `analisis-filosofico-claude-codex.md`** a normativo; los 13 pilares no son
   research.
7. **Convertir `roadmap/acta-mvp-fases-tecnicas.txt` a `.md`** con columna de estado.
8. **Actualizar los cuatro análisis de ecosistema** con §5 de este documento. Son de abril y
   no cubren OmegaClaw, que es el sistema objetivo.

---

## 7. Reglas de acoplamiento — lo que no se debe hacer

**El Core no debe enterarse de que Hyperon existe.** Ni MeTTa, ni PLN, ni átomos, ni valores de
verdad. Todo eso es Profile. La regla del §2 sigue vigente: si hace constitutivo un sustrato
concreto, se rechaza.

**El plugin de emisión debe ser fino.** Su único trabajo es traducir un evento del sistema
anfitrión a `ActaEventV0` y emitirlo. Si crece más allá de eso, algo está mal ubicado.

**El verificador no debe acoplarse a un intérprete concreto** (§5.4). La verificación
diferencial entre implementaciones es una propiedad que hay que preservar por diseño aunque no
se use en v0.

**No depender de MORK ni de PathMap** mientras el primero no tenga licencia y el segundo esté
en una cuenta personal (§5.8). La merkleización nativa es exploración, no ruta.

**No presentar ACTA como componente del ecosistema Hyperon.** Hyperon es el demostrador del
techo, no la identidad del proyecto (§4.3).

**Nada de lo que se construya puede requerir confiar en ACTA para verificarlo.** Es la
condición que hace que el producto sea evidencia y no servicio.

---

## 8. Criterios de hecho

Un bloque está cerrado cuando se cumple su criterio, no cuando el código pasa los tests.

| Bloque | Criterio de cierre |
|---|---|
| **A1** Firmas | Un bundle con firma manipulada es rechazado por código que no vive en `acta-core` |
| **A2** Anclaje | Un `epoch_root` está publicado en Cardano testnet y el `AnchorRefV0` correspondiente contiene `tx_id` y `slot` reales |
| **A3** Claves | Un tercero sin acceso a ningún servicio de ACTA puede resolver `attestor_id → clave pública` desde el bundle |
| **B1** Verificador | Un tercero ejecuta el binario sobre un bundle, sin red, y obtiene el mismo resultado que el emisor |
| **B2** Informe | Alguien que no sabe qué es una prueba Merkle lee el informe y entiende en quién tiene que confiar y por qué |
| **C1** AML | El flujo completo corre end-to-end con anclaje real, no mock |
| **C2** Hyperon | Los dos artefactos del §4.2 se ejecutan sobre un OmegaClaw real y el segundo muestra la recomputación fallando y después cerrando |

> **[Enmendado]** La fila E0 y el rediseño del criterio de C2 están en
> `roadmap/E0-protocolo-y-enmiendas.md` (Parte 1 y E-6) y en `next-milestones.md`.

> **[Enmendado por E-9]** A2 cierra cuando el `epoch_root` se publica en el sustrato del
> primer comprador y el `AnchorRefV0` contiene referencias reales. C1 pierde su criterio
> end-to-end y queda referencia interna. C2 demuestra borrado con compromiso conservado por
> una parte independiente y auditoría eslabón a eslabón conforme a ADR-008; el 5/5 de E0 solo
> se cita como techo bajo instrucción explícita de copia, nunca como tasa del mediador.

---

## 9. La pregunta que decide el proyecto

Ninguna de las decisiones anteriores resuelve el riesgo principal, que no es técnico ni de
posicionamiento: **no consta que exista hoy alguien que necesite ACTA lo bastante como para
pagarlo.**

El calendario regulatorio se movió. El Digital Omnibus (Reglamento UE 2026/1744, en vigor
desde el 27-jul-2026) aplazó las obligaciones de alto riesgo del Anexo III del 2-ago-2026 al
**2-dic-2027**, y las del Anexo I al 2-ago-2028. El motivo fue que las normas armonizadas no
estaban listas, así que los requisitos no cambiaron, solo la fecha. Las obligaciones de
transparencia del Artículo 50 se quedaron en agosto de 2026.

Eso da dieciséis meses de pista y significa que el mercado de herramientas está sin formar.
También significa que más de la mitad de las organizaciones encuestadas todavía no tenían
inventario de los sistemas de IA que operan — un mercado que no sabe qué tiene no compra
propagación de confianza.

**La pregunta a responder antes de comprometer meses de construcción:** *¿existe alguien, hoy,
que responda legalmente por un sistema automatizado y que no pueda reconstruir por qué hizo lo
que hizo?*

Si la respuesta es sí, la ruta crítica de §2 es correcta y hay que ejecutarla. Si la respuesta
es que se apañan con logs y un procedimiento firmado, entonces ACTA es un protocolo elegante
sin comprador, y eso no lo arregla ninguna integración ni ninguna demo.

**Tres conversaciones con personas que firman informes de cumplimiento en banca o sanidad
responden esa pregunta en una semana, y valen más que cualquier cosa que se pueda encontrar en
GitHub.** Esa validación debería ir en paralelo al Bloque A, no después.

> **[Enmendado por E-8]** Precisión regulatoria posterior (distinción procedencia del
> contenido / procedencia del proceso; el 2-dic-2026 como fin de transitorio del art. 111.4,
> no obligación nueva) en `roadmap/E0-protocolo-y-enmiendas.md`, con cita directa al DOUE.

> **[Enmendado por E-9.8]** La validación primaria pasa a: (1) constructor sobre x402/AP2 y
> (2) operador de agentes con dinero real en juego. Cumplimiento queda como conversación
> opcional para fechar el segundo mercado. La compuerta de A2 conserva su espíritu: adaptador
> real solo cuando exista una contraparte real. El informe de mercado del 31-ago que sustenta
> el giro fue incorporado literalmente el 2-sept-2026 como
> `research/investigacion-validacion-mercado-2026-08-31.md`; E-9 §0 enlaza cada afirmación con
> sus hallazgos y separa la inferencia estratégica.
