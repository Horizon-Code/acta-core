# E0 — Resultados

**Estado:** EN CURSO. Captura 2 con datos reales en tres de sus cuatro pasos; las otras tres
capturas pendientes de la imagen de mock (1 y 4) y de una clave LLM (3).
**Compuerta:** 4/4 ✗ — **cerrada**.
**Protocolo:** `roadmap/E0-protocolo-y-enmiendas.md`, Parte 1.
**Autoridad:** material exploratorio y empírico. No redefine `architecture/` ni `decisions/`.

Este fichero es el entregable de la compuerta E0 (§1.6 del protocolo). Mientras alguna de las
cuatro decisiones siga vacía, la compuerta está **cerrada**:

> Ni una línea de C2 antes de cerrar E0.

Regla de registro: **todo en crudo**. Strings completos con timestamps, o enlace al log. Nada
de resúmenes — el resumen es la decisión, y va en su apartado, no en la captura.

**Distinción que atraviesa todo el documento.** Lo marcado como *estático* se obtuvo leyendo
código y ficheros de configuración en disco, **sin ejecutar el sistema**. Lo marcado como
*capturado* proviene de una sesión real con el contenedor en marcha. Ninguna decisión derivada
se escribe solo con material estático: el estático acota la respuesta, no la cierra.

---

## 0. Datos de preparación

Estos datos no son metadatos del experimento: son **su primer resultado**. Son la primera
muestra del problema de procedencia del cierre de importaciones (E-4).

### 0.1 Runtime de contenedores — RESUELTO 2026-08-31

Comprobación del §0 de la orden de sesión, con el usuario `ubnasamarnchez` en
`Ubuntu-24.04` (WSL2):

| Comprobación | Resultado |
|---|---|
| `id` | Pertenece a los grupos `sudo` (27) y `docker` (1001) |
| `sudo -n -l` | `sudo: a password is required` — sin sudo no interactivo |
| `getent group sudo docker` | `sudo:x:27:ubnasamarnchez` · `docker:x:1001:ubnasamarnchez` |
| `docker` en la distro | Solo el shim de Docker Desktop en `/mnt/c/...`; *"The command 'docker' could not be found in this WSL 2 distro"* → **integración WSL desactivada** |
| `docker.exe info` (lado Windows) | `open //./pipe/dockerDesktopLinuxEngine: The system cannot find the file specified` → **el motor no está arrancado** |
| `podman` | No instalado; instalarlo exige `apt` y por tanto contraseña de sudo |

Pertenecer al grupo `docker` no servía: no había demonio en la distro al que conectarse.

**Resuelto** instalando Docker nativo en la distro (`docker.io`, con `systemd` ya activo en
WSL). Estado actual: **Docker Engine 29.1.3**, demonio arriba, accesible con el usuario sin
`sudo`, `overlayfs`, cgroups v2. La integración de Docker Desktop no se llegó a usar.

### 0.2 Clave de proveedor LLM — solo bloquea la captura 3

No se ha recibido ninguna clave por el canal fuera de banda del §0.3 de la orden de sesión.

**El alcance del bloqueo es menor de lo previsto.** OmegaClaw incluye un proveedor LLM
simulado y un canal de comunicación de prueba, seleccionables con `-p Test -t test` en
`scripts/omegaclaw`, con un arnés en `Autotests/mock/` que permite fijar la respuesta exacta
del "LLM" (`llm.set_answer(prompt, respuesta)`).

| Captura | ¿Necesita LLM real? | Motivo |
|---|---|---|
| 1 — Estabilidad del collapse | **No** | El mock permite emitir el mismo `(metta "(|- p1 p2)")` de forma exactamente reproducible. Es *mejor* que un LLM real: aísla el no determinismo del motor del no determinismo del mediador |
| 2 — Cierre de importaciones | **No** | No depende del bucle de turno |
| 3 — Proporción fiel/libre | **Sí** | Mide precisamente qué arrastra un mediador no determinista entre saltos. Con respuestas fijadas la medida no significa nada |
| 4 — Truncamiento | **No** | El mock puede provocar una inferencia de resultado grande de forma determinista |

