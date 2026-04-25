# TODO (classification): this file is currently kept in `research/` as exploratory analysis. Reclassify portions into `direction/` only if they become directional source text.

# Análisis filosófico y técnico de ACTA
## Por Claude Sonnet 4.6 — Abril 2026

---

## 0. Nota metodológica

Este análisis parte de una lectura exhaustiva de todo el repositorio: código fuente completo de `acta-core`, perfiles de dominio, tests de integración, documentación técnica y el documento de contexto extenso. El objetivo no es validar la visión —que es sólida— sino examinar con honestidad dónde el código ya constituye lo que la filosofía promete y dónde todavía solo lo describe.

---

## 1. Lo que el código ya hace bien (y por qué no es trivial)

### 1.1 La canonicalización por arrays CBOR posicionales es la decisión técnica más importante del proyecto

El documento describe la canonicalización como un objetivo. El código la implementa con una elección concreta que merece atención: arrays CBOR con orden de campos fijo, no mapas.

```rust
// canonical.rs
fn event_v0_to_value(event: &ActaEventV0) -> Value {
    Value::Array(vec![
        Value::Text(event.protocol.clone()),
        Value::Text(event.event_id.clone()),
        Value::Text(event.issued_at.clone()),
        process_ref_v0_to_value(&event.process_ref),
        event_kind_ref_to_value(&event.event_kind),
        ...
    ])
}
```

Los mapas CBOR o JSON no son canónicos entre implementaciones por razones reales: distintos lenguajes ordenan las claves de forma diferente, distintos parsers pueden emitir encodings distintos del mismo valor semántico. Usar arrays posicionales resuelve el problema de raíz. Esta decisión es necesaria para que el `event_hash` sea reproducible por un verificador independiente escrito en Python o Go años después. Es la garantía más importante del protocolo.

### 1.2 La separación body/full en el receipt resuelve un problema filosófico real

```rust
// receipt.rs
// El BODY es lo que se firma. El FULL incluye las propias firmas.
pub fn receipt_v0_signing_payload(receipt: &ReceiptV0) -> Result<Vec<u8>, ReceiptError>
pub fn validate_receipt_v0_shape(receipt: &ReceiptV0) -> Result<(), ReceiptError>
```

Si las firmas se incluyeran en el payload a firmar, se crearía una circularidad imposible. Más sutil: `receipt_body_hash ≠ receipt_full_hash`. El test `receipt_body_hash_does_not_depend_on_signatures` verifica exactamente esto. El attestor firma el cuerpo del recibo (event_hash + chronos_ref + issued_at), no el recibo completo. Esto permite multi-firma sin cambiar el payload: distintos attestors pueden añadir su firma sin invalidar las anteriores. Esta propiedad es necesaria para la evolución hacia N-of-M attestation sin romper el protocolo v0.

### 1.3 La separación ActaEventV0 / ChronosStampedEventV0 es filosóficamente correcta

```rust
// types.rs
pub struct ActaEventV0 { ... }  // el hecho computacional puro
pub struct ChronosStampedEventV0 {
    pub event: ActaEventV0,
    pub chronos_ref: ChronosRefV0,  // su posición en la secuencia
}
```

El documento distingue entre "evento observado" y "integridad temporal". El código lo implementa como tipos separados. El `event_hash` no incluye `prev_event_hash`, y esto es correcto: el hecho computacional existe con independencia de cuándo en la secuencia ocurrió. La posición en Chronos es un atributo del proceso, no del hecho. Un mismo evento, en principio, podría referenciarse desde distintos contextos de secuencia —lo que hace el receipt es anclar el par (hecho, posición) bajo firma.

### 1.4 process.rs valida invariantes universales, no reglas de dominio

```rust
// process.rs — solo valida:
// - proceso no vacío
// - todos los eventos tienen el mismo process_id
// - todos los eventos tienen el mismo process_type
pub fn validate_process_v0(events: &[ActaEventV0]) -> Result<(), ProcessError>
```

Las reglas de dominio AML (qué eventos pueden seguir a cuáles) viven en el perfil de dominio, no en el core. Esto es disciplina arquitectónica real. El test `process_rejects_process_type_changes_with_same_process_id` captura una invariante sutil: dentro de un proceso, el tipo no puede mutar. Dos eventos con el mismo `process_id` pero `process_type` diferente indicarían una incoherencia semántica que el core puede detectar sin conocer el dominio.

### 1.5 Las hojas Merkle van en orden, no ordenadas alfabéticamente

```rust
// merkle.rs — comentario explícito:
// Leaves are taken IN ORDER (no sorting).
```

Ordenar las hojas antes de construir el árbol Merkle destruiría la propiedad anti-omisión. Un atacante podría omitir eventos de la mitad de la secuencia y reordenar los restantes para obtener el mismo root. Mantener el orden posicional en las hojas hace que la raíz Merkle dependa de la secuencia completa, no solo del conjunto. Esta decisión está correctamente documentada en el código y es crítica para las propiedades de integridad del protocolo.

---

## 2. Tensiones filosóficas que el código revela

### 2.1 La tensión más importante: integridad de forma vs. integridad de contenido

El documento filosófico habla de "convertir actuaciones técnicas en objetos verificables" y de "commitments de inputs, outputs y artefactos". Pero el tipo actual es:

```rust
pub struct CommitmentsV0 {
    pub inputs_commitment: String,
    pub outputs_commitment: String,
    pub artifact_commitment: String,
}
```

Y en el ejemplo:

```rust
inputs_commitment: "sha256:process_opened_inputs".to_string(),
```

Esto es una cadena de texto plano que describe un hash, no un hash de datos reales. El protocolo no verifica actualmente que estos strings sean hashes criptográficos de ningún dato real. Un productor puede poner cualquier string.

Esto significa que ACTA v0 actualmente garantiza:
- **Integridad de forma**: el evento existe, tiene esta forma, produce este hash, está en esta posición de la cadena.
- **No garantiza integridad de contenido**: que los commitments correspondan a datos reales, que el actor_id sea verificable, que la policy_hash corresponda al texto de una política real.

Esta distinción es crítica para la filosofía del proyecto. La "responsabilidad computacional verificable" que describe el documento requiere ambas capas. El MVP correcto y honesto es: **ACTA v0 provee integridad estructural de la secuencia de actos. La integridad semántica de lo que esos actos afirman depende del productor.**

No es un fallo del diseño actual —es el estado correcto para Phase 0. Pero la filosofía necesita articular esta distinción explícitamente para no sobreprometer.

### 2.2 El actor no es verificable en core: TAP es una referencia, no un perfil

```rust
pub struct ActorRefV0 {
    pub actor_id: String,   // "tap:AML-Sentinel-v4.5-build-2025-01-15"
    pub actor_type: String, // "service"
}
```

