# Instrucciones para Copilot — ACTA Core: Proceso AML completo + ejemplo
## Objetivo
Implementar en el core de ACTA el soporte para **procesos completos** (Case/Process) con una **secuencia de eventos** (Chronos) y crear un **ejemplo funcional** del caso AML (con `manual_review` institucional en MVP).  
NO implementar todavía: anclaje on-chain, ZK, identidades humanas con firma personal.

---

## 0) Contexto: modelo elegido
- El caso AML se modela como **proceso completo**: una secuencia ordenada de eventos encadenados.
- El “evento nuclear” es `aml_scored` (mayor carga probatoria), pero hay eventos de apertura/cierre y de ejecución (freeze/release).
- `manual_review` debe existir y estar **cubierto en MVP**, pero:
  - firmado por la **entidad** (no por el humano),
  - con autoría humana indirecta (rol + referencia interna opcional/pseudónima).

---

## 1) Cambios en `types.rs` (core)
### 1.1 Añadir identidad de proceso
Crear `ProcessRefV0`:
- `process_id: String` (ej. `"AML-CASE-2025-000341"`)
- `process_type: String` (ej. `"aml_account_control"`)

### 1.2 Añadir tipo semántico de evento
Crear `EventTypeV0` (snake_case en serde):
- `process_opened`
- `transfer_requested`
- `aml_scored`
- `manual_review`
- `account_frozen`
- `account_released`
- `process_closed`

### 1.3 Extender el evento principal
Extender el struct del evento (ej. `ActaEventV0`) para incluir:
- `process_ref: ProcessRefV0`
- `event_type: EventTypeV0`

Mantener:
- `prev_event_hash: Option<...>` para encadenado Chronos
- commitments existentes (`inputs_commitment`, `outputs_commitment`, `artifact_commitment` o equivalentes)
- `policy_ref`
- `actor_identity_ref` (TAP reference)
- timestamps / epoch id / protocol version

### 1.4 Payload opcional (solo para manual_review en MVP)
Añadir payload opcional para que `manual_review` no sea “ciego”:
- `ManualReviewOutcomeV0`: `confirm_freeze | release | escalate`
- `ManualReviewPayloadV0`:
  - `reviewer_role: String` (ej `"aml_analyst"`)
  - `reviewer_ref: Option<String>` (hash/pseudónimo, opcional)
  - `outcome: ManualReviewOutcomeV0`
  - `notes_commitment: Option<String>` (hash de notas/documentos)
- `EventPayloadV0` tagged enum:
  - `manual_review: ManualReviewPayloadV0`
- En el evento:
  - `payload: Option<EventPayloadV0>` (solo presente en manual_review)

**Nota:** No introducir firma humana individual en MVP.

---

## 2) Validación de proceso (NO rígida en core; implementar como módulo/función)
Crear un validador (puede vivir en `chronos.rs` o en un nuevo módulo `process.rs`, pero sin romper el diseño actual):

### 2.1 Reglas de secuencia del proceso `aml_account_control`
Definir validación de secuencia:
- Obligatorios:
  - `process_opened` primero
  - `transfer_requested` después
  - `aml_scored` después
  - `process_closed` último
- Condicionales (ramas válidas):
  - Ruta A (bloqueo automático):
    - `account_frozen` debe aparecer después de `aml_scored`
  - Ruta B (manual review):
    - `manual_review` puede aparecer después de `aml_scored`
    - tras `manual_review` debe aparecer `account_frozen` o `account_released`
- No permitir eventos después de `process_closed`.

### 2.2 Validación de encadenado Chronos
Validar que:
- cada evento (salvo el primero del proceso) tiene `prev_event_hash` != None
- el hash referenciado corresponde al hash del evento anterior dentro del mismo `process_id`

### 2.3 Validación básica de coherencia de payload
- Si `event_type == manual_review`:
  - `payload` debe estar presente y ser `manual_review`