Limitación: el README de `Autotests/mock/` advierte que *"the mock infrastructure is part of
the source tree, so the image must be built locally rather than pulled from the registry"*. La
imagen publicada no sirve para el mock; hay que construir `omegaclaw:mock` localmente.

### 0.3 Procedencia de los clones (estático)

Clonados en `/home/ubnasamarnchez/proyectos/e0-substrate/`, fuera del repositorio de ACTA.

| Repositorio | Commit | Fecha del commit |
|---|---|---|
| `asi-alliance/OmegaClaw-Core` | `642c53676cf795cb7a0030823b36018c029b1416` | 2026-08-24 18:09:48 +0300 |
| `trueagi-io/PeTTa` (tag `v1.0.4`, el que fija el Dockerfile) | `7037f4c2ad378c52fc328004fe216d5118b674f0` | 2026-07-19 13:06:37 +0200 |
| `patham9/petta_lib_chromadb` (HEAD de `master` hoy) | `218484875d5d1bfb217a9a03d3983dc1ed9d406c` | 2026-08-04 22:41:02 +0200 |

Fecha de la sesión de preparación: **2026-08-31**.

**El HEAD de `petta_lib_chromadb` es el dato que el lockfile tendría que haber fijado.** Queda
anotado aquí precisamente porque el sistema no lo anota en ningún sitio.

Datos tomados **de dentro de la imagen publicada**, que es el artefacto que la gente ejecuta:

| Dato | Valor | Cómo se obtuvo |
|---|---|---|
| Imagen publicada | `singularitynet/omegaclaw@sha256:b819e71e293a7974d562b7b6f745367103296c9c7bc555ee23f115724d02a6e3` | `docker inspect` |
| Fecha de construcción de la imagen | 2026-08-24T17:15:29Z | `docker inspect` |
| Versión declarada de OmegaClaw | `v0.1.19` (fichero `/PeTTa/repos/OmegaClaw-Core/version`) | `cat` dentro del contenedor |
| SWI-Prolog | **10.0.2** para x86_64-linux | `swipl --version` dentro del contenedor |
| Python | **3.11.2** | `python3 --version` dentro del contenedor |
| Proveedor y modelo LLM | Por defecto `Anthropic` / `claude-opus-4-8`; en esta sesión se usará `Test` (mock) | `config/config.yaml`, `src/loop.metta` |

Nota de coherencia: la imagen se construyó a las 17:15 del 24-ago y el commit del clon
(`642c536`) es de las 18:09 del mismo día, o sea **posterior**. Los hashes coinciden igual
(§2.5), lo que significa que ese último commit no tocó ningún fichero del cierre.

### 0.4 Punto de intercepción del paso 5 (estático)

El paso 5 del bucle está en `OmegaClaw-Core/src/loop.metta`, dentro de `(= (omegaclaw $k) ...)`.
El string que entra en la skill es `$s` y el que devuelve es `$R`:

```metta
(HandleError SINGLE_COMMAND_ERROR_NOTHING_WAS_DONE_PLEASE_FIX_AND_RETRY $s
  (catch (let $R (eval $s) (py-call (helper.normalize_string $R)))))
```

- **Existe el punto**, y es exactamente el par (entrada, salida) que E0 §1.1.4 pide registrar.
- **No está expuesto por la API de plugin.** `reference-plugin-api.md` permite añadir canales,
  proveedores y skills (`add-skill` / `remove-skill`); no permite envolver el `eval` del bucle.
  Instrumentarlo exige **modificar `src/loop.metta`** y montar el código o reconstruir la
  imagen — el aviso que la propia documentación de OmegaClaw ya da.
- Confirmación pendiente de ejecución. Pero la conclusión provisional es que **el aviso del
  §5.1 de la Directiva se cumple en el sentido malo**: la vía es parche de fuente montado, no
  plugin. El canal `wschat` intermediario no ayuda aquí, porque ve el contexto ensamblado, no
  el par (entrada, salida) del eval.

---

