# E-10 — El stack de Tether (WDK, Pears, QVAC) como destino de integración

**Fecha:** 27 de noviembre de 2026
**Tipo:** enmienda de rumbo al roadmap, dentro del marco de E-9
**Estado:** **Accepted — ratificada por el operador bajo ADR-001**
**Ratificación:** 28 de noviembre de 2026, sobre el commit
`596442b3b79c142b1d00ebc9175a75d4a70f010d`
**Autoridad:** enmienda a `roadmap/next-milestones.md` (secuencia de sesiones y orden de
construcción). Subordinada a E-9, a las Foundations y al Protocol. **No toca el Core, el
Protocol v0 ni ningún Profile. No modifica ninguna ADR aceptada.**

> **Qué está decidido.** El operador indicó el 27-nov-2026 que ACTA apunta al stack de Tether
> y ratificó esta enmienda el 28-nov-2026. Los tres conectores que la motivan están ratificados
> (ADR-019, ADR-020, ADR-021). Aceptar E-10 fija el **orden de construcción**; no ratifica
> ninguna ADR posterior, que sigue su propio paso.

---

## 0. Por qué (base de evidencia)

La evidencia fechada está en `research/tether-stack-fit-2026-11-27.md` y
`research/hypercore-mini-e0-2026-11-27.md`. Aquí se separa lo medido de lo inferido; aceptar
E-10 no convierte lo segundo en hecho.

1. **x402 deja abierta la ejecución, y eso ya no es una lectura.** E-9 lo afirmaba leyendo el
   protocolo. S10 lo ejecutó con un monedero WDK: comprador y vendedor retienen copias
   idénticas byte a byte de la autorización, del recibo y de la respuesta, y el protocolo no
   deja constancia de qué se entregó ni ofrece canal para que el vendedor entregue evidencia.
   **Medido:** `adapters/x402-wdk/README.md`.
2. **El servidor MCP del monedero no guarda registro y no impone la vista previa.** Su propia
   documentación lo dice; ejecutado aquí, `wdk-mcp` 1.0.0-beta.9 ejecutó una transferencia que
   ninguna llamada anterior había previsualizado.
   **Medido:** `adapters/wdk-mcp/README.md`.
3. **Hypercore es canal, no custodia.** Su escritor puede truncar y reescribir, y un lector
   adopta la historia nueva sin aviso.
   **Medido:** `research/hypercore-mini-e0-2026-11-27.md`.
4. **En un stack sin servidores la evidencia solo puede estar en manos de las partes.**
   **Inferencia.** No consta en ninguna fuente que Tether lo perciba como hueco.
5. **QVAC está en fase temprana** (SDK 0.7.0) y la inferencia delegada entre pares que
   documenta no lleva pago, recibo ni prueba de ejecución.
   **Leído en documentación, no ejecutado.**

**Tesis de la enmienda:** dentro del posicionamiento de E-9 —evidencia entre autorización y
liquidación—, ACTA toma el stack de Tether como **primer destino de integración concreto**,
con WDK por delante, sin dejar de servir a x402 en general y sin que ninguna pieza de ese
stack pase a ser constitutiva.

## E-10.1 — Orden entre las tres vías

1. **WDK (monedero): activo.** Dos conectores ratificados: pagos x402 (S10) y sesiones MCP
   (S11).
2. **Pears / Hypercore: solo como transporte.** ADR-020 fija que nunca es custodia.
3. **QVAC: en vigilancia.** Disparador para abrir sesión: que la inferencia delegada lleve
   pago, o que publique un punto de enganche para registrar llamadas.

## E-10.2 — Neutralidad de sustrato: sin cambios

La postura de E-9.4 sigue intacta: EVM/Base-first, multi-anclaje, ninguna cadena
constitutiva. Apuntar a Tether **no** convierte Plasma ni Stable en sustrato preferente.
Añadirlas al catálogo de anclaje sigue requiriendo enmienda del operador, y la decisión se
toma en la ADR de modularidad del anclaje, no aquí.

## E-10.3 — Lo que se abre a continuación

