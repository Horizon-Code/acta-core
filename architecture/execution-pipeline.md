# Execution Pipeline (Architecture)

This file captures architectural content extracted from former mixed notes.

## Goal

Define a standard ACTA execution pipeline so implementations preserve consistent protocol behavior.

## Pipeline

1. Input
2. Observed Event
3. Assessment
4. Act
5. Validation
6. Evidence
7. Persistence
8. Evaluation

## Phase boundaries

- Input: case data, external context, relevant policy references.
- Observed Event: open verifiable process history (`chronos_event_id`, `prev_event`, epoch context).
- Assessment: decision proposal (LLM, deterministic rules, or ML), never accepted as valid by itself.
- Act: normalize decision into an ACTA act with actor, context, policy reference, and confidence.
- Validation: enforce policy applicability, actor authorization, and confidence thresholds.
- Evidence: produce commitments for inputs, outputs, and artifacts.
- Persistence: register Chronos events and evidence commitments as immutable history.
- Evaluation: record result quality (`correct`, `incorrect`, `disputed`) as precursor to dispute flow.

## Architectural invariants

- Store acts, not free-form decision artifacts.
- Validation gates execution; invalid acts are rejected, escalated, or routed to fallback.
- Evaluation must connect to dispute handling rather than remain isolated telemetry.