## 1. Captura 1 — Estabilidad del collapse

### Hallazgos estáticos previos

Ninguno relevante. La estabilidad del `collapse` y el comportamiento de `unique-atom` no son
derivables por lectura: dependen del orden de resolución de SWI-Prolog en tiempo de ejecución.

### Datos en crudo

- Premisas usadas: *(pendiente)*
- Ejecución 1 (misma sesión) — salida completa: *(pendiente)*
- Ejecución 2 (misma sesión) — salida completa: *(pendiente)*
- Ejecución 3 (tras reiniciar el contenedor) — salida completa: *(pendiente)*
- Caso de conclusión duplicada con TV distinto — construcción y salida: *(pendiente)*
- ¿`unique-atom` deja una o las dos? *(pendiente)*

### Decisión derivada

*(pendiente — bloqueada por §0.1 y §0.2)* — predicado de verificación de recomputación.
Opciones del protocolo: igualdad de serialización canónica / igualdad de conjuntos o
pertenencia / comparación de pares (conclusión, TV).

---

## 2. Captura 2 — Cierre de importaciones: ¿estático o dinámico?

### 2.1 Enumeración y hash del cierre (estático — paso 1 del procedimiento, COMPLETADO)

Cierre transitivo desde `run.metta` y `lib_omegaclaw.metta`, resuelto contra el layout en
disco de los tres repositorios a los commits del §0.3. Script en
`/home/ubnasamarnchez/proyectos/e0-substrate/closure.py`.

**34 ficheros resueltos: 21 `.metta` y 13 `.py`.**