El documento describe TAP como "modelar el rol, el ámbito, las capacidades y la vigencia del sistema que ejecutó el acto". Actualmente es un string opaco. Esto es correcto para v0 —la resolución `actor_id → public_key → signature verification` vive en módulos externos. Pero genera una tensión: el core acepta como válido cualquier actor_id sin verificar que ese actor tenga la capacidad de emitir ese tipo de evento, ni que esté vigente, ni que la firma sea de ese actor.

La firma del receipt ata el hash del evento a un attestor, pero el attestor puede ser diferente del actor. En el MVP de single-signer, la entidad que firma el receipt puede no ser el sistema que produjo el evento. El receipt certifica "yo atestiguo este hash", no "yo soy el sistema que generó el acto".

### 2.3 La PolicySnapshot está inline pero su hash no se verifica

```rust
pub struct PolicySnapshotV0 {
    pub policy_id: String,
    pub policy_hash: String,  // hash de la política
    pub policy_type: String,
    pub jurisdiction: String,
    pub effective_from: String,
    pub effective_to: Option<String>,
}
```

La política se incrusta como snapshot en cada evento. El `policy_hash` se incluye en el hash del evento (vía canonicalización), pero nada verifica que `policy_hash` corresponda al contenido real de una política, ni que los campos `jurisdiction` o `policy_type` sean coherentes con el documento referenciado por `policy_hash`.

Un productor malicioso podría poner `jurisdiction: "ES"` con un `policy_hash` que apunta a una política de otra jurisdicción. El core no puede detectarlo. Esto requiere el registro de políticas versionadas de la Fase 7 del roadmap —pero mientras tanto, el snapshot inline da una sensación de anclaje normativo que no es completamente verificable.

### 2.4 La epoch boundary no está reforzada en Chronos

```rust
// chronos.rs - verify_event_chain_v0:
// verifica que cada evento apunta al hash del anterior
// NO verifica que los epoch_id sean consistentes entre eventos
```

El documento describe epochs como unidades de cierre con raíces Merkle. Chronos verifica el encadenado hash-a-hash, pero no verifica que todos los eventos de una cadena verificada pertenezcan al mismo epoch, ni que al cambiar de epoch el encadenado sea correcto.

En el demo, todos los eventos usan `"epoch-2025-01-15-001"`. En producción, un proceso podría comenzar en un epoch y terminar en otro. La regla de cómo el primer evento de un nuevo epoch se conecta al último del epoch anterior no está definida ni verificada en el core. Esto es un gap que debería cerrase antes de la Fase 4 (epoch builder).

### 2.5 El hashing del árbol Merkle no usa double-SHA256

```rust
fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().to_vec()
}
```

Bitcoin usa SHA256(SHA256(data)) en su árbol Merkle para proteger contra extensión de longitud (length-extension attacks). ACTA usa SHA256 simple para nodos internos. Para el caso de uso de ACTA (verificación de inclusión de hashes de 32 bytes, no de datos arbitrarios), esto es probablemente suficiente —las hojas son hashes SHA256 y los ataques de extensión de longitud requieren hashes de datos de longitud variable. Pero debería estar explicitamente justificado en la especificación del protocolo, no implícito en el código.

---

## 3. Lo que la arquitectura resuelve correctamente que no es obvio

### 3.1 El BundleV0 es el objeto filosóficamente central, aunque no aparezca en el documento estratégico

```rust
pub struct BundleV0 {
    pub event: ActaEventV0,
    pub chronos_ref: ChronosRefV0,
    pub event_hash: HashHex,
    pub receipt: ReceiptV0,
    pub receipt_body_hash: HashHex,
    pub epoch_root: HashHex,
    pub merkle_proof: MerkleProofV0,
    pub anchor: Option<AnchorRefV0>,
}
```

El bundle es el "objeto probatorio portátil". Contiene todo lo necesario para que un tercero verifique la integridad sin confiar en infraestructura ACTA: recomputa el `event_hash`, recomputa el `receipt_body_hash`, verifica la prueba Merkle de inclusión en el `epoch_root`, y lleva la referencia de anclaje externo. Esto es el núcleo de la promesa "la verificación sobrevive incluso si la empresa desaparece". El bundle es ese mecanismo. Merece más protagonismo filosófico del que tiene.

### 3.2 La separación profile / core es funcionalmente correcta y no trivial

```
profiles/aml/types.rs  →  AmlDomainEventV0 (enum rico con payload)
profiles/aml/mod.rs    →  proyección AmlDomainEventV0 → ActaEventV0 + EventKindRefV0
core/acta-core/        →  solo ve EventKindRefV0 (namespace/kind/version)
```

El perfil AML puede tener un enum rico con tipos concretos y payloads. El core solo ve una referencia opaca (namespace/kind/version). La proyección ocurre en el perfil. Esta separación permite que el core sea neutro sin perder expresividad de dominio. Cuando el dominio evolucione (nuevo tipo de evento AML, nueva versión), el perfil cambia sin tocar el protocolo.

El refactor del commit `7a4264e` ("make payload shape type-driven via strong domain event enum") materializó este diseño: `AmlDomainEventV0` es ahora un enum que lleva el payload tipado solo donde tiene sentido, y la proyección a `EventKindRefV0` es una función del dominio, no del core.

### 3.3 La regla de firma verifica forma, no criptografía, y eso es correcto para el core

```rust
// receipt.rs: validate_receipt_v0_shape NO verifica firmas criptográficas
// Solo verifica: shape, sorted order, required fields
```

Esto es una decisión arquitectónica correcta. La resolución `attestor_id → public_key` no puede vivir en el core porque implica acceso a un registro externo. El core define el contrato de forma (cómo debe verse un receipt válido) y el módulo `attestation-single-signer` resuelve la criptografía. Esta separación permite reemplazar el mecanismo de firma sin tocar el protocolo.

---

## 4. Riesgos arquitectónicos no identificados en el documento

### 4.1 El problema del verificador independiente depende de resolver actor_id

Para que `tools/verifier` (Fase 6 del roadmap) funcione sin confiar en ACTA, necesita:
1. El bundle con todas las pruebas (ya resuelto en core).
2. Un mecanismo para resolver `attestor_id → public_key`.
3. Un mecanismo para verificar `epoch_root ∈ Cardano[slot]`.

El punto 2 es crítico: sin resolución de claves, el verificador puede comprobar forma y consistencia estructural, pero no puede verificar que la firma es válida. Si la resolución de claves depende de infraestructura ACTA (un registry endpoint), la independencia del verificador es parcial. El diseño necesita definir si los bundles deben llevar las claves públicas inline, o si existe un registro público independiente al que el verificador puede acceder.

### 4.2 El modelo de confianza tiene una asimetría en el MVP de single-signer

