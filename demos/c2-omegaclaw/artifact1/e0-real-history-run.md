# Artefact 1 — retained E0 history execution record

- Execution date: 2026-09-01
- Source: local Docker volume `omegaclaw-memory`, mounted `readonly`
- Source history SHA-256:
  `d320182a34ad8e737f8df404d1359851589675a7c8b0e4ff869770988f19983a`
- Source size: 21,642 bytes
- Parsed history records: 100
- Profile lifecycle events: 102 (`run_started`, `instruction_received`, 100 contexts)
- Epoch root:
  `bd60c5e6fbdd425047387e9d87f1a3e2b3307ed718d229b19141afd843238fba`
- Generated witness-file SHA-256:
  `3aa94889b07052ea6191ebd953cd35629497869a73e9d135dcd4a9e3a22217f8`

The original verification returned `ACTA_EXTERNAL_CHECK=PASS`. The test then modified only a
temporary host copy by deleting the unique record containing `[E0-c1-dup]` (record index 2).
The local OmegaClaw-form check still passed with 99 records. ACTA returned exit code 2 with:

```text
finding=history_record_sequence_changed
finding=committed_history_record_missing:sha256:8fc308b5c7dec23984139ed92313255fe76986e40d0f7dd1d558eff4a77e12c8:count=1
finding=history_record_count_changed:committed=100:current=99
```

The source volume remained read-only and unchanged. This record makes the local execution
auditable; it does **not** turn the generated sibling-directory witness into independent
custody. S6 still requires a real independent holder or an external anchor.
