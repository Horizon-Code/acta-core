# C2 / OmegaClaw / Artefacto 1 — borrado silencioso

Este demostrador reproduce una propiedad concreta de OmegaClaw v0.1.19: su traza episódica
`memory/history.metta` es texto plano que el bucle lee por la cola y amplía mediante append.
Si se borra un registro completo, los registros restantes siguen siendo expresiones balanceadas
y el fichero continúa siendo legible. Esa legibilidad local no demuestra integridad histórica.

La comparación ACTA construye primero un prefijo válido del lifecycle del Cognitive Forensics
Profile:
`ai_agent.run_started`, `ai_agent.instruction_received` y un
`ai_agent.context_committed` v1.0 por cada bloque exacto, sin normalizarlo. El lifecycle se
valida con `validate_ai_agent_lifecycle_v0` y cada evento cruza el mapeador formal del Profile;
el `artifact_commitment` de cada contexto liga los bytes del bloque. Todos los eventos quedan
enlazados por Chronos, incluidos en un epoch Merkle y acompañados por receipts Ed25519
verificables offline. El operador guarda los bundles; un directorio hermano simula el dominio
en el que una parte separada retendría `epoch-witness.json`, que contiene la raíz y distingue
el número total de eventos del número de registros de historia. Tras el borrado, los bundles
continúan verificando, pero la historia presente ya no satisface el conjunto comprometido y el
verificador devuelve código `2`. Esta topología local prueba el mecanismo, no satisface por sí
sola el requisito de retención independiente de E-9.6.

## Ejecutar

Prerequisitos: toolchain Rust y dependencias ya disponibles en la caché local. El script fuerza
`cargo --offline`, no usa red, contenedores, servicios ni credenciales.

```bash
cd demos/c2-omegaclaw/artifact1
./run-demo.sh
```

La salida relevante es:

```text
SEALED=PASS
lifecycle_events=5
history_records=3
OMEGACLAW_LOCAL_CHECK=PASS
ACTA_EXTERNAL_CHECK=PASS
verified_lifecycle_bundles=5
verified_history_records=3
SILENT_DELETE=APPLIED
OMEGACLAW_LOCAL_CHECK=PASS
ACTA_EXTERNAL_CHECK=DETECTED
finding=committed_history_record_missing:...
finding=history_record_count_changed:committed=3:current=2
DEMO_RESULT=PASS (...)
```

`OMEGACLAW_LOCAL_CHECK` solo comprueba que el fichero sigue siendo UTF-8 y una secuencia de
registros s-expression con el formato que OmegaClaw escribe. No afirma que se haya ejecutado
un agente vivo. Para usar material real, sustituir la fixture por una copia de
`memory/history.metta` producida por OmegaClaw y ejecutar los mismos comandos `seal`,
`local-check`, `delete-record` y `verify` mostrados en `run-demo.sh`.

La ejecución ya realizada sobre la historia E0 de 100 registros queda registrada, con hashes
y hallazgos, en `e0-real-history-run.md`.

## Reproducir la captura E0 retenida

La historia exacta de esa ejecución está versionada en
`fixtures/e0-real-history.metta` (21.642 bytes; SHA-256
`d320182a34ad8e737f8df404d1359851589675a7c8b0e4ff869770988f19983a`). Se revisó el fichero
completo antes de incorporarlo: contiene solo timestamps, identificadores E0, expresiones
MeTTa/NAL y mensajes operativos genéricos; no contiene credenciales, tokens, datos personales,
URLs ni endpoints privados.

OmegaClaw y Docker solo son necesarios para producir trazas nuevas. Para esta captura no hacen
falta: cualquier máquina con el toolchain Rust puede partir de la historia versionada,
regenerar temporalmente los 102 bundles y el witness, y verificar que obtiene la raíz
`bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba` y el SHA-256 de witness
`3aa94889b07052ea6191ebd953cd35629497869a73e9d135dcd4a9e3a22217f8`:

```bash
cargo test --manifest-path Cargo.toml committed_e0_history_reproduces_original_root_and_witness
```

La prueba crea los bundles derivados en un directorio temporal, ejecuta la verificación
completa y los elimina al terminar. Los 102 bundles no se versionan porque son reproducibles a
partir de `fixtures/e0-real-history.metta`.

## Ancla externa retenida

`anchor/epoch.anchor-evidence.json` es el sidecar `acta.eas-anchor-evidence.v1` de la
atestación EAS real publicada en Base Sepolia el 3-sep-2026 para esta misma raíz. Contiene
evidencia pública de cadena, no credenciales.

