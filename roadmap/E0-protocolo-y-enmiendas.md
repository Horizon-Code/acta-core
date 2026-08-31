# E0 — Protocolo de experimento OmegaClaw + enmiendas a la directiva

**Fecha:** 31 de agosto de 2026
**Tipo:** protocolo de ejecución + lista de enmiendas
**Autoridad:** complementa `ACTA — Directiva de construcción (31-ago-2026)`. Donde contradiga
a las Foundations o al Protocol, mandan esos.
**Origen:** tres intercambios de análisis (directiva → crítica → verificación en código →
refinamiento). Todo lo analizable por lectura está agotado; lo que queda es empírico.

## Nota de ubicación en el repositorio

Este documento vive en `roadmap/` porque es material de secuenciación y ejecución, no una
definición vinculante. Conforme a `decisions/ADR-001-repository-authority-levels.md`:

- Las enmiendas E-1..E-8 **no son vinculantes por estar aquí**. Se vuelven vinculantes solo
  cuando se reescriban bajo `constitution/`, `architecture/` o `decisions/`.
- Varias de ellas (E-4, E-6 y el predicado de igualdad) tienen su **forma pendiente de E0**,
  de modo que congelarlas ahora en un ADR violaría la propia regla de la compuerta.
- El texto de la directiva que este documento enmienda está en
  `roadmap/directiva-construccion-2026-08-31.md`, incorporado íntegro y literal (T1).
- El entregable de E0 tiene su esqueleto en `research/E0-resultados.md`.

### Resolución de las referencias cruzadas

Contra `roadmap/directiva-construccion-2026-08-31.md` resuelven todas menos dos:

| Referencia | Dónde aparece aquí | Resuelve contra |
|---|---|---|
| §1 | E-6 | Directiva §1, *La decisión que ordena todo lo demás* |
| §3 | E-1, E-3 | Directiva §3, *Especificación del informe de confianza residual* (nueve códigos base) |
| §5.1 | §1.1 paso 4 | Directiva §5.1, *El sistema objetivo: OmegaClaw-Core* |
| §5.2 | E-5, Anexo A.2 | Directiva §5.2, *El testigo ya existe como tipo: ETV* |
| §5.8 | Anexo A.3 | Directiva §5.8, *Riesgo legal del sustrato* |
| §6 | Parte 3 | Directiva §6, *Deuda documental* |
| §8 | §1.0 | Directiva §8, *Criterios de hecho* |
| §9 | E-8, Parte 3 | Directiva §9, *La pregunta que decide el proyecto* |
| §4.2 | E-6 (Artefacto 2, sin citar el §) | Directiva §4.2, *Hyperon — el techo* |

**Las dos que no resuelven contra la Directiva**, porque apuntan a un tercer documento:

| Referencia | Dónde aparece aquí | Apunta a |
|---|---|---|
| §1.2 | E-3, «la escala del §1.2 hecha operativa» | `acta-estado-consolidado.md` §1.2 — la escala de vocabulario. La Directiva la cita de segunda mano en su §1.1 («La escala de vocabulario del §1.2 se reinterpreta»); su propio §1.2 es *Forma comercial* |
| §3.1 | E-7, «decisión 3 del §3.1» | `acta-estado-consolidado.md` §3.1 — las decisiones de canonicalización. La Directiva cita de ahí las decisiones 1 y 2 en su §5.7 |

`acta-estado-consolidado.md` **no está en el repositorio**. La Directiva declara complementarlo
(«describe *qué hay*; este documento describe *qué construir y en qué orden*») y depende de él
en §3.1, §6.1, §6.7, §8.1 y §11. Es deuda documental abierta, anotada en
`research/open-questions.md`.

---

## Parte 1 — E0: el experimento compuerta

### 1.0 Qué es y por qué bloquea