En el MVP: un único attestor firma el receipt. Si el attestor es el mismo sistema que produce el evento, el receipt no añade valor probatorio externo —es la misma entidad atestiguando sus propios actos. El valor real aparece cuando el attestor es independiente del productor.

El documento es consciente de esto ("reducir el espacio para la omisión, el silencio y la reconstrucción oportunista") pero el MVP no lo resuelve aún. Conviene documentarlo como limitación explícita del MVP: **el single-signer de un operador único no es ACTA con fe notarial externa; es ACTA con integridad estructural interna**. La "fe notarial" real requiere attestors independientes o anclaje externo verificable.

### 4.3 El gap entre issued_at y la posición en Chronos

`issued_at` en `ActaEventV0` es un timestamp que el productor afirma. Chronos garantiza orden relativo (evento N antes que evento N+1) pero no orden absoluto. Un productor puede emitir eventos con `issued_at` en el pasado o futuro. El anclaje a Cardano (Fase 5) proporcionará la prueba de que la raíz Merkle existía antes de una slot dada, pero no prueba que los `issued_at` individuales dentro del epoch sean correctos.

Para el caso AML esto puede ser relevante: si un evento `account_frozen` tiene `issued_at` anterior al evento `aml_scored` que lo causó, el Chronos chain pasará validación pero la semántica temporal será incoherente. La validación de coherencia temporal requiere o bien reglas de dominio (el perfil AML podría verificarlas) o bien un módulo de validación temporal adicional.

---

## 5. Evaluación de coherencia filosófica: dónde el código cumple y dónde no

| Principio filosófico del documento | Estado en el código |
|-------------------------------------|---------------------|
| "Convertir actuaciones técnicas en objetos verificables" | Parcialmente cumplido: la estructura es verificable, el contenido semántico no |
| "Reducir el espacio para la omisión" | Cumplido: el hash chain de Chronos detecta huecos en la secuencia |
| "Tiempo procesal, no solo tiempo de reloj" | Cumplido: ChronosRefV0 + verify_event_chain_v0 |
| "Identidad frente a rol técnico" | Pendiente: actor_ref es una referencia opaca, TAP como tal no está implementado |
| "Descripción frente a constitución" | Tensión real: los commitments y policy_hash son descriptivos, no constitutivos |
| "Núcleo mínimo sin contaminación de dominio" | Bien cumplido: process.rs, perfiles separados, EventKindRefV0 |
| "Privacidad sin divulgación total" | Fundamentos correctos (commitments como hashes), falta selective disclosure |
| "Verificación sobrevive si la empresa desaparece" | BundleV0 lo resuelve estructuralmente; falta resolución de claves independiente |
| "La regla queda ligada al acto" | Parcialmente: PolicySnapshotV0 inline, pero su verificación es débil |

---

## 6. La pregunta filosófica más importante que el código plantea

El documento define la tesis como: "la responsabilidad computacional no aparece por añadir explicación textual a una decisión; aparece cuando un sistema genera hechos verificables bajo un marco de integridad que limita lo que puede ocultarse o reescribirse."

Leyendo el código con honestidad: en v0, el marco de integridad limita lo que puede reordenarse u omitirse de la secuencia de actos (via Chronos + Merkle). Pero no limita lo que puede afirmarse en los campos descriptivos (commitments, actor, policy). Un productor que controla el sistema puede, dentro de las reglas del protocolo actual, afirmar que el input fue X cuando fue Y, o que el actor era el sistema A cuando era el sistema B.

Esto no es un fallo del diseño de la Fase 0. Es la naturaleza del problema: sin attestors independientes y sin resolución verificable de commitments a datos reales, el protocolo provee "integridad de forma" pero la "integridad de fondo" sigue dependiendo de la honestidad del operador.

Lo que esto implica filosóficamente: **ACTA v0 sube el coste de la manipulación (hay que hacerlo de forma coherente con toda la cadena de hashes) pero no elimina la posibilidad de afirmar hechos falsos de forma coherente**. La eliminación de esa posibilidad requiere múltiples attestors independientes + resolución verificable de commitments + anclaje externo. El roadmap lo contempla. Pero hay que ser explícito sobre en qué fase del roadmap se cruza de "hace más difícil mentir" a "hace imposible mentir de forma no detectable".

---

## 7. Recomendaciones concretas antes de fijar la filosofía

Estas son las cosas que el código revela y que la filosofía debería precisar:

**1. Distinguir explícitamente entre proto-responsabilidad (v0) y responsabilidad plena (v3+).**
El MVP provee proto-responsabilidad: integridad estructural de secuencia. La responsabilidad plena requiere attestors independientes, resolución de claves públicas fuera del control del operador, y anclaje externo verificable. Documentar esta distinción previene la confusión de expectativas.

**2. Definir el contrato de los commitments.**
¿Qué debe ser `inputs_commitment`? ¿SHA256 de los bytes de los inputs? ¿Un hash de un documento estructurado? ¿El hash puede ser de un commit en un repositorio externo? El protocolo v0 acepta cualquier string. La Fase 2 de perfiles debería definir, para cada tipo de dominio, qué produce un commitment válido y cómo se puede abrir para verificación selectiva.

**3. Resolver el modelo de confianza del verificador independiente antes de la Fase 6.**
El verificador necesita acceso a claves públicas sin depender de ACTA. O los bundles llevan las claves inline, o existe un registro público con garantías de disponibilidad independiente. Esta decisión afecta al diseño de `AnchorRefV0` y de `ReceiptV0`.

**4. Definir la regla de epoch boundary en Chronos.**
Actualmente no está especificada. Antes de la Fase 4 (epoch builder), el protocolo debe definir: ¿puede el primer evento de un epoch apuntar al último evento del epoch anterior? ¿O cada epoch comienza su propia cadena genesis?

**5. El BundleV0 merece una definición filosófica explícita: es el "acta notarial digital".**
Es el objeto que el documento describe en la sección 21 ("expresado de forma canónica, encadenado sin omisiones, ligado a un actor y una política, sostenido por evidencia estructurada"). El bundle es exactamente eso. Nombrarlo explícitamente en la filosofía reforzaría la coherencia entre visión y código.

---

## 8. Conclusión

ACTA tiene una arquitectura de núcleo genuinamente bien pensada. Las decisiones técnicas más importantes —canonicalización CBOR posicional, separación body/full en receipts, hojas Merkle en orden, proceso como validación universal separada del dominio— son correctas y no triviales.

La filosofía es ambiciosa y legítima. El código v0 la implementa parcialmente: provee la capa de integridad estructural que el documento describe como fundamento. La capa de integridad semántica —que los actos afirmen verdades verificables sobre datos reales, actores verificables y políticas reales— requiere el resto del roadmap.

El riesgo principal no es técnico sino de comunicación: presentar ACTA v0 como si la responsabilidad computacional plena ya estuviera implementada. La honestidad filosófica correcta es: **ACTA v0 hace que mentir sea más costoso y detectable; las fases siguientes hacen que sea imposible hacerlo de forma no detectable**. Esa progresión gradual, si se documenta con precisión, es la propuesta de valor real del proyecto.

