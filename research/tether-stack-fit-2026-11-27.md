# Encaje de ACTA con el stack de Tether (QVAC, WDK, Pears)

> **Naturaleza:** research, no normativo. Nivel de autoridad más bajo del repositorio. No
> cambia el rumbo: cualquier cambio de dirección es una enmienda de roadmap y corresponde al
> operador. Las secciones 1–3 son hechos con fuente o medidos; la sección 4 es inferencia y
> está marcada como tal.

**Fecha:** 27 de noviembre de 2026
**Pregunta:** ¿hay en el stack de Tether un hueco real que ACTA cubra, y qué haría falta para
cubrirlo?

## 1. Qué construye Tether (hechos, con fuente)

- **QVAC**: plataforma para ejecutar agentes de IA en el dispositivo, sin nube, conectados en
  red P2P. Anuncio oficial: tether.io/news (QVAC). SDK publicado el 9-abr-2026.
- **WDK** (Wallet Development Kit): monederos autocustodiados para personas y agentes. Tiene
  MCP Toolkit (`@tetherto/wdk-mcp-toolkit`), Agent Skills y una página propia de **x402**.
  Documentación: docs.wdk.tether.io.
- **x402 en WDK** (docs.wdk.tether.io/ai/x402): el comprador firma una autorización EIP-3009
  con su monedero WDK; un facilitador verifica y liquida. Token USD₮0 en Plasma
  (`eip155:9745`) y Stable (`eip155:988`). El facilitador alojado que citan (Semantic) es de un
  tercero que Tether no opera ni garantiza. La página no documenta los campos del recibo.
- **Pears / Bare / Holepunch**: runtime JavaScript (Bare) y piezas P2P (Hypercore, Hyperswarm,
  Autobase…). Código en otros lenguajes entra como addon nativo. Los módulos WDK declaran
  compatibilidad con Node.js y Bare. Documentación: docs.pears.com.
- **Programa de ayudas** (11-may-2026): por tareas, de unos 1.500 a 4.000 USD cada una, en
  USD₮ o BTC, sobre librerías de QVAC/WDK/Pears, documentación, aplicaciones e investigación.
  Fuente: prensa del anuncio (cryptonomist.ch, ffnews.com). **Las bases no se han leído
  directamente.**
- **Oobit** (respaldada por Tether): tarjetas Visa para agentes con "audit trail" por tarjeta.
  Es un registro del operador.

## 2. Qué se ha medido aquí

Un intercambio x402 v2 real en local: monedero WDK (`@tetherto/wdk-wallet-evm`
1.0.0-beta.27) firmando una autorización EIP-3009 real, cliente y middleware reales
(`@x402/*` 2.26.0), **facilitador sustituto** (no se liquidó nada en ninguna cadena).

- Comprador y vendedor tienen copias idénticas byte a byte de `PAYMENT-SIGNATURE`,
  `PAYMENT-RESPONSE` y el cuerpo de la respuesta.
- Solo el comprador tiene `PAYMENT-REQUIRED`.
- x402 no ofrece canal para que el vendedor entregue evidencia al comprador.
- El perfil `agent_commerce` v1.0 expresa el intercambio completo con sus siete tipos de
  evento, salvo el rechazo de un pago.

Registro y límites: `adapters/x402-wdk/README.md`.

## 3. Qué tiene ACTA y qué le falta para este stack

| Pieza | Estado |
|---|---|
| Evidencia de transacción entre agentes | Hecho (perfil `agent_commerce`, experimental) |
| Informe para máquinas y requisitos negociables | Hecho (experimental) |
| Conector x402/WDK → eventos ACTA | **Candidato de implementación (S10, ADR-019 Proposed)** |
| Verificar el recibo de liquidación contra la cadena | No existe. Añade un sustrato: requiere enmienda del operador |
| Transporte de bundles entre las partes | No existe |
| Ejecutar el verificador en Bare/Pear | No existe. Vía prevista por Pear: addon nativo |
| Anclaje donde vive USD₮0 | No existe. Hoy solo Base Sepolia |
| Identidad de quien firma | Congelada (`roadmap/next-milestones.md`, punto 5) |

Solape a tener presente: Hypercore ya es un registro firmado de solo-añadir. **No verificado
en esta sesión**; antes de diseñar el transporte hay que medir qué garantiza y qué no, como se
hizo con SCITT en el mini-E0.

## 4. Síntesis — INFERENCIA ESTRATÉGICA

Nada de lo que sigue es un hecho.

- **El hueco existe y no es exclusivo de Tether.** x402 cubre autorización y liquidación y no
  la ejecución; es la lectura de E-9, ahora medida sobre un intercambio real. Que Tether lo
  perciba como hueco no consta en ninguna fuente.
- **En un stack sin servidores el hueco pesa más.** Si los agentes corren en dispositivos y
  hablan P2P, no hay operador cuyos registros consultar; la evidencia solo puede estar en manos
  de las partes.
- **Riesgos:** Tether puede construirlo dentro; puede no importarle hasta que haya volumen y
  disputas; ya hay propuestas académicas en la misma zona (TessPay, arXiv 2602.00213); y ACTA
  sigue siendo experimental y en red de pruebas.
- **Vía de entrada plausible:** el programa de ayudas, no una relación comercial. Pendiente de
  leer las bases.

## 5. Orden de construcción propuesto (para decisión del operador)

1. **Conector x402/WDK** — entregado como candidato (S10).
2. **Mapper sobre Profile v1.1**, para expresar el rechazo de un pago.
3. **Mini-E0 de Hypercore**: medir qué garantiza antes de usarlo como transporte.
4. **Transporte** de bundles entre las partes.
5. **Verificación del recibo contra la cadena** — requiere enmienda del operador.
6. **Verificador en Bare** y demostración de extremo a extremo sobre el stack.
