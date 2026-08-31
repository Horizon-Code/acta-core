# E0 — Resultados

**Estado:** NO EJECUTADO. Ninguna captura tiene datos todavía.
**Protocolo:** `roadmap/E0-protocolo-y-enmiendas.md`, Parte 1.
**Autoridad:** material exploratorio y empírico. No redefine `architecture/` ni `decisions/`.

Este fichero es el entregable de la compuerta E0 (§1.6 del protocolo). Mientras alguna de las
cuatro decisiones siga vacía, la compuerta está **cerrada**:

> Ni una línea de C2 antes de cerrar E0.

Regla de registro: **todo en crudo**. Strings completos con timestamps, o enlace al log. Nada
de resúmenes — el resumen es la decisión, y va en su apartado, no en la captura.

---

## 0. Datos de preparación

Estos datos no son metadatos del experimento: son **su primer resultado**. Son la primera
muestra del problema de procedencia del cierre de importaciones (E-4).

| Dato | Valor | Cómo se obtuvo |
|---|---|---|
| Commit de `asi-alliance/OmegaClaw-Core` | *(pendiente)* | |
| Commit de `trueagi-io/PeTTa` instalado | *(pendiente)* | |
| Commit HEAD de `patham9/petta_lib_chromadb` | *(pendiente)* | |
| Versión de SWI-Prolog | *(pendiente)* | |
| Versión de Python | *(pendiente)* | |
| Proveedor y modelo LLM configurado | *(pendiente)* | |
| Fecha y hora de la sesión | *(pendiente)* | |

### 0.1 Punto de intercepción

Paso 5 del bucle de turno (eval de skills). Registrar el string exacto que **entra** en la
skill `metta` y el string exacto que **devuelve**, antes de ensamblarse en el contexto del LLM.

- ¿Ofrece el paso 5 punto de intercepción? *(pendiente: sí / no)*
- Si no: mecanismo elegido — plugin vs canal `wschat` intermediario, y por qué. *(pendiente)*
- Referencia de la documentación consultada (`reference-internals-loop.md`,
  `reference-plugin-api.md`): *(pendiente)*

---

## 1. Captura 1 — Estabilidad del collapse

### Datos en crudo

- Premisas usadas: *(pendiente)*
- Ejecución 1 (misma sesión) — salida completa: *(pendiente)*
- Ejecución 2 (misma sesión) — salida completa: *(pendiente)*
- Ejecución 3 (tras reiniciar el contenedor) — salida completa: *(pendiente)*
- Caso de conclusión duplicada con TV distinto — construcción y salida: *(pendiente)*
- ¿`unique-atom` deja una o las dos? *(pendiente)*

### Decisión derivada

*(pendiente)* — predicado de verificación de recomputación. Opciones del protocolo:
igualdad de serialización canónica / igualdad de conjuntos o pertenencia / comparación de
pares (conclusión, TV).

---

## 2. Captura 2 — Cierre de importaciones: ¿estático o dinámico?

### Datos en crudo

- Enumeración previa desde disco de los `.metta` que `lib_omegaclaw.metta` importa, directa y
  transitivamente, con SHA-256 de cada uno: *(pendiente)*
- Momento de ejecución de `!(git-import! "...petta_lib_chromadb.git")` — ¿en carga o
  perezosamente en el primer uso? *(pendiente)*
- Re-hash de los mismos ficheros tras una sesión de trabajo; ¿cambió alguno? *(pendiente)*
- Procedencia recuperable **desde el proceso en ejecución** para cada fichero cargado desde la
  instalación de PeTTa:

  | Fichero | Hash SHA-256 | Procedencia (commit / versión pip) | ¿Recuperable? |
  |---|---|---|---|
  | `lib_patrick` | *(pendiente)* | *(pendiente)* | *(pendiente)* |
  | `lib_llm` | *(pendiente)* | *(pendiente)* | *(pendiente)* |
  | `lib_vector` | *(pendiente)* | *(pendiente)* | *(pendiente)* |
  | `lib_combinatorics` | *(pendiente)* | *(pendiente)* | *(pendiente)* |
  | `lib_he` | *(pendiente)* | *(pendiente)* | *(pendiente)* |

### Decisión derivada

*(pendiente)* — forma del lockfile (E-4). Opciones del protocolo: manifiesto estático de pares
(ruta, hash) con `git-import!` resuelto a commit / captura en runtime instrumentando el
cargador de PeTTa / integridad sin procedencia declarada como límite en el informe.

---

## 3. Captura 3 — Proporción fiel/libre en cadena real

### Datos en crudo

Una fila por eslabón, comparación **byte a byte**. Cuando no hay coincidencia literal, guardar
el par de strings completo para clasificar el tipo de reescritura.

| # | Tarea | Salida salto 1 (conclusión comprometible) | Entrada salto 2 (premisa arrastrada) | ¿Literal? | Tipo de reescritura |
|---|---|---|---|---|---|
| 1 | *(pendiente)* | | | | paráfrasis / resumen / traducción de formato / invención |

- Tareas usadas (3–5, cada una con dos saltos como mínimo): *(pendiente)*
- Proporción fiel/libre resultante: *(pendiente)*

### Decisión derivada

*(pendiente)* — naturaleza de `TR-CHAIN-MEDIATED` en el informe (E-1, E-2). Opciones del
protocolo: proporción fiel alta → línea excepcional con auditoría eslabón a eslabón /
proporción fiel baja o paráfrasis sistemática → paisaje por defecto y C2 se cuenta desde ahí.

---

## 4. Captura 4 — Truncamiento de la salida

### Datos en crudo

Inferencia con resultado de decenas de conclusiones, seguida por las tres etapas:

| Etapa | Longitud del string | ¿Hay corte? |
|---|---|---|
| Salida de `(repr (swrite (eval $code)))` | *(pendiente)* | *(pendiente)* |
| Ensamblado en `LAST_SKILL_USE_RESULTS` | *(pendiente)* | *(pendiente)* |
| Contexto final enviado al LLM | *(pendiente)* | *(pendiente)* |

### Decisión derivada

*(pendiente)* — punto de captura para el compromiso. Si la salida puede truncarse, comprometer
el string que ve el LLM es comprometer una conclusión mutilada, y la recomputación no cierra
nunca por un motivo ajeno a la semántica: el punto de captura debe ser anterior al
truncamiento (la salida cruda del eval), y el string truncado del contexto LLM se registra
como artefacto separado.

---

## 5. Estado de la compuerta

| Captura | Datos | Decisión escrita |
|---|---|---|
| 1 — Estabilidad del collapse | ✗ | ✗ |
| 2 — Cierre de importaciones | ✗ | ✗ |
| 3 — Proporción fiel/libre | ✗ | ✗ |
| 4 — Truncamiento | ✗ | ✗ |

**Compuerta cerrada.** Trabajo desbloqueado cuando las cuatro filas estén completas:

- Especificación del lockfile (E-4, forma según captura 2).
- Predicado de igualdad del verificador (según captura 1).
- Rediseño final de C2 (según capturas 3 y 4).