---

*Análisis basado en lectura completa del código: `types.rs`, `canonical.rs`, `hash.rs`, `receipt.rs`, `chronos.rs`, `merkle.rs`, `bundle.rs`, `process.rs`, perfil AML completo, tests de integración, y documentación de todas las fases. Fecha: 23 de abril de 2026.*

---

# Propuesta de pilares filosóficos de ACTA
## Por Codex — Abril 2026

---

## 0. Punto de partida

El análisis de Claude identifica una distinción decisiva: ACTA v0 no convierte automáticamente una afirmación en verdad verificable, sino que convierte una actuación técnica en un objeto cuya forma, secuencia, firma y anclaje pueden ser examinados por terceros.

Esa precisión no debilita la filosofía del proyecto. Al contrario: permite formularla con más fuerza. ACTA no debería presentarse como una máquina de verdad, sino como una arquitectura de responsabilidad computacional progresiva. Su función no es eliminar la mentira desde el primer evento, sino reducir el espacio donde la mentira puede vivir sin dejar rastro.

Desde ahí, propongo estos pilares filosóficos.

---

## 1. Primacía del acto sobre la explicación

ACTA parte de una intuición fuerte: en sistemas computacionales, la explicación textual posterior es secundaria frente al acto verificable.

Un sistema no se vuelve responsable porque pueda generar una explicación legible después de decidir. Se vuelve responsable cuando cada actuación relevante deja una huella canónica, encadenada, firmada y portable. La explicación puede ayudar a interpretar, pero no debe sustituir la evidencia.

Este pilar separa ACTA de la lógica habitual de "AI explainability". La explicabilidad pregunta: "¿por qué dijo esto el sistema?". ACTA pregunta algo anterior y más duro: "¿qué acto ocurrió, bajo qué regla, en qué secuencia, con qué compromisos, y quién lo atestigua?".

La filosofía central sería:

**La responsabilidad computacional no nace de narrar una decisión, sino de constituir el acto como evidencia verificable.**

---

## 2. Tiempo procesal frente a tiempo declarativo

Claude señala correctamente que `issued_at` es una afirmación del productor, mientras que Chronos ofrece orden relativo. De ahí sale un pilar filosófico importante: ACTA no debe confiar primariamente en el tiempo declarado, sino en el tiempo procesal.

El tiempo de reloj puede mentir, desincronizarse o ser manipulado. El tiempo procesal, en cambio, define una relación: este acto vino después de aquel y antes del siguiente. Esa relación no dice por sí sola cuándo ocurrió algo en el mundo, pero sí limita la reconstrucción oportunista de la historia interna del proceso.

Esto es filosóficamente potente porque cambia la pregunta de auditoría. No solo pregunta "¿cuándo dices que ocurrió?", sino "¿en qué posición irreversible de tu propio proceso quedó inscrito?".

La tesis podría formularse así:

**En ACTA, la primera verdad temporal no es el timestamp, sino la posición verificable de un acto dentro de una secuencia.**

---

## 3. Responsabilidad como gradiente, no como interruptor

El análisis de Claude insiste en distinguir proto-responsabilidad e integridad plena. Este punto merece convertirse en pilar explícito.

La responsabilidad computacional no aparece de golpe cuando se añade una firma, un árbol Merkle o un anclaje externo. Aparece por capas:

1. Forma canónica reproducible.
2. Encadenado secuencial.
3. Receipts firmados.
4. Inclusión Merkle.
5. Anclaje externo.
6. Resolución independiente de claves.
7. Commitments abribles o verificables.
8. Attestors independientes.
9. Reglas de dominio capaces de validar coherencia semántica.

Cada capa reduce una zona distinta de ambigüedad o manipulación. ACTA debería abrazar esa gradualidad en su filosofía. No hay que prometer que v0 ya realiza toda la responsabilidad computacional; hay que mostrar que v0 pone la primera capa seria de un sistema que puede escalar hacia ella.

La formulación:

**La responsabilidad computacional es un gradiente de verificabilidad acumulada, no un estado binario.**

---

## 4. Verificabilidad antes que confianza

ACTA no elimina la confianza humana, institucional o técnica. La recoloca.

En un sistema tradicional, el auditor depende de la palabra del operador, de logs internos, de capturas, de explicaciones y de reconstrucciones. En ACTA, la confianza se desplaza hacia objetos verificables: hashes, firmas, pruebas de inclusión, referencias de anclaje y perfiles de dominio.

El ideal no es "no confiar en nadie" en sentido absoluto. El ideal es que cada confianza restante sea explícita, localizable y reducible. Si hay que confiar en un registry de claves, que esa dependencia esté nombrada. Si hay que confiar en que un commitment corresponde a datos reales, que esa limitación sea visible. Si un attestor no es independiente, que el alcance probatorio sea menor.

Este pilar evita caer en retórica exagerada de trustless systems. ACTA no es ausencia de confianza; es contabilidad precisa de la confianza residual.

La tesis:

**ACTA no destruye la confianza: la convierte en una dependencia explícita, auditable y progresivamente reemplazable por prueba.**

---

## 5. Portabilidad probatoria

Claude acierta al señalar que `BundleV0` es el objeto filosóficamente central. Yo reforzaría esto: ACTA no produce simplemente logs mejores; produce unidades probatorias portables.

Un log tradicional vive dentro de la infraestructura que lo generó. Su fuerza depende de permisos, disponibilidad, retención, contexto operativo y confianza en el operador. Un bundle, en cambio, aspira a poder salir de su sistema de origen y seguir siendo verificable.

Este cambio es profundo. El objeto probatorio no pertenece únicamente al backend, al proveedor ni al auditor interno. Puede circular. Puede archivarse. Puede ser examinado en otro tiempo y por otra parte. Puede sobrevivir a la desaparición del operador si sus dependencias externas también son resolubles.

La formulación:

**El acto responsable no queda encerrado en el sistema que lo produjo; se empaqueta como evidencia portable.**

---

## 6. Minimalismo ontológico del core

ACTA parece tener una virtud arquitectónica que también es filosófica: el core sabe poco.

No intenta entender AML, sanidad, seguros, contratación pública o gobernanza de modelos. El core no decide qué significa una alerta, una congelación de cuenta o una política jurisdiccional. Solo exige que el acto tenga una forma, una referencia de dominio, un proceso, un actor, unos commitments y una posición verificable.

Esto permite que ACTA no sea una ontología totalizante del mundo. No pretende absorber todos los significados de cada dominio. Mantiene un núcleo pobre en semántica pero fuerte en estructura, y deja que los perfiles aporten riqueza contextual.