Con él, la cadena completa —historia → bundles → raíz → ancla— queda reproducible desde el
repositorio y comprobada en cada build. El mismo test regenera la corrida, adjunta la
referencia de anclaje a los 102 bundles y comprueba que queda bien puesta: que es exactamente
la del sidecar, que solo cambia el campo `anchor` (los bytes firmados no se tocan, ADR-012 §5)
y que cada bundle anclado sigue verificando offline. `attach_refuses_a_foreign_root_and_an_occupied_anchor`
fija los rechazos: raíz ajena, ancla ya ocupada y versión de evidencia distinta.

Ese test es offline y no comprueba el estado en cadena: verifica que la referencia se coloca
bien, no que la atestación siga siendo válida. La comprobación contra cadena es el camino
separado del verificador Rust por JSON-RPC, descrito en `research/s8-eas-base-deployment-2026-09-03.md`.

El attach vive en el publicador Node (`adapters/evm-eas-anchor`); el test replica sus tres
reglas en Rust a propósito, para que el demostrador siga siendo offline y sin dependencias de
`node_modules`. Es una copia deliberada: si el publicador cambiara su semántica, este test no
lo reflejaría por sí solo.

La fixture contiene tres bloques deterministas con la forma documentada y ejercitada por el
arnés mock de OmegaClaw; no se presenta como captura de una sesión real.

## Vinculación con OmegaClaw y el punto de intercepción

La versión medida queda fijada en el witness como commit
`642c53676cf795cb7a0030823b36018c029b1416`. Puede comprobarse localmente contra el checkout
usado por E0:

```bash
./check-upstream.sh /home/ubnasamarnchez/proyectos/e0-substrate/OmegaClaw-Core
```

El chequeo confirma estáticamente las operaciones reales `read_file_tail` y
`append-file-raw`. También confirma el punto de intercepción de E0 §0.4: el par `$s`/`$R`
existe alrededor de `(eval $s)` en `src/loop.metta` y no está expuesto por la API de plugins.
La instrumentación de inferencias del Artefacto 2 debe, por tanto, montarse como parche de
fuente. Este Artefacto 1 no duplica ese parche: compromete la salida episódica al cerrarse cada
turno. En una integración viva, el emisor fino debe recibir el bloque exacto inmediatamente
después de `appendToHistory` y entregar el receipt o la raíz fuera del control del operador
antes de considerar constituida la evidencia.

## Frontera externa obligatoria

`seal` rechaza que el witness esté dentro de `operator-dir`, después de normalizar componentes
`.` y `..` y resolver cualquier prefijo existente con symlinks. En el script, ambos dominios
son directorios hermanos:

```text
tmp/
├── operator/                 # historia y bundles, control del operador
└── external-party/
    └── epoch-witness.json   # copia retenida fuera del operador
```

La separación de directorios representa una topología de despliegue, no una institución real.
El operador de la máquina de demo puede modificar ambos. Una demostración ante terceros debe
sustituirla por un receipt/`epoch_root` realmente retenido por la contraparte o por un anclaje
externo verificable. Sin esa frontera, el operador podría borrar la historia, regenerar el
epoch y presentar una narrativa local nueva.

## Qué prueba y qué no

Prueba:

- que borrar un registro completo puede dejar una traza OmegaClaw todavía legible;
- que los bytes borrados ya no coinciden con los compromisos constituidos antes del borrado;
- que Chronos, receipts y pruebas Merkle se verifican con el Core y el adaptador existentes;
- que, si una raíz previa se retiene realmente fuera del control del operador, la comparación
  impediría sustituir silenciosamente el epoch local; la ejecución incluida simula esa frontera.

No prueba:

- que OmegaClaw haya emitido todos los actos que estaba obligado a registrar;
- verdad material de los textos o inferencias comprometidos;
- identidad institucional del attestor: la clave pública va inline y la semilla del demo es
  deliberadamente pública (`TR-KEY-SELF-ASSERTED` sigue siendo aplicable);
- independencia real de la carpeta que simula a la contraparte;
- existencia de un anclaje en una cadena pública;
- una ejecución viva del LLM ni una tasa general de fidelidad del mediador.

El demostrador usa el vocabulario y el lifecycle formal existentes; no añade semántica al Core
ni modifica el Profile. Los dos eventos de apertura son evidencia de ciclo y no se cuentan como
registros de `history.metta`: con la fixture hay cinco bundles de lifecycle y tres compromisos
de historia.

## Tests

```bash
cargo fmt --manifest-path Cargo.toml -- --check
cargo test --offline --manifest-path Cargo.toml
```

Los tests cubren preservación exacta de bytes, validez y mapeo del lifecycle, separación de
conteos 5/3, reproducción de la captura E0 retenida con sus hashes y conteos 102/100, detección
del registro central borrado y rechazo de un witness colocado bajo el dominio del operador,
incluidos intentos con `..` y con symlink.