E0 no es una verificación previa a C2: es una **compuerta de decisión**. Sus cuatro capturas
determinan, respectivamente: el predicado de igualdad del verificador, la forma del lockfile
de importaciones, la naturaleza de `TR-CHAIN-MEDIATED` en el informe, y la viabilidad del
compromiso de resultados. **No se escribe una línea de C2 hasta que las cuatro capturas tienen
su decisión derivada documentada.**

Entra como fila propia en la tabla de criterios de hecho (§8 de la directiva):

| Bloque | Criterio de cierre |
|---|---|
| **E0** Experimento | Las cuatro capturas están documentadas y cada una tiene su decisión derivada escrita |

### 1.1 Preparación

1. Clonar `github.com/asi-alliance/OmegaClaw-Core` y **anotar el commit exacto** del clon.
2. Levantar en Docker según su documentación, con un proveedor LLM configurado.
3. Anotar también: commit de PeTTa instalado, versión de SWI-Prolog, versión de Python.
   (Estos datos son parte del resultado: son la primera muestra del problema de procedencia
   del cierre de importaciones.)
4. Localizar el punto de registro: el paso 5 del bucle de turno (eval de skills). Instrumentar
   el logging del **string exacto** que entra en la skill `metta` y del **string exacto** que
   devuelve, antes de que se ensamble en el contexto del LLM. Si el paso 5 no ofrece punto de
   intercepción, documentarlo: eso decide plugin vs canal `wschat` intermediario (aviso ya
   presente en §5.1 de la directiva).

Todo lo que sigue se registra en crudo. Nada de resúmenes: strings completos, con timestamps.

### 1.2 Captura 1 — Estabilidad del collapse

**Procedimiento.** Lanzar el mismo `(|- premisa1 premisa2)` con dos premisas conocidas:
- dos veces en la misma sesión,
- una tercera vez tras reiniciar el contenedor.

**Registrar.** Los tres strings de salida completos. Además, construir un caso donde dos
reglas deriven **la misma conclusión con valores de verdad distintos** y observar si
`unique-atom` deja una o las dos (eso define qué es "el resultado").

**Decisión que resuelve.** El predicado de verificación de recomputación:
- Si el orden es estable dentro de un motor fijado → igualdad de serialización canónica.
- Si el orden varía → igualdad de conjuntos, o pertenencia de la conclusión al conjunto
  recomputado.
- Si sobreviven duplicados con TV distinto → el predicado compara pares (conclusión, TV), no
  conclusiones.

### 1.3 Captura 2 — Cierre de importaciones: ¿estático o dinámico?

**Procedimiento.**
1. Antes de arrancar: enumerar desde disco todos los `.metta` que `lib_omegaclaw.metta`
   importa (directa y transitivamente), con hash SHA-256 de cada uno.
2. Arrancar y observar **cuándo** se ejecuta `!(git-import! "...petta_lib_chromadb.git")`:
   en carga o perezosamente en el primer uso.
3. Tras una sesión de trabajo, volver a hashear los mismos ficheros y comprobar que ninguno
   cambió durante la ejecución.
4. Para cada fichero cargado desde la instalación de PeTTa (`lib_patrick`, `lib_llm`,
   `lib_vector`, `lib_combinatorics`, `lib_he`): intentar determinar su procedencia (commit
   del clon o versión pip) **desde el proceso en ejecución**. Anotar si es posible o no.

**Decisión que resuelve.** La forma del lockfile:
- Cierre determinable estáticamente → manifiesto de pares (ruta, hash) generado antes de la
  ejecución, con `git-import!` resuelto a commit.
- Carga perezosa o cierre no enumerable → hay que instrumentar el cargador de PeTTa, y el
  lockfile se captura en runtime.
- Procedencia no recuperable → el manifiesto da **integridad sin procedencia** (el fichero no
  cambió, pero no consta cuál era) y eso se declara como límite en el informe.

### 1.4 Captura 3 — Proporción fiel/libre en cadena real