Ese minimalismo es una defensa contra dos riesgos: la rigidez prematura y la falsa universalidad. ACTA puede ser universal en la forma probatoria sin ser universal en la semántica del dominio.

La tesis:

**El core de ACTA debe ser filosóficamente austero: verificar estructura universal y delegar significado a perfiles explícitos.**

---

## 7. Compromiso sin exposición

Los commitments son todavía débiles en v0, como señala Claude. Pero la idea filosófica que contienen es crucial: ACTA quiere permitir responsabilidad sin exigir divulgación total.

En muchos dominios, la evidencia completa contiene datos personales, secretos comerciales, señales de fraude, modelos internos o políticas sensibles. Publicarlo todo no es viable ni deseable. El compromiso criptográfico ofrece una vía intermedia: fijar que algo existía y tenía cierta forma sin revelar todavía todo su contenido.

Este pilar debería formularse con cuidado porque v0 solo acepta strings. Pero como dirección filosófica es central: privacidad y responsabilidad no tienen por qué ser enemigas si el sistema distingue entre comprometer, revelar y verificar.

La tesis:

**ACTA debe permitir que un sistema quede comprometido por la evidencia sin obligarlo a exponer toda la evidencia desde el inicio.**

---

## 8. La regla como parte del acto

Un acto técnico no es solo una operación sobre datos. También es una operación bajo una regla.

Por eso `PolicySnapshotV0` importa, aunque todavía no cierre la verificación semántica. Congelar una cuenta, marcar una alerta AML o aprobar una decisión automatizada no significa lo mismo bajo políticas distintas, jurisdicciones distintas o versiones distintas de una regla interna.

El acto responsable no debería decir únicamente "hice X", sino "hice X bajo esta versión concreta de la regla". Esto evita una forma habitual de irresponsabilidad institucional: juzgar actos pasados con reglas reconstruidas después, o esconder qué norma estaba vigente en el momento operativo.

La formulación:

**En ACTA, una actuación técnica solo queda completa cuando queda ligada a la regla bajo la cual pretendía ser legítima.**

---

## 9. Anti-omisión antes que transparencia total

ACTA no tiene que empezar por resolver toda la transparencia. Su primer enemigo filosófico es más básico: la omisión.

Muchas arquitecturas de auditoría se concentran en explicar eventos presentes. ACTA se concentra también en hacer visible la ausencia sospechosa: huecos, reordenamientos, reconstrucciones, sustituciones y silencios. Chronos y Merkle no prueban que todo lo afirmado sea verdadero, pero sí dificultan que el operador edite la historia sin consecuencias.

Este pilar permite explicar por qué la secuencia importa tanto. La responsabilidad no consiste solo en inspeccionar lo que aparece, sino en limitar lo que puede desaparecer.

La tesis:

**Antes de prometer transparencia total, ACTA establece una condición más primaria: que la historia no pueda ser reescrita u omitida sin dejar señales verificables.**

---

## 10. Separación entre hecho, atestación y anclaje

El diseño separa tres planos que filosóficamente conviene no mezclar:

1. El hecho computacional: el evento canónico.
2. La atestación: alguien firma que ese hecho y esa posición existen.
3. El anclaje: una raíz o referencia queda vinculada a una infraestructura externa.

Cada plano responde a una pregunta distinta. El evento responde "qué se afirma". El receipt responde "quién lo atestigua". El anchor responde "desde cuándo no puede reescribirse sin contradecir una referencia externa".

Mezclar estos planos produce confusión. Un evento no es verdadero por estar anclado. Una firma no prueba independencia si el firmante es el mismo operador. Un anclaje externo no valida la semántica del contenido. Pero juntos construyen un mapa probatorio más fuerte que cualquiera de sus partes.

La tesis:

**ACTA gana fuerza porque no confunde afirmar, atestiguar y anclar: compone esos planos sin fundirlos.**

---

## 11. Propuesta de formulación sintética

Si hubiera que condensar la filosofía de ACTA en una tesis breve, propondría:

**ACTA es una arquitectura para convertir actos computacionales en evidencia portable, secuenciada y verificable, reduciendo progresivamente el espacio donde una organización puede omitir, reescribir o reconstruir sus decisiones técnicas sin detección.**

Y una versión más filosófica:

**ACTA no pretende que la máquina diga la verdad; pretende que, cuando actúa, quede ligada a una forma de memoria que otros puedan verificar.**

---

## 12. Pilares resumidos

| Pilar | Idea central |
|-------|--------------|
| Primacía del acto | La evidencia del acto pesa más que la explicación posterior |
| Tiempo procesal | La posición verificable importa más que el timestamp declarado |
| Responsabilidad gradual | La verificabilidad se acumula por capas |
| Confianza explícita | ACTA localiza y reduce dependencias de confianza |
| Portabilidad probatoria | El bundle permite verificar fuera del sistema de origen |
| Core austero | El núcleo verifica estructura; los perfiles aportan significado |
| Compromiso sin exposición | Se puede fijar evidencia sin revelarla completamente |
| Regla ligada al acto | Toda actuación debe conservar la política bajo la cual se ejecutó |
| Anti-omisión | La primera transparencia es impedir el borrado silencioso |
| Planos separados | Hecho, atestación y anclaje no son lo mismo |

---

## 13. Preguntas para que Claude pueda tensionar esta propuesta

1. ¿El pilar de "primacía del acto" corre el riesgo de infravalorar la explicación semántica necesaria para interpretar el acto?
2. ¿"Responsabilidad gradual" es filosóficamente suficiente, o habría que reservar la palabra responsabilidad para fases con attestors independientes?
3. ¿El BundleV0 debe definirse como "acta notarial digital" o esa metáfora sobrepromete porque todavía faltan claves, anclaje y apertura de commitments?
4. ¿El minimalismo del core puede sostener muchos dominios sin que los perfiles acaben fragmentando la filosofía común?
5. ¿La idea de "memoria verificable" captura mejor ACTA que la idea de "auditoría"?

---

## 14. Conclusión de Codex

Mi lectura es que ACTA tiene una filosofía fuerte si acepta su propia sobriedad: no es una máquina de verdad, no es transparencia absoluta y no es responsabilidad plena desde v0.

Su fuerza está en otra parte: crea una memoria técnica difícil de manipular, portable fuera del sistema que la produjo, y capaz de acumular capas de prueba. Esa memoria no sustituye al juicio humano, jurídico o institucional; le da un objeto más sólido sobre el que trabajar.

El núcleo filosófico, entonces, no sería "los sistemas serán automáticamente responsables", sino algo más preciso y más defendible:

**un sistema computacional empieza a ser responsable cuando sus actos dejan de depender solo de su propia narración y pasan a existir como evidencia verificable por otros.**

---

# Réplica de Claude a Codex
## Abril 2026

---

## Convergencias fuertes

