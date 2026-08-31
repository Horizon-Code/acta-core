# Guion de validación — tres conversaciones

**Fecha de preparación:** 2026-08-31
**Responsable de las conversaciones:** Rub (`[RUB]`, no delegable)
**Pregunta de decisión:** ¿existe hoy alguien que responde legalmente por un sistema
automatizado y que no puede reconstruir por qué hizo lo que hizo?

Este guion no presenta ACTA ni busca aprobación de una solución. Busca un caso pasado, el
procedimiento actual, su coste y quién responde. Tres opiniones favorables sin hechos no
desbloquean A2.

**Nota de gobernanza:** E-9 propone redefinir los interlocutores y la pregunta central, pero
mientras siga `Proposed` este guion y su regla Cardano/A2 continúan vigentes sin cambios. No se
ejecuta la variante E-9.8 hasta ratificación explícita del operador.

## Selección de interlocutores

1. **Banca:** MLRO, responsable de compliance, DPO o auditor interno que firme informes.
2. **Sanidad:** DPO, responsable de cumplimiento o seguridad clínica que responda por una
   decisión automatizada.
3. **Contraste:** auditor interno/externo de banca o sanidad que firme conclusiones y revise
   evidencias producidas por otros.

No sustituirlos por vendedores, consultores sin responsabilidad de firma ni perfiles técnicos
que solo produzcan los logs.

## Apertura común (2 minutos)

> No te quiero vender una solución. Necesito entender cómo reconstruís hoy una decisión
> automatizada cuando alguien la cuestiona, qué evidencia os basta y quién responde si no se
> puede reconstruir.

Pedir permiso para tomar notas y conservar una síntesis anonimizada. No explicar ACTA antes de
recoger el caso y el procedimiento actuales.

## Conversación 1 — banca (25–30 minutos)

1. Cuéntame el último caso real en que cliente, auditor o supervisor cuestionó una decisión
   automatizada. ¿Qué decisión era y quién tuvo que responder?
2. ¿Qué evidencias recuperasteis exactamente: entradas, logs, reglas, versión del modelo,
   aprobaciones, firmas y marcas temporales?
3. ¿Qué no pudisteis reconstruir? ¿Cómo cerrasteis ese hueco en el informe?
4. ¿Cuántas personas, horas o días costó preparar la respuesta?
5. ¿Quién firmó finalmente? ¿Qué responsabilidad asumió esa persona?
6. Cuando el operador afirma que sus logs son completos y no fueron alterados, ¿cómo lo
   comprobáis sin depender de su palabra?
7. ¿Os bastan hoy los logs más un procedimiento firmado? Describe el caso en que dejarían de
   bastar.
8. ¿Qué tendría que contener un expediente para que un tercero lo verificase sin llamar al
   proveedor?
9. ¿Quién controla el presupuesto para reducir este riesgo? ¿Qué evento activa gasto: examen
   supervisor, incidente, reclamación, auditoría o renovación?
10. Compara el valor con un coste actual: una investigación manual, una auditoría externa, una
    provisión o una sanción. ¿Qué rango sería razonable?
11. Si el proveedor desaparece mañana, ¿podéis seguir verificando el expediente?
12. ¿Quién más firma realmente este tipo de respuesta y debería ser entrevistado?

## Conversación 2 — sanidad (25–30 minutos)

1. Cuéntame el último caso real en que paciente, profesional, DPO o autoridad cuestionó una
   decisión automatizada. ¿Afectaba acceso, prioridad, diagnóstico, tratamiento o facturación?
2. ¿Quién debía reconstruirla y quién respondía legalmente?
3. ¿Qué evidencias existían y cuáles dependían del proveedor del sistema?
4. ¿Podíais fijar qué versión de reglas/modelo y qué datos se usaron en ese instante?
5. ¿Qué huecos quedaron y cómo se expresaron ante el afectado o la autoridad?
6. ¿Cuánto tiempo y cuántas personas consumió la reconstrucción? ¿Hubo impacto clínico u
   operativo por la demora?
7. ¿Cómo verificáis que un log no fue seleccionado, alterado o completado después?
8. ¿Os bastarían logs más procedimiento firmado? ¿Qué evidencia exigirías en un caso con daño?
9. ¿Necesitáis verificar offline si el proveedor deja de colaborar o desaparece?
10. ¿Quién pagaría por esa independencia y contra qué coste actual se compararía?
11. ¿Qué afirmaciones jamás permitirías que hiciera un dossier técnico sobre una decisión
    clínica?
12. ¿Quién más firma estas respuestas y debería ser entrevistado?

## Conversación 3 — auditor que recibe evidencia (25–30 minutos)

1. Describe el último expediente de decisión automatizada que tuviste que aceptar o rechazar.
2. ¿Qué afirmaba el operador y qué pudiste verificar de forma independiente?
3. ¿Qué comprobaciones hiciste sobre integridad, completitud, orden temporal, identidad del
   firmante y versión de reglas/modelo?
4. ¿Qué parte acabó siendo una manifestación de la dirección en vez de evidencia?
5. ¿Qué deficiencia habría cambiado tu conclusión o generado una salvedad?
6. ¿Los logs más un procedimiento firmado son suficientes? ¿Bajo qué controles?
7. ¿Qué necesitas conservar para repetir la verificación años después y sin acceso al emisor?
8. ¿Cuánto cuesta hoy revisar y conciliar manualmente este material?
9. ¿Quién compra la mejora: auditado, auditor, asegurador o supervisor? ¿Cuál es el disparador?
10. ¿Qué pagarían por reducir dependencia de manifestaciones del operador? Pedir rango o unidad
    de comparación.
11. Si recibieras un dossier autocontenido, firmado y verificable offline, ¿qué seguiría sin
    probar? Esta pregunta comprueba si el informe de confianza residual resulta comprensible.
12. ¿Qué otro firmante debería ser entrevistado?

## Ficha de evidencia por conversación

```text
ID / fecha:
Sector y función (sin nombre si se anonimiza):
¿Firma informes?: sí / no — qué tipo:
Caso pasado concreto:
Quién cuestionó la decisión:
Quién respondió y firmó:
Evidencia disponible:
Hueco no reconstruible:
Solución provisional usada:
Coste (personas / tiempo / dinero):
¿Logs + procedimiento bastan?: sí / no / depende — condición:
Dependencia de la palabra del operador:
Necesidad de verificación offline:
Comprador y disparador presupuestario:
Rango o coste comparable:
Frase casi literal autorizada:
Referido siguiente:
Señal: fuerte / débil / contradice la hipótesis:
```

## Regla de decisión sobre A2

- **Sí, desbloquear A2:** las tres evidencias permiten responder por escrito que existe un
  firmante responsable con un caso no reconstruible o dependiente de la palabra del operador;
  al menos dos aportan coste concreto, comprador o disparador presupuestario.
- **Incierto, no desbloquear aún:** existe dolor, pero no hay caso pasado, responsable de firma
  o mecanismo de compra. Hacer entrevistas de reemplazo, no construir Cardano real por fe.
- **“Se apañan con logs”, parar:** los responsables aceptan el procedimiento actual, pueden
  verificar lo necesario y el coste marginal no importa. Congelar S5–S7 y replantear antes de
  construir más.

La salida final será un documento corto en `research/` con las tres fichas sintetizadas, las
frases autorizadas y una decisión explícita sobre A2.