**Procedimiento.** Plantear al agente una tarea que exija razonamiento en **dos saltos** de
inferencia como mínimo (la conclusión del salto 1 es insumo necesario del salto 2). Registrar:
- el string de salida del salto 1 (la conclusión comprometible),
- el string de entrada del salto 2 (las premisas que el LLM decidió arrastrar).

Comparar **byte a byte** cada premisa del salto 2 contra la conclusión del salto 1. Repetir
con 3–5 tareas distintas para tener algo parecido a una proporción.

**Registrar.** Por cada eslabón: coincidencia literal sí/no, y si no, el par de strings para
ver el tipo de reescritura (paráfrasis, resumen, traducción de formato, invención).

**Decisión que resuelve.** La naturaleza de `TR-CHAIN-MEDIATED` en el informe:
- Proporción fiel alta → línea excepcional; el rediseño de C2 puede prometer auditoría
  eslabón a eslabón con mayoría de eslabones fieles.
- Proporción fiel baja o paráfrasis sistemática → `TR-CHAIN-MEDIATED` es el paisaje por
  defecto y C2 se cuenta desde ahí (sigue siendo el producto: el informe diciendo la verdad
  sobre una cadena mediada).

### 1.5 Captura 4 — Truncamiento de la salida

**Procedimiento.** Provocar una inferencia cuyo resultado sea un conjunto grande (decenas de
conclusiones). Seguir el string por las tres etapas: salida de `(repr (swrite (eval $code)))`,
ensamblado en `LAST_SKILL_USE_RESULTS`, y contexto final enviado al LLM. Buscar cortes en
cualquiera de las tres.

**Registrar.** Longitud del string en cada etapa; si hay corte, en cuál.

**Decisión que resuelve.** Si la salida puede truncarse, **comprometer el string que ve el
LLM es comprometer una conclusión mutilada** y la recomputación no cierra nunca por un motivo
ajeno a la semántica. En ese caso el punto de captura para el compromiso debe ser anterior al
truncamiento (la salida cruda del eval), y el string truncado del contexto LLM se registra
como artefacto separado. Es el tipo de fallo que no aparece en pruebas pequeñas y arruina la
demo delante de alguien.

### 1.6 Entregable de E0

Un único documento `E0-resultados.md` con: los datos de preparación (commits, versiones), las
cuatro capturas en crudo (o enlaces a los logs), y **una decisión escrita por captura**. Ese
documento es el insumo directo del rediseño de C2 y de la especificación del lockfile.

> En este repositorio, el esqueleto de ese entregable es `research/E0-resultados.md`.

---

## Parte 2 — Enmiendas a la directiva

Cambios acordados en los tres intercambios. Cada uno indica dónde toca.

### E-1. Tabla del informe (§3): cuatro códigos nuevos

| Código | Condición detectada | Texto (marco afirmativo) |
|---|---|---|
| `TR-ENGINE-UNPINNED` | `PolicySnapshotV0` de tipo `inference_ruleset` presente pero sin fijar motor y versión | Las reglas de inferencia están fijadas; la recomputación exige además fijar la semántica del motor que las ejecuta |
| `TR-IMPORT-UNPINNED` | El cierre de importaciones contiene una carga sin revisión fijada (p. ej. `git-import!` sin commit) | El conjunto de reglas declarado carga código externo sin revisión fijada; dos ejecuciones pueden diferir sin constancia |
| `TR-KEY-SELF-ASSERTED` | Claves de verificación transportadas inline en el bundle, sin vinculación externa de identidad | Las firmas son verificables con la clave incluida; la correspondencia entre `attestor_id` y una entidad real requiere un registro externo |
| `TR-CHAIN-MEDIATED` | Cadena de inferencia cuyos eslabones pasan por un mediador no determinista (p. ej. LLM) | Ver E-2: anotación por eslabón, no descargo global |

### E-2. `TR-CHAIN-MEDIATED`: anotación por eslabón

**Predicado de eslabón fiel** (vive en el verificador; comparación de bytes, sin semántica):
una premisa del salto *n+1* es **fiel** si existe una conclusión comprometida C tal que:

1. `bytes(premisa) == bytes(C)`,
2. `C.process_ref == premisa.process_ref` (mismo proceso, no otro del mismo emisor),
3. C **precede** a la premisa en la cadena Chronos (no vale una conclusión aún no derivada).

Toda premisa no fiel es **libre**: afirmación del productor. Una paráfrasis semánticamente
equivalente sale marcada como libre **a propósito**: decidir equivalencia es adjudicación, no
evidencia. La frontera literal/parafraseado cae sobre la frontera evidencia/adjudicación.

**La línea del informe lleva la cifra y el descargo juntos, inseparables:**

> N de M premisas coinciden literalmente con conclusiones previamente comprometidas de este
> proceso; la selección de qué conclusiones se realimentan es del productor y este dossier no
> la acota.

Razón del descargo pegado: la auditoría por eslabón detecta **distorsión**, no **selección**.
Un agente puede arrastrar todo con literalidad perfecta y haber descartado lo que le
contradecía (pilar de completitud gobernada: ACTA protege lo emitido; la obligación de emitir
todo viene de fuera). Una cifra alta sin descargo invita a la sobrelectura que el marco
afirmativo existe para no provocar.

**Nota de documentación:** la métrica es manipulable en la dirección buena — subir la
proporción fiel exige arrastrar conclusiones intactas, que es el comportamiento deseado.
Decirlo explícitamente, porque el primer reflejo de un auditor será preguntar cómo se falsea.

### E-3. Partición del informe en dos registros (§3)

- **Condiciones estructurales de la versión**: lo que v0 no garantiza nunca
  (`TR-TIME-DECLARED` y, mientras duren, `TR-NO-ANCHOR`, `TR-KEYS-DEPENDENT`/inline...).
  Declaradas una vez, arriba. Esta sección **es** la escala del §1.2 hecha operativa: cada
  fase que se cierra borra una línea de aquí.
- **Condiciones detectadas en este dossier**: lo que varía. La señal se preserva porque el
  lector no aprende a saltarse líneas.

### E-4. Lockfile de cierre de importaciones (Profile)

El `PolicySnapshotV0` de tipo `inference_ruleset` no compromete un blob: compromete un
**manifiesto de pares (ruta, hash SHA-256)** — el cierre completo de importaciones, con
`git-import!` resuelto a commit. El hash del snapshot es el hash del manifiesto: mismo
compromiso criptográfico, informe diagnosticable (un fallo de recomputación dice *qué*
importación derivó, no solo "algo cambió").

Límite conocido: los ficheros cargados desde la instalación de PeTTa pueden tener integridad
sin procedencia (el hash consta; de qué commit/versión venía, quizá no). Si E0 captura 2
confirma que la procedencia no es recuperable, se declara como límite en el informe.

### E-5. Corrección del §5.2: el testigo, por reproducibilidad y no por transporte

- El §5.2 afirmaba que el testigo existe como tipo nativo (ETV con stamps). Es cierto del
  ecosistema (`trueagi-io/PLN`, `pln-experimental`) y **falso del sistema objetivo**: el
  `lib_pln` de 309 líneas que OmegaClaw ejecuta no contiene maquinaria de stamps (grep de
  `stamp`, `evidence`, `EvID`: cero coincidencias). La divergencia de los tres `lib_pln` no
  es solo de versión: la de producción perdió el tracking de evidencia.
- Reformulación del nivel 2 para este sustrato: no hace falta capturar la derivación, hace
  falta que sea **reproducible**. Con (premisas literales, lockfile del ruleset, motor+versión,
  conclusión) un tercero reejecuta `(|- ...)` y comprueba. La identidad de la regla es
  recuperable por recomputación; no se transporta.
- Alcance exacto del nivel 2 resultante: **un salto es recomputable; una cadena es un conjunto
  de saltos individualmente recomputables unidos por eslabones mediados** (E-2). El AtomSpace
  se destruye por llamada; el multi-paso pasa por el LLM.

