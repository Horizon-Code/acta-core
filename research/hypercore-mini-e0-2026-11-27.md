# Hypercore mini-E0 — what it guarantees before ACTA rides on it

> **Naturaleza:** research, no normativo. Medido el 27 de noviembre de 2026 con
> `hypercore` 11.40.5 sobre Node.js 22 y sobre Bare 1.39.0. Reproducible con
> `adapters/x402-wdk/js/test/transport.test.mjs`.

**Pregunta:** Hypercore es un registro firmado de solo-añadir. ¿Hace eso redundante la cadena
Chronos/Merkle de ACTA, y se puede usar como transporte de bundles entre las partes?

## Medido

1. **Los bloques llegan byte a byte.** Lo que el escritor añade es lo que el lector lee.
2. **Solo el poseedor de la clave escribe.** Un lector no puede añadir al log.
3. **El escritor puede reescribir su historia.** `truncate(n)` seguido de `append` sustituye
   bloques ya replicados.
4. **El lector adopta la historia nueva sin error.** Tras la reescritura, `get(1)` en el lector
   devuelve el bloque nuevo. No hay excepción, aviso ni evento que lo distinga de un añadido.
5. **Lo único que se mueve es el contador `fork`** (de 0 a 1) y el hash del árbol.
6. **Recorder y transporte cargan y funcionan en Bare**, con una corrección: Bare no tiene
   `TextEncoder` ni `TextDecoder`. Sin esa prueba, el recorder fallaba en el lado vendedor.

## Qué se deduce (inferencia)

- Hypercore prueba **quién escribió** un bloque. No prueba que **lo que se lee hoy sea lo que
  se leyó ayer**. Frente a su propio escritor no es evidencia: es un canal.
- Por tanto no sustituye a la cadena de ACTA y no debe presentarse como custodia. Es el mismo
  fallo que el fichero de auditoría de OpenWorker en S9, con firma.
- Sí sirve como transporte, con una condición: quien lee guarda un punto de control
  (clave, longitud, `fork`, hash del árbol) y lo compara después. Eso es lo que hace
  `compareWithCheckpoint`.
- La protección real sigue viniendo de fuera del log: la copia que retiene la contraparte y,
  cuando exista, el anclaje de la raíz de época.

## No medido

- Descubrimiento de pares por red (Hyperswarm / HyperDHT): los dos logs se conectaron en
  proceso. Desde este entorno no hay salida a la DHT.
- Autobase, Hyperbee y multi-escritor.
- Comportamiento con cifrado de bloques.