```
3f2894d9b1bc4e39fd7d0384aa6c8503b847eed38826222941ffa14dc0610283  OmegaClaw-Core/lib_nal.metta
1b7b94f148006184676533aee338c8e964c679db1fd6a3590a288da59c9d689f  OmegaClaw-Core/lib_omegaclaw.metta
05b63266ad79c3cd0b32a83a4596ad47923e612e746303628da7500bbcffa7ae  OmegaClaw-Core/lib_pln.metta
a097fc6a3cdb5e097547a4590511f04572501added79d21d632b2ef10ba1c2e6  OmegaClaw-Core/profile/policy.metta
c5c6461012f09f8a0d04af8a9849dfc6aa8394181227b7ff340e3e94037bebb3  OmegaClaw-Core/profile/policy.py
745318e2ce626400cabc36774069e5d65e4fd39fbc6511802f1ff0341ed5707d  OmegaClaw-Core/providers/lib_llm_ext.py
a84528f34853db8f7848f37e370de1d2733b689ef9af87445ca54c30115c4bcd  OmegaClaw-Core/run.metta
fe1aa031924f4ba37e6325f3ab5823998e344562c033ab5125923bd7a7e7ee38  OmegaClaw-Core/src/channels.metta
5de44bacd7e1cdc8efb252a66082ea8ed6a6250961e6a9302c20cc61d4e9d410  OmegaClaw-Core/src/channels.py
69e4d6ef27d3496653aee3e239eb69d580a1f2928f61b9dce68a279605b9dd26  OmegaClaw-Core/src/config.metta
27149cc74d5dc6eeaa90fc5f34cd66d49360f86b000caf7e146dbadde1a15dd2  OmegaClaw-Core/src/config.py
b318cb9925f64d71bac0ef84fbfe7a4ca9b2ad7c3ac9abfd09715c6dd2a3fbbe  OmegaClaw-Core/src/fileio.py
de0682520b6d17b2080a19b3ea3849f061c17f052d9baa07bd1fd92c3e554f20  OmegaClaw-Core/src/helper.py
f96dd00ab9370f2051d9e569cc93a57aac1df2b4e55ce4593bdaff1c8d5a0d7b  OmegaClaw-Core/src/log.metta
5ceb810cfd703ac7f87eaeeb7a25b7d675028458c07a24d6fcc74b21e60ffc00  OmegaClaw-Core/src/logger.py
f5b0028fa7b24666d3192e857228fb782d4638469b773eb7352a30a841eab45a  OmegaClaw-Core/src/loop.metta
d5857e294a600ee8c5d3e41e91a0be749f068fc810004c9661b29f480c1f0def  OmegaClaw-Core/src/memory.metta
3e48c7b7e9f7d7b4cc685e1e57aab15e7ff899912f7c60f2b1344a7e865b5427  OmegaClaw-Core/src/plugin.metta
ba11341800e4afb17815c478f1c6e2e4fbaf503be1a9d9cad04f70635852ce0f  OmegaClaw-Core/src/plugin.py
872f84f39504d69db0b25b9ce3457984d7bdffdf7b9665d6656362150c622ee2  OmegaClaw-Core/src/providers.metta
aee57c01815be06f0e41b568c8f01f61abc6832e701cab8125fe22281cf62c15  OmegaClaw-Core/src/providers.py
305ed39d3a21f22c9e819451fdda66bc2db9e817593b730d16e5a3aae93411a1  OmegaClaw-Core/src/rag.py
44d0f0d3e662d20ba490088844c059b208bf94edee655457b6800a1819602674  OmegaClaw-Core/src/skills.metta
9cfa04d6e87228cf533fc677d69099671ba7cf2e2c62b6b4185449b05380f31f  OmegaClaw-Core/src/utils.metta
8f35474512bd41e1d730b565b3f78c80e4bfc17a20e03d8aab2d3242dcc61ef5  OmegaClaw-Core/src/websearch.py
aca0a188210050d7db0539e59361bea85040368331fa63f35b6485daf763e428  PeTTa/lib/lib_combinatorics.metta
af53f7c4bc68ef3797eaa8652c37d0d175ff2ea510ea57340b1e688064973faf  PeTTa/lib/lib_he.metta
609731cade81b0dc1aa749f3360a76d354d11d1d333c851785ab8aaf420d3f68  PeTTa/lib/lib_import.metta
1c9b3272b800b4a6ebadb292d9ee149af60e5f1427d31193f8487d1d23e02ad7  PeTTa/lib/lib_llm.metta
cbdeef7260308604acf0185e896e1d4268e57abeab250e37c09fc5a97139d273  PeTTa/lib/lib_llm.py
0a7824ad9a72c5f89f5b5a3196de72eed96e78a034f7970a498f43e734982f6f  PeTTa/lib/lib_patrick.metta
f049a0f65cddc6ec1edb959f15d64d24a580886b7ef3984c79243166f5a34aee  PeTTa/lib/lib_vector.metta
729e893cf6864f0137f5aefea1657f62e87352ec06a6e512c1dc3ea2d4faf86e  petta_lib_chromadb/lib_chromadb.metta
461b78f92234b58e1090e66c332d627a063e201751d8247459d82a18e7afd36d  petta_lib_chromadb/lib_chromadb.py
```

**Cuatro observaciones que cambian el enunciado del problema:**

1. **El cierre no es solo de `.metta`.** Trece de los treinta y cuatro ficheros son Python,
   importados con la misma forma `(library ... ./src/config.py)`. El procedimiento de E0 §1.3
   dice "enumerar todos los `.metta`"; enumerarlos solos deja fuera el 38% del cierre. **El
   lockfile de E-4 tiene que cubrir el cierre completo, no la parte MeTTa.**
2. **Hay una importación que no resuelve:** `!(import! &self (library OmegaClaw-Core ./src/context))`
   en `lib_omegaclaw.metta`. No existe `src/context.metta` ni `src/context.py` en el árbol al
   commit `642c536`. Un manifiesto generado estáticamente tiene aquí una entrada sin fichero;
   qué hace el cargador con ella es dato de ejecución.
3. **`lib_import.pl` genera artefactos derivados.** `static-import!` compila `.metta` → `.pl` →
   `.qlf` y consulta el `.qlf` si existe. El estado en disco tras una ejecución **no es el
   mismo** que antes: hay ficheros nuevos que no están en el manifiesto de origen. El paso 3
   del procedimiento (re-hashear tras la sesión) tiene que distinguir *fichero de origen
   modificado* de *artefacto derivado nuevo*.
