# Risk Register

- Risk: Imported research notes contain historical paths from pre-restructure layout.
  - Impact: Reader confusion.
  - Mitigation: Keep notes unchanged as historical import, add orientation note in research index.

- Risk: Foundations document authority (direction vs architecture extraction) not yet fully settled.
  - Impact: Potential ambiguity in normative interpretation.
  - Mitigation: Distill canonical architecture statements into `architecture/` and freeze via ADRs.

## E0 / OmegaClaw substrate

Source: `E0-protocolo-y-enmiendas.md`. These are unresolved until the E0 captures run.

- Risk: The import closure of the target system loads external code at an unpinned revision
  (`git-import!` without a commit).
  - Impact: Two executions of the same declared ruleset can differ with no record; recomputation
    fails for reasons unrelated to semantics.
  - Mitigation: `TR-IMPORT-UNPINNED` in the report (E-1); lockfile as a (path, hash) manifest
    (E-4), shape pending capture 2.

- Risk: Files loaded from the PeTTa installation may have integrity without provenance — the
  hash is on record, the commit or pip version it came from may not be.
  - Impact: The manifest proves the file did not change, but not which file it was.
  - Mitigation: Capture 2 determines whether provenance is recoverable from the running process;
    if it is not, declare it as a stated limit in the report.

- Risk: The inference result string may be truncated at any of the three stages between eval and
  the LLM context.
  - Impact: Committing the string the LLM sees would commit a mutilated conclusion. This is the
    kind of failure that does not appear in small tests and breaks a live demo.
  - Mitigation: Capture 4; if truncation exists, move the commitment capture point upstream of
    it and record the truncated context string as a separate artifact.

- Risk: Chain links pass through a non-deterministic mediator (the LLM), so multi-step reasoning
  is not a single recomputable derivation.
  - Impact: Per-link audit detects distortion, not selection: an agent can carry everything
    forward verbatim and still have discarded what contradicted it.
  - Mitigation: `TR-CHAIN-MEDIATED` annotated per link, with the figure and the disclaimer
    inseparable in the same report line (E-2).

- Risk: Two of the differential-verification references are not safe to depend on —
  `trueagi-io/MORK` has no license, `Adam-Vandervorst/PathMap` is a personal account.
  - Impact: Accidental dependency on material that cannot be relied upon legally or for
    continuity.
  - Mitigation: Read methodology only; never vendor or depend. Same rule already stated for the
    `oprow` vendored inside `singnet/watermarks_PoC` (§5.8).
