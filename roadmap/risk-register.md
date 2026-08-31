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

### Confirmed statically on 2026-08-31 (pre-flight, container not yet run)

Source: `research/E0-resultados.md`. Recorded as risks because they change the shape of E-4 and
of the capture-4 decision, and were found by reading code rather than by running it.

- Risk: The import closure of the target system is **not only MeTTa**. 13 of its 34 files are
  Python, imported through the same `(library ...)` form.
  - Impact: A lockfile covering only `.metta` would leave 38% of the closure uncommitted while
    looking complete.
  - Mitigation: E-4's manifest must cover the whole closure. Enumeration script and hashes in
    `research/E0-resultados.md` §2.1.

- Risk: **Nothing in the substrate is pinned to an immutable revision.** The base image, PeTTa
  and FAISS are pinned to tags, `petta_lib_chromadb` to `master`, and PeTTa's own `build.sh`
  clones `mork_ffi` and `faiss_ffi` with no ref at all. `git-import!` runs
  `git clone --depth 1` with no branch and no commit, and skips entirely if the directory
  already exists.
  - Impact: Two builds of the same declared system can differ with no record anywhere.
  - Mitigation: `TR-IMPORT-UNPINNED`; the lockfile records what the system does not.

- Risk: The string the LLM sees is **transformed and truncated** before it gets there.
  `string-safe` substitutes newlines, doubled double-quotes and apostrophes; `normalize_string`
  drops invalid UTF-8 with `errors="ignore"`; `last_chars` truncates to `maxFeedback` (50000)
  **keeping the tail**, so the cut falls on the head of the string.
  - Impact: Committing the LLM-visible string commits a mutilated conclusion. Worse for E-2: a
    conclusion containing a newline can never match a fed-back premise byte for byte, for
    reasons that have nothing to do with the mediator.
  - Mitigation: Capture point upstream of `normalize_string`; discount the deterministic loop
    transformation before measuring the faithful ratio in capture 3.

- Risk: `lib_omegaclaw.metta` imports `./src/context`, for which no file exists in the tree at
  commit `642c536`; and `static-import!` generates `.pl`/`.qlf` artifacts during execution.
  - Impact: A statically generated manifest has an entry with no file, and a post-session
    re-hash sees new files that were never in it.
  - Mitigation: Capture 2 must distinguish *source file changed* from *derived artifact
    created*.