4. **Dos `git-import!`, ninguno con revisión en la llamada**, tal como predice
   `TR-IMPORT-UNPINNED`:
   - `run.metta` → `https://github.com/asi-alliance/OmegaClaw-Core.git`
   - `lib_omegaclaw.metta` → `https://github.com/patham9/petta_lib_chromadb.git`

### 2.2 Semántica de `git-import!` (estático — paso 2, parcialmente respondido)

De `PeTTa/lib/lib_import.pl`:

```prolog
'git-import!'(GitPath, BuildCmd, BaseDir, true) :-
    ...
    ( exists_directory(LocalDir) -> true
                                  ; clone_repo(GitPath, LocalDir),
                                    run_build_step(LocalDir, BuildCmd) ),
    asserta(library_path(LocalDir)).

clone_repo(GitPath, LocalDir) :- ...
    process_create(path(git), ['clone', '--depth', '1', GitPath, LocalDir], ...).
```

- `git clone --depth 1` **sin `--branch` y sin commit**: siempre la punta de la rama por
  defecto en el momento del clon. No hay revisión que fijar ni registro de cuál se trajo.
- **Si el directorio ya existe, no clona ni actualiza.** El `git-import!` es idempotente por
  existencia de directorio, no por contenido.
- Ambas llamadas son *bangs* de nivel superior en sus ficheros, lo que sugiere ejecución en
  carga y no perezosa — **pendiente de confirmar observando el arranque**, que es lo que el
  paso 2 pide literalmente.

### 2.3 Dónde sí hay fijación, y de qué calidad (estático)

El `Dockerfile` de OmegaClaw-Core es el único sitio del sistema donde algo se fija:

| Dependencia | Cómo se fija | Calidad |
|---|---|---|
| Imagen base | `ARG SWIPL_IMAGE=docker.io/library/swipl:10.0.2` | **Tag, mutable.** El propio fichero lo admite: *"For maximum integrity, set this to an immutable digest in CI/CD"* |
| PeTTa | `ARG PETTA_REF=v1.0.4` | **Tag, mutable.** Resuelve hoy a `7037f4c2…` |
| FAISS | `ARG FAISS_REF=v1.8.0` | Tag, mutable |
| `petta_lib_chromadb` | `ARG CHROMADB_REF=master` | **Rama.** El caso exacto de `TR-IMPORT-UNPINNED` |
| Paquetes Python | `requirements.txt` con `==` | Versión fijada, **sin hash**; `torch==2.12.1` desde el índice de PyTorch |
| `mork_ffi`, `faiss_ffi` | `PeTTa/build.sh`: `git clone <url>` + `git pull`, **sin ref alguna** | Sin fijar. Dependencias transitivas del sustrato |

El clon de `petta_lib_chromadb` lo hace el **Dockerfile en tiempo de build**, en
`/PeTTa/repos/petta_lib_chromadb`. En ejecución, `git-import!` encuentra el directorio y no
hace nada. Es decir: **la fijación efectiva es la del build, y fija a `master`.**

### 2.5 Verificación del cierre contra el artefacto desplegado (CAPTURADO)

La enumeración del §2.1 se hizo sobre clones en el host. Comprobación de que corresponde a lo
que realmente se ejecuta: se hashearon los mismos 34 ficheros **dentro de la imagen
publicada**, mapeando el layout del host al del contenedor
(`OmegaClaw-Core/…` → `/PeTTa/repos/OmegaClaw-Core/…`, `PeTTa/…` → `/PeTTa/…`,
`petta_lib_chromadb/…` → `/PeTTa/repos/petta_lib_chromadb/…`).

```
IDÉNTICOS : 34/34
DISTINTOS : 0
AUSENTES  : 0
```

**El cierre enumerado estáticamente es exactamente el desplegado.** No hay ficheros del cierre
que la imagen resuelva de otro modo, ni ninguno que falte. La opción "instrumentar el cargador
de PeTTa porque el cierre no es enumerable" queda descartada por evidencia, no por conjetura.