### E-6. Rediseño del Artefacto 2 de C2 — y C2 sube de prioridad

El Artefacto 2 original ("la recomputación cierra") asumía un ETV con stamp que no existe en
OmegaClaw, y era una afirmación técnica que puede fallar por seis motivos distintos. El
rediseño: **"la cadena entera es auditable eslabón a eslabón, y esto es lo que cada eslabón
soporta"** — el informe de confianza residual aplicado a un razonamiento real. Es el §1
demostrándose a sí mismo sobre un sistema que otros mantienen.

Consecuencia: C2 deja de ser el artefacto que enseña el techo del protocolo y pasa a ser el
que enseña **el producto funcionando**. Sube en prioridad dentro del bloque C. El Artefacto 1
(borrado silencioso de `memory/history.metta`) no cambia.

Forma final del artefacto: pendiente de E0 (capturas 3 y 4).

### E-7. Regla de canonicalización, escrita como par (Profile)

Dos reglas opuestas sobre la misma operación, cada una con su razón al lado:

- **Hojas Merkle: en orden de emisión, nunca ordenadas** — porque el orden **es la evidencia**
  (anti-omisión). [Ya en decisión 3 del §3.1.]
- **Conjuntos de resultados no deterministas: en orden normalizado antes de comprometer** —
  porque el orden **es ruido del motor** (anti-falso-fallo de recomputación).

El criterio que decide es qué es señal en cada plano. Escribir las dos juntas; formuladas por
separado, alguien aplicará una donde va la otra.

### E-8. Corrección del §9: procedencia del contenido vs. procedencia del proceso

La distinción que sostiene el §9 tal como está: el artículo 50.2 (marcado de contenido
sintético) es **procedencia del contenido** — territorio C2PA/OProW. Las obligaciones que ACTA
sirve (registro de eventos, trazabilidad, conservación de logs de alto riesgo, capítulo III)
son **procedencia del proceso**, y son exactamente las aplazadas al 2-dic-2027 (Anexo III) y
2-ago-2028 (Anexo I). Los dieciséis meses del §9 quedan intactos.

Nota de fechas verificada: el 2-dic-2026 **no** es una obligación nueva de marcado; es el fin
del período transitorio de cuatro meses (nuevo art. 111.4) para sistemas ya en mercado antes
del 2-ago-2026. Lo nuevo a esa fecha son dos prohibiciones añadidas al art. 5 (material íntimo
no consentido, material de abuso de menores).

**Al repo, con referencia directa al Reglamento (UE) 2026/1744 en el DOUE, no con notas de
despacho:**

- Reglamento (UE) 2026/1744 (DOUE, texto íntegro en ES):
  https://eur-lex.europa.eu/legal-content/ES/TXT/PDF/?uri=OJ%3AL_202601744
- Reglamento (UE) 2024/1689 (AI Act base, ELI):
  https://eur-lex.europa.eu/eli/reg/2024/1689/oj

---

## Parte 3 — Orden de ejecución resultante

```
AHORA (en paralelo):
  ├── E0: las cuatro capturas               ← media jornada con Docker andando
  ├── A1: verificación de firmas            ← necesario para cualquier demo
  ├── A3: claves inline en bundle           ← necesario para cualquier demo
  └── §9: las tres conversaciones           ← con alguien que firma cumplimiento
      de validación                            en banca o sanidad

TRAS E0 (con las cuatro decisiones escritas):
  ├── Especificación del lockfile (E-4, forma según captura 2)
  ├── Predicado de igualdad del verificador (según captura 1)
  └── Rediseño final de C2 (según capturas 3 y 4)

TRAS LAS CONVERSACIONES:
  └── A2: adaptador Cardano real            ← hasta entonces, mock + TR-ANCHOR-UNVERIFIED
                                               es el producto funcionando como se diseñó

EN PARALELO SIEMPRE:
  └── Deuda documental (§6 de la directiva) + enmiendas E-1..E-8 de este documento
```

