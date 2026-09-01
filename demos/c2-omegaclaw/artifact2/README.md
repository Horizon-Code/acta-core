# C2 / Artefacto 2 — auditoría eslabón a eslabón

Este demostrador produce un informe de cadena mediada sin enseñar a Core qué es Hyperon,
MeTTa u OmegaClaw. Compara cada premisa del salto siguiente con la transformación declarada
de una conclusión comprometida anterior:

```text
bytes(premisa) == bytes(T(C))
```

La igualdad solo clasifica el eslabón como **fiel** si ambos objetos tienen el mismo
`ProcessRefV0` completo (`process_id` y `process_type`) y la conclusión precede a la premisa
en un orden Chronos ya verificado. Si falla cualquiera de esas tres condiciones, el eslabón
es **libre**. El artefacto no decide equivalencia semántica.

## Ejecutar

No requiere red, credenciales ni dependencias externas:

```bash
cd demos/c2-omegaclaw/artifact2
python3 audit_links.py vectors/e0-capture3.json --format text
python3 audit_links.py vectors/e0-capture3.json --format json
python3 -m unittest -v
```

El vector referencia directamente las cadenas t2, t3, t4, t5 y t7 de
`research/e0-logs/c3-measurements.json`. Los tests comprueban que sus strings y líneas de
procedencia siguen coincidiendo con aquel artefacto de E0.

Captura 3 precede a la instrumentación ACTA de C2: sus cuatro strings por enlace —conclusión
cruda, `T(C)`, bloque exacto `LAST_SKILL_USE_RESULTS` y premisa siguiente— son evidencia
capturada real, pero no contiene `ProcessRefV0`, receipts Chronos ni un lockfile ACTA. El
vector añade un `ProcessRefV0` de demo por tarea y traduce el orden observado salto 1 → salto
2 a posiciones de prueba. Esto valida el contrato del auditor, no afirma que aquellos logs
históricos ya fueran bundles ACTA. En la demo viva, ambos campos deben proceder de los eventos
y de la verificación Chronos reales.

## Transformación declarada

`T` reproduce el orden de código medido en captura 3:

1. `normalize_string`: UTF-8 encode/decode con `errors="ignore"`;
2. `string-safe`: `""` → `_quote_`, salto de línea → `_newline_`, `'` →
   `_apostrophe_`;
3. `last_chars(maxFeedback)`.

El input debe declarar además la fuente/versión del arnés, `maxFeedback` y `maxHistory`.
Este último no transforma C directamente, pero forma parte de la configuración fijable del
arnés según E0. La codificación, la política de errores, las sustituciones y el lado retenido
por el corte también son campos máquina-legibles. Un orden o una semántica de `T` distintos
se rechazan: una transformación no declarada no es evidencia de fidelidad.

El objeto `verification_context` liga esa declaración a hashes que el auditor comprueba. El
cierre mínimo de `T` conserva por separado `helper.py`, `loop.metta` base y `utils.metta`, con
los hashes medidos en E0 §2.6; el hash de `loop.metta` con la sonda `RAW_EVAL` se añade como
artefacto de observación y no se hace pasar por el cierre entero. También se fijan la
configuración completa del arnés y el descriptor de `T`. Estos dos últimos digests se
recalculan mediante JSON UTF-8 con claves ordenadas y separadores compactos, seguido de
SHA-256. Cambiar `maxFeedback`, el orden, una sustitución o un hash de fuente sin actualizar
el contexto hace fallar el informe.

En el vector E0, `kind` es `verification-envelope`, `historical_lockfile_present` es `false`
y cada clasificación dice `contract-validation-over-synthetic-verification-envelope`. Sirve
como forma candidata de los campos que deberá transportar el lockfile: no fabrica
retroactivamente uno conforme a ADR-004 ni afirma que captura 3 verificase dicho lockfile. El
auditor S6 rechaza incluso un input que intente marcar ese lockfile histórico como presente:
esa modalidad solo podrá habilitarse integrando el verificador ADR-004 real.

`chronos.conclusion_index` y `chronos.premise_index` son posiciones entregadas a esta demo
por una verificación previa de la cadena, no campos nuevos de Protocol v0. La bandera
`order_verified` impide que números afirmados sin verificación satisfagan la precedencia.
En el vector histórico esa bandera sigue siendo parte de la envoltura sintética y el texto
del informe lo dice; una captura viva debe alimentarla desde la verificación Chronos real.