### 2.6 Procedencia desde el artefacto en ejecución (CAPTURADO — paso 4)

El paso 4 pregunta si la procedencia de los ficheros cargados es recuperable desde el proceso.
Comprobado dentro de la imagen publicada:

| Directorio | `.git` | `HEAD` recuperado |
|---|---|---|
| `/PeTTa` | **Sobrevive** | `7037f4c2ad378c52fc328004fe216d5118b674f0` |
| `/PeTTa/repos/petta_lib_chromadb` | **Sobrevive** | `218484875d5d1bfb217a9a03d3983dc1ed9d406c` |
| `/PeTTa/repos/OmegaClaw-Core` | **SIN `.git`** | — |

**El resultado es el contrario del que anticipaba el protocolo.** E0 §1.3 y E-4 preveían que
los ficheros de la instalación de PeTTa tuvieran *integridad sin procedencia*. Ocurre lo
opuesto:

- **Los cinco `lib_*` de la captura 2 sí tienen procedencia recuperable.** Viven en `/PeTTa/lib`
  y el `.git` de PeTTa sobrevive en la imagen con `HEAD = 7037f4c2…`, que coincide exactamente
  con el commit al que resuelve el tag `v1.0.4` del Dockerfile. Lo mismo vale para
  `petta_lib_chromadb`: aunque su `git-import!` no fija revisión ninguna, el `.git` sobrevive
  y **el commit efectivamente usado es recuperable a posteriori**, `218484875d…`, el mismo que
  el HEAD de `master` en la fecha de esta sesión.
- **Quien pierde la procedencia es el sistema objetivo.** La etapa `versioned-source` del
  Dockerfile hace `rm -rf ./.git` sobre OmegaClaw-Core deliberadamente. Lo único que queda es
  el fichero `version` con `v0.1.19`: una cadena de versión, no un commit. De los 25 ficheros
  del cierre que pertenecen a OmegaClaw-Core, **ninguno tiene procedencia recuperable desde el
  proceso**; solo integridad.

Consecuencia para E-4: el límite de "integridad sin procedencia" existe, pero hay que declararlo
sobre el sistema objetivo, no sobre el sustrato. Es una línea distinta del informe.

### 2.7 Los dos `git-import!` no clonan nunca en producción (CAPTURADO — paso 2, parcial)

En la imagen publicada existen ya `/PeTTa/repos/OmegaClaw-Core/` y
`/PeTTa/repos/petta_lib_chromadb/`. Como `git-import!` es
`( exists_directory(LocalDir) -> true ; clone_repo(...) )`, ambas llamadas **encuentran el
directorio y no hacen nada**: ni clonan ni actualizan.

La fijación efectiva no está en el `git-import!` en absoluto: está en el `docker build`, y ahí
`petta_lib_chromadb` se fija a `master`. Queda por observar en el arranque real si la llamada
se evalúa en carga o perezosamente — pero para el lockfile da igual, porque en ningún caso trae
código nuevo.

### 2.8 Datos en crudo pendientes de una sesión de trabajo

- Momento real de ejecución del `git-import!` (carga vs primer uso): *(pendiente)*
- Re-hash de los 34 ficheros tras una sesión de trabajo; ¿cambió alguno? *(pendiente)*
- Artefactos `.pl`/`.qlf` generados durante la sesión: *(pendiente)*
- Procedencia recuperable **desde el proceso en ejecución**:

Procedencia: resuelta en §2.6. Los cinco `lib_*` la tienen; los ficheros de OmegaClaw-Core no.

### Decisión derivada

*(pendiente del paso 3 — re-hasheo tras una sesión de trabajo real)*

Pasos 1, 2 y 4 cerrados con datos. Lo que ya está determinado:

- **El lockfile es un manifiesto estático de pares (ruta, hash), generado antes de la
  ejecución.** El cierre es enumerable y la enumeración coincide 34/34 con el artefacto
  desplegado. No hace falta instrumentar el cargador de PeTTa.