Regla de la compuerta: **ni una línea de C2 antes de cerrar E0.** Regla de la validación:
**ni una línea del adaptador Cardano real antes de las tres conversaciones.**

---

## Anexo A — Fuentes externas, agrupadas por uso

Ninguna de estas fuentes es vinculante para ACTA. Son sustrato ajeno que el experimento
observa, o referencia normativa citada. Su lugar en la jerarquía de autoridad es el de
material externo: solo obliga lo que se reescriba bajo `constitution/`, `architecture/` o
`decisions/`.

### A.1 E0 — ruta crítica

| Fuente | Para qué | Qué anotar |
|---|---|---|
| https://github.com/asi-alliance/OmegaClaw-Core | El sistema objetivo. Contiene `memory/history.metta`, `lib_pln.metta`, `lib_nal.metta`, `lib_omegaclaw.metta` y la documentación de referencia (`reference-internals-loop.md`, `reference-internals-memory-store.md`, `reference-plugin-api.md`, `introduction.md`) | Commit exacto del clon |
| https://github.com/trueagi-io/PeTTa | El sustrato. Ahí viven los `lib_patrick`, `lib_llm`, `lib_vector`, `lib_combinatorics`, `lib_he` de la captura 2, y su propio `lib/lib_pln.metta` (466 líneas) | Commit instalado |
| https://github.com/patham9/petta_lib_chromadb | El `git-import!` sin revisión fijada | Commit HEAD en el momento del experimento: es el dato que el lockfile tendría que haber fijado |

### A.2 Contraste del §5.2 — los que sí tienen stamps

| Fuente | Para qué |
|---|---|
| https://github.com/trueagi-io/PLN | El `lib_pln.metta` de 430 líneas, con `StampDisjoint`/`StampConcat` |
| https://github.com/trueagi-io/pln-experimental | `common/truthvalue/EvidentialTruthValue.metta`, el tipo ETV |

Los tres `lib_pln` (309 / 430 / 466 líneas) son el material de la corrección E-5: la
divergencia no es solo de versión, la de producción perdió el tracking de evidencia.

### A.3 Bloque A2 — solo tras las conversaciones de validación

| Fuente | Para qué |
|---|---|
| https://github.com/singnet/watermarks_PoC | `openwater_mk/cardano.py` como referencia del esquema de anclaje. Recordatorio del §5.8: el `oprow` vendorizado dentro **se lee, no se reutiliza** |
| https://blockfrost.io | El backend real tras el mock |

### A.4 Referencia semántica y verificación diferencial — posterior, no E0

| Fuente | Para qué | Cautela |
|---|---|---|
| https://github.com/trueagi-io/hyperon-experimental | La especificación congelada (v0.2.10) | — |
| https://github.com/trueagi-io/MORK | Metodología de `differential/run.py` | ⚠️ **Sin licencia**: solo leer la metodología, no depender |
| https://github.com/Adam-Vandervorst/PathMap | Exploración | ⚠️ **Cuenta personal**: exploración, no dependencia |

**JeTTa — sin resolver.** La directiva lo cita (v0.9.0, MIT, Kotlin) pero sin organización, y
una búsqueda rápida no lo resuelve. Es material de verificación diferencial futura, no de E0,
así que no bloquea nada. Cuando haga falta: buscarlo en las organizaciones `trueagi-io` o
`patham9` antes que fiarse de un enlace reconstruido. Queda anotado en
`research/open-questions.md`.

### A.5 Sustrato de anclaje — solo para reconfirmar estado

- https://github.com/asi-alliance/asi-chain

### A.6 Regulatorio — enmienda E-8

- Reglamento (UE) 2026/1744 (DOUE, ES):
  https://eur-lex.europa.eu/legal-content/ES/TXT/PDF/?uri=OJ%3AL_202601744
- Reglamento (UE) 2024/1689 (AI Act base, ELI):
  https://eur-lex.europa.eu/eli/reg/2024/1689/oj