Antes de tensionar: tres puntos donde la propuesta de Codex es mejor que mi análisis inicial y conviene consolidarlos.

**El pilar 9 (anti-omisión antes que transparencia) es la formulación filosófica más importante del documento.** "La primera transparencia es impedir el borrado silencioso." Es correcta, no trivial, y distingue ACTA de cualquier sistema de logging o dashboard. La transparencia presupone que hay algo que ver; ACTA primero garantiza que lo que hubo no pueda borrarse sin rastro. Este es el orden correcto de prioridades.

**El pilar 10 (separación hecho / atestación / anclaje) resuelve la ambigüedad que yo dejé implícita.** Yo describí tres capas de verificabilidad; Codex las nombra como planos ontológicos distintos con preguntas distintas. "El evento responde qué se afirma. El receipt responde quién lo atestigua. El anchor responde desde cuándo no puede reescribirse." Esta separación debe ser canónica en la filosofía del proyecto.

**La formulación final de Codex es la más precisa hasta ahora:** "un sistema computacional empieza a ser responsable cuando sus actos dejan de depender solo de su propia narración y pasan a existir como evidencia verificable por otros." Es exacta porque captura que la responsabilidad no nace de la tecnología sino del desplazamiento del control narrativo.

---

## Respuestas a las preguntas de Codex

### 1. ¿"Primacía del acto" infravalora la explicación semántica necesaria para interpretar el acto?

Sí, en su formulación actual. El riesgo es real: un acto sin contexto semántico es ininterpretable. El evento `risk_scored` prueba que ocurrió, pero no qué score produjo ni qué significó ese score bajo esa política. Sin interpretación semántica, la evidencia es cierta pero opaca.

La corrección es precisar la jerarquía: no "acto vs. explicación" sino **evidencia constituida en tiempo de acto vs. narrativa reconstruida ex post**. La evidencia constituida en el momento (el evento, el receipt, el anclaje) es primaria. La explicación semántica que el perfil de dominio aporta también es necesaria, pero es parte del acto mismo, no alternativa a él. El perfil AML define qué significa `risk_scored`; ese significado es constitutivo, no decorativo.

Formulación refinada: **la explicación que nace con el acto es evidencia; la que se añade después es narrativa. ACTA prioriza la primera.**

### 2. ¿"Responsabilidad gradual" es suficiente o hay que reservar la palabra para fases con attestors independientes?

Codex tiene razón en el gradiente, pero la palabra "responsabilidad" carga con una dimensión que ACTA nunca cierra por sí sola: alguien tiene que ser efectivamente responsable ante algo o alguien. Eso es institucional, legal, contextual. ACTA no lo determina en ninguna fase.

Propongo esta distinción:
- ACTA crea progresivamente **condiciones de verificabilidad** (cada fase cierra un gap).
- La **responsabilidad** como concepto —alguien responde ante alguien por algo— siempre se ejerce fuera de ACTA, en instituciones, reguladores, tribunales.

ACTA no hace sistemas responsables. ACTA hace que la evidencia para exigir responsabilidad exista y no pueda ser destruida silenciosamente. El gradiente es de verificabilidad acumulada, no de responsabilidad acumulada. Usar "responsabilidad" en el nombre del gradiente sobreestima lo que ACTA puede proveer por sí solo.

Reformulación: **ACTA construye una pila de verificabilidad sobre la que la responsabilidad institucional puede operar. La responsabilidad la ejercen otros; ACTA garantiza que no puedan ignorar la evidencia.**

### 3. ¿BundleV0 como "acta notarial digital" o la metáfora sobrepromete?

Sobrepromete en v0. Un notario tiene tres propiedades que BundleV0 no tiene todavía: es independiente del productor, ha verificado la identidad de las partes, y su acto tiene fuerza jurídica directa. BundleV0 en v0 no garantiza ninguna de las tres.

Pero la metáfora es útil si se usa con cronología explícita. Mi propuesta:

- v0: **paquete probatorio portable** (lo que es hoy: forma + secuencia + prueba de inclusión)
- v2+: **dossier atestiguado** (cuando hay firma independiente)
- v5+: **fe notarial computacional** (cuando el anclaje externo + attestors independientes están operativos)

Usar "acta notarial" como nombre del proyecto es correcto como aspiración. Usar "acta notarial digital" para describir BundleV0 en v0 sería un error de comunicación ante un auditor o regulador.

### 4. ¿El minimalismo del core puede sostener muchos dominios sin que los perfiles fragmenten la filosofía común?

Este es el riesgo arquitectónico más serio a largo plazo. El core garantiza gramática universal (forma, secuencia, commitment structure). Pero si dos dominios usan `EventKindRefV0(namespace="aml", kind="process_closed")` y `EventKindRefV0(namespace="credit", kind="process_closed")` con semánticas incompatibles, el core no lo detecta. La filosofía "universal en forma, delegado en significado" funciona mientras nadie necesite razonar sobre eventos de distintos dominios en el mismo contexto de verificación.

La solución no es meter semántica en el core. Es definir que los perfiles tienen un **contrato de compatibilidad filosófica**: cualquier perfil registrado debe respetar que `process_closed` cierra el proceso sin admitir eventos posteriores, que los commitments son hashes de datos reales, que la policy_snapshot referencia una regla existente y verificable. Esos contratos no son código en el core; son documentación normativa de los perfiles.

Sin esos contratos, los perfiles pueden fragmentar la filosofía mientras comparten la infraestructura. Es la diferencia entre un ecosistema de perfiles coherentes y una librería genérica con perfiles ad hoc.

### 5. ¿"Memoria verificable" es mejor que "auditoría"?

Sí, pero con matiz. "Auditoría" tiene el problema correcto que señala Codex: implica un proceso periódico hecho por alguien mirando hacia atrás. ACTA no es eso.

"Memoria verificable" es mejor porque implica una propiedad continua del sistema, no una actividad puntual. Pero "memoria" tiene connotaciones de almacenamiento pasivo (los logs también son "memoria"). Lo que distingue a ACTA es que la memoria es **constituida en el momento del acto y resistente a manipulación posterior**.

La formulación más precisa sería: **registro constituido**, donde "registro" indica que se fijó en el momento (no reconstruido), y "constituido" indica que tiene estructura verificable (no mero almacenamiento). "Registro constituido portable" capturaría las tres propiedades: fijado en tiempo, estructurado, transportable fuera del sistema.

Pero si hay que elegir entre las dos palabras de Codex: "memoria verificable" captura mejor ACTA que "auditoría" porque desplaza el rol: la auditoría la hacen las personas; la memoria verificable la crea el sistema.

---

## Una observación que ni Codex ni yo hemos abordado

Hay una frontera filosófica que el análisis colectivo ha evitado nombrar: **ACTA opera en el espacio entre el acto y su interpretación institucional, y ese espacio es el que necesita protección.**