- **El manifiesto cubre el cierre completo, no solo los `.metta`.** 13 de los 34 son Python.
- **El manifiesto lleva procedencia para el sustrato y solo integridad para el sistema
  objetivo.** PeTTa y `petta_lib_chromadb` conservan su `.git` y su commit es recuperable;
  OmegaClaw-Core no, porque el Dockerfile lo borra a propósito.
- **El `git-import!` no es el punto donde fijar nada**: en producción nunca clona. Lo que hay
  que fijar es el `docker build`, donde `CHROMADB_REF=master` es la revisión no fijada real.

Falta el paso 3 para cerrar: comprobar que ninguno de los 34 cambia durante una sesión, y
separar *fuente modificada* de *artefacto `.pl`/`.qlf` derivado*.

---

## 3. Captura 3 — Proporción fiel/libre en cadena real

### Hallazgos estáticos previos

**El bucle mutila el string antes de que lo vea el LLM, y no solo por longitud.** De
`src/utils.metta`:

```metta
(= (string-safe $str)
   (string-replace (string-replace (string-replace $str "\"\"" "_quote_") "\n" "_newline_") "'" "_apostrophe_"))
```

`string-safe` se aplica al escribir `&lastresults` y al ensamblar todo el contexto en
`getContext`. Sustituye comillas dobles dobladas, **saltos de línea** y apóstrofos por
literales `_quote_`, `_newline_`, `_apostrophe_`.

Consecuencia directa sobre el predicado de eslabón fiel de E-2, que es comparación de bytes:
**una conclusión con salto de línea nunca puede coincidir byte a byte con la premisa
realimentada**, porque la segunda pasó por la sustitución y la primera no. Esto no es
paráfrasis del LLM: es transformación determinista del bucle, y hay que descontarla del
numerador antes de medir nada. Si no se descuenta, la proporción fiel medida es artificialmente
baja por una razón que no tiene que ver con el mediador.

Además, `helper.normalize_string` codifica y decodifica con `errors="ignore"`, lo que
**descarta silenciosamente** cualquier byte que no sea UTF-8 válido.

### Datos en crudo

Una fila por eslabón, comparación **byte a byte**. Cuando no hay coincidencia literal, guardar
el par de strings completo para clasificar el tipo de reescritura. Añadida columna para separar
la transformación del bucle de la reescritura del mediador.

| # | Tarea | Salida salto 1 (conclusión comprometible) | Entrada salto 2 (premisa arrastrada) | ¿Literal? | ¿Coincide tras normalizar `string-safe`? | Tipo de reescritura |
|---|---|---|---|---|---|---|
| 1 | *(pendiente)* | | | | | paráfrasis / resumen / traducción de formato / invención |

- Tareas usadas (3–5, cada una con dos saltos como mínimo): *(pendiente)*
- Proporción fiel/libre resultante: *(pendiente)*

### Decisión derivada

*(pendiente — bloqueada por §0.1 y §0.2)* — naturaleza de `TR-CHAIN-MEDIATED` en el informe
(E-1, E-2).

---

## 4. Captura 4 — Truncamiento de la salida

### Hallazgos estáticos previos: hay truncamiento, y es por la cabeza

De `src/loop.metta`, ensamblado del contexto:

```metta
" LAST_SKILL_USE_RESULTS: " (last_chars (get-state &lastresults) (maxFeedback))
" HISTORY: " (getHistory)
```

De `src/utils.metta`:

```metta
(= (last_chars $String $N)
   (let $Len (string_length $String)
        (sub_string $String (max 0 (- $Len $N)) $_ 0)))
```

De `config/config.yaml` y `src/memory.metta`:

```
maxFeedback: 50000     ; caracteres de LAST_SKILL_USE_RESULTS
maxHistory:  30000     ; read_file_tail sobre memory/history.metta
```

**`last_chars` conserva los últimos N caracteres: el corte cae en la cabeza del string.** Si
una inferencia devuelve más de 50 000 caracteres, lo que el LLM ve empieza a mitad de
conclusión, y lo que se pierde es el principio, no el final. `getHistory` hace lo mismo sobre
el fichero de historia con `read_file_tail` y 30 000.