## Cómo leer el 5/5

El resultado del vector es **5 de 5 fieles como techo empírico bajo una instrucción explícita
de copia byte a byte**. Las cinco tareas son variantes isomorfas; el dato no es una tasa del
mediador ni se generaliza a otros prompts. Por diseño, el objeto `chain_mediation` mantiene
juntos el numerador, el denominador, el alcance y este descargo obligatorio:

> la selección de qué conclusiones se realimentan es del productor y este dossier no la
> acota.

La métrica detecta distorsión, no selección. t1 y t6 se transportan como evidencia narrativa,
fuera del denominador: en t1 el mediador intentó el segundo salto omitiendo la conclusión;
en t6 recalculó tres veces el primero antes de arrastrarla. Los logs originales son
`research/e0-logs/c3-t1.rawlog` y `research/e0-logs/c3-t6.rawlog`.

## Dos condiciones distintas

- `TR-CHAIN-MEDIATED` se emite únicamente cuando el contexto declara presente un mediador
  no determinista que une los saltos. Permanece aunque todos sus enlaces sean fieles; sin
  mediador no aparece.
- `TR-NONDET-INPUT` aparece cuando una premisa está declarada como formulada o seleccionada
  por un proceso no determinista. Nombra el origen y la selección de la entrada; es
  independiente de la presencia del mediador de cadena y no sustituye la clasificación
  fiel/libre del eslabón.

Ambas condiciones viven aquí, en Profile/demo. Este artefacto no modifica `acta-core`, el
Protocol ni los perfiles normativos.

## Punto de intercepción reproducible: paso 5

El fichero
`patches/0001-log-raw-eval-before-normalization.patch` es el diff mínimo observado en el
clon de OmegaClaw sobre el commit completo
`642c53676cf795cb7a0030823b36018c029b1416`. Solo modifica `src/loop.metta`, dentro del
paso 5 (`eval each skill`): conserva `$R = (eval $s)`, registra
`(RAW_EVAL: $s $R)` y después ejecuta `helper.normalize_string`. No añade un plugin ni cambia
el resultado realimentado.

Para producir una fuente instrumentada sin tocar siquiera el árbol de trabajo del clon:

```bash
cd demos/c2-omegaclaw/artifact2
prepared_dir="$(./prepare_loop_source.sh /ruta/a/OmegaClaw-Core)"
test -f "$prepared_dir/src/loop.metta"
```

El script no copia el fichero de trabajo: extrae `src/loop.metta` directamente del objeto Git
fijado, verifica su SHA-256, comprueba el parche antes de aplicarlo y verifica el SHA-256 final.
Su única escritura ocurre en un directorio nuevo de `/tmp`, cuya ruta imprime. No lee ni
transporta configuración o credenciales. El directorio no se borra automáticamente para que
pueda montarse; el operador puede eliminar esa ruta temporal concreta al terminar.

La imagen de captura carga el loop desde
`/PeTTa/repos/OmegaClaw-Core/src/loop.metta`. En una ejecución Docker controlada, se monta
solo el fichero preparado y en modo lectura:

```bash
docker run --rm \
  --mount "type=bind,src=$prepared_dir/src/loop.metta,dst=/PeTTa/repos/OmegaClaw-Core/src/loop.metta,readonly" \
  IMAGEN_OMEGACLAW \
  commchannel=test provider=Test embeddingprovider=Local
```

La orden es deliberadamente una plantilla: el canal, la red, los límites de seguridad y la
imagen deben ser los de la sesión de captura. Las credenciales, si una captura futura las
necesita, se suministran por el mecanismo externo vigente y nunca se añaden a esta orden ni a
ningún fichero. `scripts/omegaclaw` no admite hoy un volumen de fuente arbitrario; para no
modificarlo, el montaje se hace en el `docker run` controlado por la sesión.

Comprobación esperada en los logs:

```text
... | e0_capture3 | (RAW_EVAL: <skill> <salida cruda>)
```

Esa línea es el punto de compromiso de ADR-005. Cualquier texto posterior de
`LAST_SKILL_USE_RESULTS` sigue siendo un artefacto separado y transformado por el arnés.