| Pieza | Por qué | Requiere |
|---|---|---|
| Verificación del recibo de liquidación contra la cadena que nombra | Convierte `settlement_observed` de "el comprador vio un recibo" en un hecho comprobado. Autorizada por el operador el 27-nov | ADR propia, Proposed |
| Modularidad del anclaje (perfil de cadena configurable) | El operador pidió poder anclar en Base y en la cadena de USD₮0 cuando toque. Hoy el adaptador EAS lleva Base Sepolia como constantes en archivos ratificados | ADR propia que enumere qué tablas de hash habría que refrescar (ADR-012, ADR-013) |
| Core mapping de los eventos v1.1 de `agent_commerce` | El conector x402 emite un rechazo que hoy no llega a Core | Pertenece al Profile; ADR-017 sigue Proposed |
| Verificador como addon de Bare | Verificar dentro del runtime de Pears sin salir a un binario | Sin ADR hasta que haya diseño |

## E-10.4 — Lo que queda fuera, a propósito

- **Nada que decida.** El conector MCP registra y no bloquea (ADR-021 §2). Un componente que
  aplique límites es otro producto y queda fuera de la identidad congelada de ACTA.
- **Identidad.** Sigue congelada como en `roadmap/next-milestones.md`, punto 5.
- **Mainnet.** Fuera de alcance.
- **Afirmaciones de exclusividad.** Ningún material externo dirá que ACTA es lo primero o lo
  único en este espacio. Existen propuestas en la misma zona (TessPay, arXiv 2602.00213).

## E-10.5 — Pendientes del operador que no se delegan

- Cuenta limpia de Base Sepolia para la segunda pasada de anclaje (aplazada por decisión del
  27-nov). Anclará también las raíces de S10 y S11.
- Prueba de descubrimiento de pares por red (Hyperswarm) y un pago real en red de pruebas:
  solo ejecutables desde su equipo. Aplazadas hasta justo antes de enseñar el trabajo, y hasta
  que exista la verificación del recibo, respectivamente.
- Decisión sobre ADR-014, ADR-017 y ADR-018, que siguen Proposed. Los conectores de S10 y S11
  dependen de la implementación v1.1 presentada en ADR-017.

## Riesgos

- **Dependencia de un tercero en fase beta.** WDK, x402 y Hypercore cambian con frecuencia; las
  versiones ejercitadas están fijadas en cada conector y los fixtures retenidos son la
  referencia.
- **Que Tether lo construya dentro**, o que no le importe hasta que haya volumen y disputas.
- **Dispersión.** Tres conectores a tecnología ajena aumentan la superficie de mantenimiento
  sin aumentar la madurez del núcleo: todo sigue `experimental` y en red de pruebas.

## Tabla de impacto — qué toca y qué no

| Elemento | ¿Lo toca E-10? |
|---|---|
| Core, Protocol v0 | No |
| Profiles `ai_agent` y `agent_commerce` | No |
| ADR aceptadas | No |
| Catálogo de anclaje | No (lo decide su propia ADR) |
| Secuencia de sesiones de `next-milestones.md` | Sí: añade S11 y S12 |
| Posicionamiento de E-9 | No: lo concreta en un primer destino |

## Secuencia de sesiones (amplía la de E-9)

| Sesión | Alcance | Puerta |
|---|---|---|
| **S10 — cerrada** | Conector x402/WDK y transporte de evidencia | ADR-019 y ADR-020 Accepted, `experimental` |
| **S11 — cerrada** | Conector MCP del monedero WDK | ADR-021 Accepted, `experimental` |
| **S12** | Perfil de cadena EVM configurable; verificación del recibo de liquidación; propuesta de modularidad del anclaje | ADR Proposed; nada ratificado se modifica |
| **S13** | Anclaje de las raíces de S10/S11 y segunda pasada del Artefacto 1 | Cuenta limpia del operador |

## Ratificación

El operador ratificó explícitamente esta enmienda bajo ADR-001 el 28 de noviembre de 2026,
contra el commit `596442b3b79c142b1d00ebc9175a75d4a70f010d`. La ratificación acepta el orden de construcción y la tabla de
impacto tal como están escritos. No ratifica ADR-022 ni ninguna otra propuesta posterior, ni
convierte en hecho ninguna de las inferencias marcadas en §0.