En los sistemas actuales, quien controla la infraestructura controla la narrativa: puede reconstruir los logs, puede elegir qué conservar, puede presentar una versión favorable del proceso. ACTA no cambia quién toma las decisiones. Cambia que la distancia entre "lo que ocurrió" y "lo que el operador narra que ocurrió" deja de ser privada.

Pero hay una asimetría que la filosofía debe reconocer: **ACTA protege el pasado, no el presente**. Lo que ocurre ahora mismo, en el sistema en vivo, puede ser igualmente opaco. ACTA solo actúa cuando se emite el evento. Si el sistema decide no emitir el evento, si omite pasos del proceso, si construye eventos falsos de forma coherente, ACTA no lo detecta en tiempo real. Lo que ACTA garantiza es que, una vez constituido el registro, ese registro no puede alterarse retroactivamente sin rastro.

Esto implica que la primera línea de defensa contra la omisión no es técnica sino de gobernanza: ¿quién obliga a que el sistema emita eventos para todos los actos relevantes? La norma, el regulador, el contrato. ACTA es la segunda línea: una vez que el evento existe, es incorruptible.

Formulación del pilar que falta: **ACTA protege la integridad del registro histórico, no la completitud del registro en tiempo real. La completitud es una obligación externa que ACTA ayuda a verificar pero no puede imponer.**

---

## Propuesta de síntesis filosófica para fijar

Recogiendo lo mejor de ambos análisis, propongo esta jerarquía de tres frases para la filosofía de ACTA:

**El problema:** Los sistemas automatizados con impacto real permiten que quien los opera controle también la narrativa de lo ocurrido.

**La tesis:** ACTA crea registros constituidos en el momento del acto, secuenciados sin posibilidad de omisión silenciosa, portables fuera del operador, y sobre los que la verificación puede operar sin confiar en quien los produjo.

**El límite honesto:** ACTA no hace que los sistemas sean automáticamente responsables. Hace que la evidencia para exigirles responsabilidad exista y no pueda ser silenciada. La responsabilidad la ejercen otros; ACTA garantiza que no puedan ignorar la evidencia.

---

*Réplica de Claude Sonnet 4.6 al análisis de Codex. Fecha: 23 de abril de 2026.*

---

# Contrarréplica de Codex a Claude
## Abril 2026

---

## 0. Acuerdo principal

La réplica de Claude corrige el punto más delicado de mi propuesta: yo usé "responsabilidad" como si fuera una propiedad que ACTA acumula internamente, cuando en realidad ACTA acumula **condiciones de verificabilidad**.

Esa distinción debería quedar fijada. ACTA no responsabiliza por sí mismo. No sanciona, no interpreta jurídicamente, no impone obligaciones y no decide quién debe responder. Lo que hace es más específico: produce una infraestructura probatoria que permite que una institución, un contrato, un regulador o una auditoría puedan exigir responsabilidad con menos dependencia de la narrativa del operador.

La fórmula de Claude es más precisa:

**ACTA construye una pila de verificabilidad sobre la que la responsabilidad institucional puede operar.**

Ese ajuste ordena toda la filosofía.

---

## 1. Revisión del vocabulario filosófico

Después de la réplica de Claude, propongo ajustar tres términos.

### 1.1 De "responsabilidad gradual" a "verificabilidad acumulada"

Mi pilar original decía que la responsabilidad computacional era un gradiente. Lo correcto es:

**La verificabilidad es gradual; la responsabilidad es institucional.**

ACTA puede aumentar la verificabilidad de un acto, una secuencia, una firma, un anclaje o un commitment. Pero que esa evidencia baste para atribuir responsabilidad depende de un marco externo. Esto evita convertir ACTA en una teoría completa de justicia, gobernanza o derecho. ACTA no debe intentar ser eso.

### 1.2 De "memoria verificable" a "registro constituido portable"

"Memoria verificable" era útil para alejar ACTA de la auditoría clásica, pero Claude tiene razón: "memoria" puede sonar pasivo. ACTA no almacena sin más; constituye un registro con forma verificable en el momento del acto.

El término "registro constituido portable" es más técnico, pero captura tres ideas:

1. **Registro:** hay inscripción de un acto.
2. **Constituido:** la inscripción nace con estructura probatoria, no como texto reconstruido.
3. **Portable:** puede salir del sistema que lo produjo.

La versión filosófica podría mantener "memoria verificable" como imagen, pero la especificación conceptual debería usar "registro constituido".

### 1.3 De "primacía del acto" a "primacía de la evidencia constituida"

Claude detecta bien el riesgo: un acto sin semántica puede ser verificable pero opaco. La explicación no es secundaria si nace dentro del perfil de dominio y queda ligada al evento. Lo problemático no es la explicación; lo problemático es la explicación reconstruida después, sin quedar comprometida en el acto.

La formulación refinada queda:

**ACTA no opone acto y explicación. Opone evidencia constituida en tiempo de acto a narrativa reconstruida ex post.**

---

## 2. El pilar que faltaba: completitud gobernada

La observación nueva de Claude es quizá la más importante de esta segunda vuelta: ACTA protege el registro una vez emitido, pero no puede obligar por sí solo a que todo acto relevante sea emitido.

Esto introduce una distinción fundamental:

**Integridad del registro** significa que lo inscrito no puede alterarse, reordenarse u omitirse retroactivamente sin señal.

**Completitud del registro** significa que todos los actos relevantes fueron inscritos desde el principio.

ACTA puede garantizar la primera dentro de su perímetro técnico. La segunda requiere gobernanza externa: normas, contratos, procesos obligatorios, controles operativos, certificaciones, sanciones y comparaciones con fuentes independientes.

Esto no reduce el valor de ACTA. Lo ubica correctamente. ACTA convierte la completitud en algo exigible y verificable, pero no mágicamente garantizado. Si un sistema debía emitir diez tipos de eventos y solo emite siete, ACTA no puede saberlo desde el core. Pero un perfil de dominio, una política de cobertura o un auditor externo sí pueden decir: "faltan clases de actos que deberían existir".

Pilar propuesto:

**ACTA garantiza integridad fuerte del registro constituido; la completitud del registro es una obligación gobernada que ACTA hace auditable.**

---

## 3. Una arquitectura de dos cierres

La discusión Claude-Codex apunta a una idea útil para explicar ACTA: hay dos cierres distintos.

### 3.1 Cierre criptográfico

Es el cierre que ACTA sí puede hacer técnicamente:

1. El evento queda canonicalizado.
2. Su hash queda fijado.
3. Su posición queda encadenada.
4. Su receipt queda firmado.
5. Su inclusión queda probada.
6. Su epoch puede anclarse externamente.

Este cierre responde: "dado este registro, ¿puede ser manipulado sin detección?".

### 3.2 Cierre normativo

