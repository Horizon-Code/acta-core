# Revisión de silencios en los perfiles registrados

**Fecha:** 3 de septiembre de 2026
**Naturaleza:** research no normativo. Aplica el principio 4.16 de
`architecture/profile-architecture-v1.0.md` a los perfiles existentes. **No modifica ninguno.**
**Lente:** qué queda fuera del registro y quién se beneficia de ese silencio.

## `ai_agent` v1.0 — ya diagnosticado

Seis event kinds, los seis describen acciones ejecutadas. Ninguno describe un intento
frustrado. Medido en S9 y recogido en Proposed ADR-015; no se repite aquí.

## `agent_commerce` v1.0 — tres silencios, uno declarado y dos no

Siete event kinds: `mandate_received`, `action_executed_against_mandate`, `delivery_committed`,
`delivery_received`, `settlement_requested`, `settlement_observed`, `dispute_opened`.

Este perfil parte con ventaja: `dispute_opened` **sí** es un evento negativo, cosa que
`ai_agent` no tiene. Y ADR-010 declara explícitamente que no define evento terminal, «so
settlement observation does not silently become closure». Eso es exactamente lo que 4.16 pide:
un silencio declarado. Los otros dos no lo están.

### Silencio 1 — no hay acción rechazada contra el mandato (no declarado)

Existe `action_executed_against_mandate`. No existe su contrario: una acción que se intentó
contra el mandato y un control rechazó. Es la misma forma exacta del hueco de `ai_agent`.

**Quién se beneficia:** el operador del agente. Un intento bloqueado de exceder el mandato es
precisamente la evidencia que una contraparte querría ver, y es la que hoy no puede existir. El
registro solo puede mostrar un agente que se portó bien, nunca uno al que hubo que frenar.

**Agravante respecto a `ai_agent`:** aquí el mandato es el objeto central del perfil. Un perfil
construido alrededor de un mandato que no puede registrar sus violaciones intentadas tiene el
hueco en su propio centro.

### Silencio 2 — la disputa se abre pero no se resuelve (no declarado)

`dispute_opened` no tiene contrapartida. No hay `dispute_resolved`, `dispute_withdrawn` ni
`dispute_rejected`. Una disputa abierta y luego resuelta deja constancia únicamente de su
apertura.

**Quién se beneficia — y aquí la dirección se invierte:** el que abre la disputa. La acusación
es registrable; la exoneración no. Quien resulta absuelto arrastra en el registro una disputa
abierta permanente sin forma de constar que se cerró a su favor.

Esto merece subrayarse porque contradice la intuición de que estos huecos siempre protegen al
operador. Aquí perjudican al acusado, y una asimetría que solo permite registrar la acusación es
un defecto probatorio con dirección propia.

### Silencio 3 — la transacción incompleta es un ciclo de vida válido (parcialmente declarado)

El validador exige el evento previo pero nunca el siguiente. `delivery_committed` sin
`delivery_received`, o `settlement_requested` sin `settlement_observed`, son ciclos de vida
válidos. El registro simplemente se para.

**Quién se beneficia:** quien incumplió. Un vendedor que nunca entregó y un comprador que nunca
pagó producen el mismo registro: uno que termina antes.

**Por qué está solo parcialmente declarado:** ADR-010 declara que no hay evento terminal, y eso
es honesto. Pero declarar que no se afirma cierre **no es lo mismo que hacer contable el hueco**.
Un lector sabe que no debe inferir cierre; no sabe si faltan eventos ni cuántos. Las piezas 2 y 3
de ADR-015 —intervalos de cobertura y cobertura declarada por el perfil— convertirían este
silencio de declarado a contable. Es el mismo mecanismo, aplicable sin cambiar el vocabulario de
`agent_commerce`.

## Conclusión

`agent_commerce` no se modifica en esta sesión: cambiar un perfil aceptado sigue las reglas de
versionado de ADR-010 y es decisión del operador.

Lo que sí se puede afirmar es que el sesgo que 4.16 describe **no fue exclusivo de `ai_agent`**.
Los dos perfiles de dominio del repositorio lo tienen, en distinto grado, y el segundo lo tiene
en su concepto central. Eso sostiene 4.16 como principio de diseño y no como reacción a un caso
aislado.

Si ADR-015 se ratifica, conviene decidir si `agent_commerce` recibe un tratamiento equivalente en
su propia versión, o si el evento de denegación y los intervalos de cobertura se especifican una
sola vez a nivel de arquitectura de perfiles para que ambos los hereden. La segunda opción parece
mejor y evita dos vocabularios divergentes para el mismo problema, pero es decisión del operador.