- Si `event_type != manual_review`:
  - `payload` debe ser None (en MVP)

---

## 3) Canonicalización (solo asegurar determinismo)
En `canonical.rs` (o donde toque):
- Asegurar que las nuevas estructuras (`ProcessRefV0`, `EventTypeV0`, payloads) quedan incluidas en la canonicalización del evento.
- Reglas:
  - orden de campos determinista
  - `serde(rename_all="snake_case")` aplicado donde proceda
  - encoding consistente (JSON canónico o CBOR canónico según el proyecto)

---

## 4) Receipt / Bytes-to-sign (mínimo para ejemplo)
En `receipt.rs`:
- Asegurar que los bytes firmados incluyen:
  - protocol
  - event_id
  - process_ref
  - event_type
  - prev_event_hash
  - issued_at / epoch_id
  - policy_ref
  - actor_identity_ref
  - commitments
  - payload si existe
- Debe ser estable: el mismo evento produce los mismos bytes.

No hace falta integrar criptografía real en el ejemplo si ya existe mock/stub; usar lo que el repo tenga.

---

## 5) Ejemplo completo (nuevo archivo de ejemplo / test)
Crear un ejemplo runnable o test (según estilo del repo) que construya un proceso AML completo y lo valide.

### 5.1 Dataset de ejemplo
Process:
- `process_id`: `"AML-CASE-2025-000341"`
- `process_type`: `"aml_account_control"`
Actor (TAP ref):
- `"tap:AML-Sentinel-v4.5-build-2025-01-15"`
Policy ref:
- `policy_id`: `"AML-2025-Q1"`
- `policy_hash`: `"sha256:..."`
Commitments (placeholders):
- `inputs_commitment`: `"sha256:inputs"`
- `outputs_commitment`: `"sha256:outputs"`
- `artifact_commitment`: `"sha256:artifact"`

### 5.2 Ruta del ejemplo (manual review confirma bloqueo)
Eventos en orden:
1. `process_opened`
2. `transfer_requested`
3. `aml_scored` (score se refleja dentro de outputs comprometidos, no necesariamente en claro)
4. `manual_review` con payload:
   - reviewer_role = "aml_analyst"
   - reviewer_ref = Some("sha256:internal_user_123") (opcional)
   - outcome = confirm_freeze
   - notes_commitment = Some("sha256:review_notes")
5. `account_frozen`
6. `process_closed`

### 5.3 Qué debe demostrar el ejemplo
- Se calculan hashes de evento (según el core actual)
- Se encadenan con `prev_event_hash`
- Pasa el validador de secuencia AML
- Pasa el validador de “no eventos tras cierre”
- `manual_review` requiere payload y lo valida

---

## 6) Criterios de aceptación (Definition of Done)
- [ ] `types.rs` compila con las nuevas estructuras
- [ ] Los eventos soportan `process_ref` + `event_type`
- [ ] Existe `manual_review` payload MVP (institucional)
- [ ] Existe un validador de proceso AML:
  - secuencia y ramas
  - encadenado prev_event_hash
  - cierre explícito
- [ ] Existe ejemplo/test que construye el proceso completo y valida OK
- [ ] Canonicalización y bytes-to-sign incluyen los nuevos campos de forma determinista

---

## 7) Notas de diseño (importantes)
- No introducir firma humana individual en MVP.
- No meter reglas de proceso “hard-coded” dentro del hash del evento (evitar rigidizar acta.v0). La validación puede ser un módulo separado.
- Mantener compatibilidad con el core: los cambios deben ser aditivos, no romper consumers existentes.
- `process_closed` no es “decorativo”: es un evento que cierra la historia y facilita anti-omisión.

---

## 8) Deliverables esperados
- Cambios en: `types.rs`, `canonical.rs`, `receipt.rs`, `chronos.rs` (o nuevo `process.rs`)
- Nuevo ejemplo/test: `examples/aml_process.rs` o `tests/aml_process.rs` (según convención del repo)

FIN