
# ACTA — Roadmap de Mejoras Futuras
## Evolución post-MVP del proceso AML y revisión manual

---

## 0. Propósito del documento

Este documento describe **exclusivamente las mejoras futuras** sobre el VMVP de ACTA.  
No redefine ni modifica el MVP ya cerrado.

Su objetivo es:
- Documentar **cómo evoluciona ACTA** sin romper el core
- Cubrir escenarios avanzados (humanos, disputas, ZK, responsabilidad)
- Servir como **guía técnica y estratégica** para siguientes fases

---

## 1. Punto de partida (estado MVP)

En el MVP:

- El proceso AML es **completo y cerrado**
- `manual_review` existe como evento opcional
- `manual_review` está:
  - firmado por la **entidad**
  - con autoría humana **indirecta**
- No existe:
  - firma humana individual
  - identidad personal verificable
  - atribución de responsabilidad individual

Estas limitaciones son **intencionadas** para reducir complejidad inicial.

---

## 2. Motivación de las mejoras

En disputas avanzadas pueden surgir preguntas como:

- ¿Quién revisó exactamente el caso?
- ¿Estaba autorizado en ese momento?
- ¿Qué información tuvo disponible?
- ¿Puede atribuirse responsabilidad individual?
- ¿Hubo desacuerdo entre analistas?
- ¿Se corrigió un error humano?

Las siguientes fases responden a estas preguntas.

---

## 3. Fase 2 — Manual Review con autoría humana verificable

### 3.1 Human Actor Profile (HAP)

Introducción de un **perfil humano verificable**.

Campos orientativos:
- `hap_id`
- `role` (aml_analyst, senior_analyst, supervisor)
- `organization`
- `valid_from`
- `valid_to`
- `capabilities` (confirm_freeze, release, escalate)

El HAP representa **una persona concreta**, no solo un rol.

---

### 3.2 Firma humana individual

Evolución del evento `manual_review`:

- Doble firma:
  - firma institucional (responsabilidad legal)
  - firma humana (responsabilidad operativa)
- La firma humana:
  - no sustituye a la institucional
  - la complementa

Permite atribución clara de decisiones humanas.

---

### 3.3 Gestión de claves humanas

Requisitos adicionales:

- Emisión y revocación de claves personales
- Rotación periódica
- Gestión de bajas y cambios de rol
- Auditoría de uso de claves

Implicaciones:
- PKI corporativa o DID
- Integración con sistemas IAM

---

## 4. Fase 3 — Contexto cognitivo y justificativo

### 4.1 Context Bundle

Introducción de un `review_context_bundle`:

Incluye hashes de:
- documentos consultados
- pantallas o snapshots
- políticas secundarias
- comunicaciones internas relevantes

Todo bajo modelo **hash-only por defecto**.

---

### 4.2 Justificación estructurada

Evolución del outcome humano hacia un esquema formal:

```json
{
  "decision": "CONFIRM_FREEZE",
  "justification_schema": "AML-REVIEW-V2",
  "justification_hash": "sha256:..."
}
```

Permite:
- explicabilidad estructurada
- comparación entre revisiones
- análisis ex-post

---

## 5. Fase 4 — Disputa, contradicción y corrección

### 5.1 Eventos de contradicción

Nuevos tipos de eventos:

- `review_challenged`
- `secondary_review`
- `review_overturned`

Permiten:
- trazabilidad de desacuerdos
- corrección de errores humanos
- análisis de calidad operativa

---

### 5.2 Responsabilidad distribuida

Modelo escalonado:

- analista decide
- segundo analista valida
- supervisor cierra

Cada paso:
- evento ACTA independiente
- firma diferenciada
- Chronos detecta omisiones

---

## 6. Privacidad avanzada y ZK (fases posteriores)

Posibles extensiones:

- Identidad humana revelable solo bajo orden judicial
- Selective disclosure mediante ZK
- Integración con sidechains de privacidad
- Pruebas de rol sin revelar identidad

---

## 7. Beneficio estratégico para ACTA

Estas mejoras permiten que ACTA evolucione hacia:

- infraestructura de **responsabilidad híbrida**
- soporte completo humano-máquina
- alta defendibilidad legal y regulatoria

Sin:
- romper el core
- inflar el MVP
- comprometer neutralidad técnica

---

## 8. Resumen ejecutivo

- El MVP prueba **procesos automáticos**
- Las fases futuras prueban **decisiones humanas**
- El diseño actual es compatible con esta evolución
- ACTA escala sin reescrituras

---

## FIN DEL DOCUMENTO