Las tres etapas del procedimiento quedan localizadas:

| Etapa | Dónde | Transformación aplicada |
|---|---|---|
| 1. Salida del eval | `(let $R (eval $s) (py-call (helper.normalize_string $R)))` en `src/loop.metta` | `normalize_string`: descarta bytes no UTF-8 (`errors="ignore"`) |
| 2. `&lastresults` | `(change-state! &lastresults (string-safe (repr $results)))` | `string-safe`: sustituye `\n`, `""`, `'` |
| 3. Contexto LLM | `(last_chars ... (maxFeedback))` dentro de `(string-safe (py-str (...)))` | Truncamiento por la cabeza a 50 000 caracteres |

### Datos en crudo

| Etapa | Longitud del string | ¿Hay corte? |
|---|---|---|
| Salida de `(repr (swrite (eval $code)))` | *(pendiente)* | *(pendiente)* |
| Ensamblado en `LAST_SKILL_USE_RESULTS` | *(pendiente)* | *(pendiente)* |
| Contexto final enviado al LLM | *(pendiente)* | *(pendiente)* |

La medición sigue haciendo falta: el código dice que el corte **existe y dónde está**, no a
partir de qué tamaño de conjunto de conclusiones se alcanza en la práctica. El bucle registra
`CHARS_SENT` en el log, lo que da la longitud de la etapa 3 sin instrumentar nada.

### Decisión derivada

*(pendiente de medición — pero el material estático ya la orienta con fuerza)*

El punto de captura para el compromiso **no puede ser el string del contexto LLM**: sufre dos
transformaciones deterministas (`normalize_string`, `string-safe`) y un truncamiento por la
cabeza a 50 000 caracteres. Comprometer eso es comprometer una conclusión mutilada, y la
recomputación no cerraría nunca por un motivo ajeno a la semántica. La captura tiene que ser la
salida cruda de `(eval $s)`, antes de `normalize_string`, y el string del contexto LLM se
registra como artefacto separado.

Falta el dato de ejecución que convierte esta orientación en decisión: a partir de qué tamaño
real de resultado se alcanzan los 50 000 caracteres.

---

## 5. Corroboración de E-5 (estático, no es una de las cuatro capturas)

La enmienda E-5 afirma que el `lib_pln` que OmegaClaw ejecuta no tiene maquinaria de stamps.
Comprobado sobre los clones del §0.3:

| Fichero | Líneas | Coincidencias de `stamp` / `evidence` / `EvID` |
|---|---|---|
| `OmegaClaw-Core/lib_pln.metta` | 309 | **0** |
| `PeTTa/lib/lib_pln.metta` (v1.0.4) | 466 | 9 |

E-5 queda corroborada: el de producción es el de 309 líneas y no tiene tracking de evidencia,
mientras que el del sustrato sobre el que corre sí lo tiene. La divergencia no es de versión.

---

## 6. Estado de la compuerta

| Captura | Datos | Decisión escrita |
|---|---|---|
| 1 — Estabilidad del collapse | ✗ — pendiente de `omegaclaw:mock` | ✗ |
| 2 — Cierre de importaciones | Pasos 1, 2 y 4 capturados (§2.1, §2.5–2.7); falta el paso 3 | ✗ |
| 3 — Proporción fiel/libre | ✗ — **requiere clave LLM real** | ✗ |
| 4 — Truncamiento | Mecanismo y límites localizados (§4); medición ✗ | ✗ |

**Compuerta cerrada, 4/4.** Trabajo desbloqueado cuando las cuatro filas estén completas:

- Especificación del lockfile (E-4, forma según captura 2).
- Predicado de igualdad del verificador (según captura 1).
- Rediseño final de C2 (según capturas 3 y 4).

### Qué falta, y de quién depende

| Pendiente | Bloqueado por |
|---|---|
| Capturas 1 y 4, y el paso 3 de la 2 | Construcción local de `omegaclaw:mock` (en curso). No necesita nada del operador |
| Captura 3 | **Clave de proveedor LLM**, por canal fuera de banda. Es lo único que sigue esperando al operador |