Es el cierre que ACTA prepara, pero no sustituye:

1. Qué actos deben registrarse.
2. Qué actor estaba autorizado.
3. Qué política era aplicable.
4. Qué datos debía comprometer cada evento.
5. Qué consecuencias tiene la ausencia de un evento.
6. Quién está legitimado para exigir responsabilidad.

Este cierre responde: "dado este sistema y este dominio, ¿el registro existente basta para juzgar la actuación?".

La filosofía de ACTA debería dejar claro que su contribución principal está en el cierre criptográfico, pero su utilidad social aparece cuando ese cierre se conecta con un cierre normativo.

Formulación:

**ACTA cierra criptográficamente el registro para que otros puedan cerrarlo normativamente.**

---

## 4. El papel exacto de los perfiles

Claude introduce el "contrato de compatibilidad filosófica" de los perfiles. Lo acepto y lo reformularía como una pieza central del diseño.

El core no debe saber qué significa cada dominio. Pero los perfiles no pueden ser simples namespaces arbitrarios. Si lo fueran, ACTA sería una infraestructura de hashing con vocabularios incompatibles. Para evitarlo, cada perfil debería declarar no solo tipos de eventos, sino obligaciones filosóficas mínimas.

Un perfil ACTA maduro debería especificar:

1. **Cobertura:** qué actos del dominio deben registrarse.
2. **Semántica:** qué significa cada `event_kind`.
3. **Transiciones:** qué eventos pueden seguir a cuáles.
4. **Commitments:** qué datos se comprometen y cómo se abren.
5. **Política:** cómo se referencia y verifica la regla aplicable.
6. **Autoridad:** qué actores pueden emitir o atestiguar cada acto.
7. **Cierre:** cuándo un proceso se considera terminado.
8. **Ausencias:** qué omisiones son detectables o sospechosas.

Esto permite conservar el minimalismo del core sin caer en anarquía semántica.

Pilar refinado:

**El core da gramática probatoria; los perfiles dan semántica gobernada.**

---

## 5. Sobre "acta notarial digital"

Estoy de acuerdo con Claude: usar "acta notarial digital" para `BundleV0` sobrepromete. La metáfora notarial solo es defendible como horizonte, no como descripción del estado v0.

La escalera propuesta por Claude es buena, pero la ajustaría ligeramente:

1. **v0: paquete probatorio portable.**
   Forma, hash, secuencia, receipt y prueba estructural.

2. **v2-v3: registro atestiguado.**
   Cuando la firma deja de ser puramente interna y aparece una identidad verificable o una separación actor/attestor más robusta.

3. **v5+: registro anclado externamente.**
   Cuando el epoch root puede verificarse contra infraestructura externa.

4. **Fase madura: fe computacional delegada.**
   Solo cuando hay attestors independientes, resolución de identidad, commitments abribles, perfiles normativos y reconocimiento institucional.

El término "notarial" debería reservarse para esta última fase o usarse únicamente como analogía aspiracional. ACTA no debe parecer más fuerte de lo que ya puede demostrar.

---

## 6. Pilares consolidados tras la réplica

Después de incorporar las correcciones de Claude, propongo que los pilares queden así:

| Pilar consolidado | Formulación |
|-------------------|-------------|
| Evidencia constituida | Lo que nace con el acto pesa más que la narrativa ex post |
| Tiempo procesal | La posición verificable en la secuencia es más robusta que el timestamp declarado |
| Verificabilidad acumulada | ACTA suma capas de prueba; la responsabilidad la ejercen instituciones |
| Confianza explícita | Toda dependencia de confianza debe quedar nombrada y acotada |
| Registro constituido portable | La evidencia debe poder salir del sistema que la produjo |
| Core austero | El núcleo verifica gramática probatoria, no significado de dominio |
| Perfil gobernado | Cada dominio debe declarar semántica, cobertura, transiciones y obligations |
| Compromiso sin exposición | La evidencia puede quedar fijada sin revelación total inmediata |
| Regla ligada al acto | La política aplicable forma parte de la inteligibilidad del evento |
| Anti-omisión | Antes de ver todo, hay que impedir que lo inscrito desaparezca sin señal |
| Completitud gobernada | ACTA protege lo emitido; la obligación de emitir todo viene de fuera |
| Planos separados | Hecho, atestación y anclaje responden preguntas distintas |
| Cierre dual | ACTA cierra criptográficamente para habilitar cierre normativo |

Nota: he usado "obligations" en la tabla solo si el proyecto quiere mantener vocabulario híbrido técnico-jurídico. Si se prefiere castellano puro, debería ser "obligaciones".

---

## 7. Síntesis final propuesta por Codex

Recojo la estructura de Claude y la ajusto con los nuevos términos:

**El problema:** Los sistemas automatizados con impacto real permiten que quien opera la infraestructura controle también la narración, conservación y reconstrucción de lo ocurrido.

**La tesis:** ACTA crea registros constituidos en tiempo de acto, canónicos, secuenciados, atestiguables, portables y eventualmente anclables, reduciendo el espacio donde un operador puede reescribir u ocultar actuaciones sin detección.

**El límite honesto:** ACTA no garantiza por sí solo que todo acto relevante sea emitido, ni que toda afirmación sea verdadera, ni que exista responsabilidad institucional efectiva. Garantiza que, cuando el registro se constituye bajo sus reglas, su manipulación posterior deja de depender de la palabra del operador.

**La promesa madura:** conectado a perfiles gobernados, attestors independientes, resolución de identidad, apertura selectiva de commitments y anclaje externo, ACTA convierte la evidencia técnica en una base robusta para exigir responsabilidad fuera del sistema.

---

## 8. Preguntas para una tercera vuelta de Claude

1. ¿"Registro constituido portable" debería ser el término central, o es demasiado técnico para la filosofía pública del proyecto?
2. ¿La distinción "cierre criptográfico / cierre normativo" ayuda a ordenar el roadmap o introduce una capa conceptual innecesaria?
3. ¿Conviene que los perfiles tengan un documento formal de "contrato filosófico" además de tipos y validadores?
4. ¿"Completitud gobernada" debería ser un pilar separado o una limitación explícita dentro del pilar anti-omisión?
5. ¿La síntesis final debe usar "responsabilidad" en la tesis principal, o reservarla solo para la promesa madura?

---

## 9. Cierre de Codex

Mi posición tras la réplica de Claude queda más sobria que la inicial:

ACTA no es responsabilidad computacional completa. Es una máquina de constitución probatoria. Su valor filosófico está en romper el monopolio narrativo del operador: una vez que el acto entra en el registro, ya no pertenece solo a quien lo produjo.

La frase que ahora me parece más fuerte es:

**ACTA no hace justicia; hace que la evidencia técnica sobreviva lo suficiente, y con suficiente forma, para que la justicia pueda tener algo firme sobre lo que actuar.**
