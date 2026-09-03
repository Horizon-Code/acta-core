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
    (E-4), shape ratified in ADR-004.

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

### Confirmed dynamically on 2026-08-31

Source: `research/E0-resultados.md`, captures 3 and 4.

- Risk: `last_chars(maxFeedback)` truncates the **head** of a large result and can cut in the
  middle of a token. The measured case transformed 108,889 characters into 50,000 and began
  with the orphaned suffix `68` of `11668`, without the opening parenthesis.
  - Impact: The LLM receives a malformed s-expression. Committing that visible string would
    make recomputation fail for a harness artifact rather than a semantic difference.
  - Mitigation: Commit raw `(eval $s)` output before `normalize_string`; record the LLM context
    separately and classify links through the declared harness transformation.

- Risk: Harness configuration changes the evidence boundary. `maxFeedback`, `maxHistory`,
  `string-safe` and `normalize_string` determine what conclusion text reaches the mediator and
  therefore whether a byte predicate classifies a link as faithful.
  - Impact: Two verifiers with the same ruleset but different harness configuration can report
    different faithful/free ratios.
  - Mitigation: Pin the harness source/version and transformation-affecting configuration in
    the Profile lockfile; use `bytes(premise) == bytes(T(C))` only for a declared `T`.

- Risk: A high faithful-link ratio does not remove mediator discretion. Capture 3 observed a
  5/5 ceiling in five isomorphic two-hop tasks under an explicit byte-copy instruction. It is
  not a generalizable mediator rate; auxiliary attempts omitted the prior conclusion
  temporarily or recomputed it several times.
  - Impact: Readers may mistake literal continuity for completeness or constrained selection.
  - Mitigation: Treat `TR-CHAIN-MEDIATED` as the default landscape whenever an LLM joins hops;
    keep the faithful/free figure and the selection disclaimer inseparable.

## S8 / demonstration evidence custody

Recorded on 2026-09-03 during the S8 anchor deployment preflight.

- Risk: Demonstration evidence held in ephemeral storage. The bundles and `epoch-witness.json`
  of the real C2 run (2026-09-01, epoch root `bd60c5e6…38fba`) lived only inside the
  `omegaclaw-memory` Docker volume and are not recoverable from the current environment. This
  is the same failure ACTA identifies in OmegaClaw's `memory/history.metta` — evidence without
  retention — reproduced at home.
  - Impact: The anchor reference cannot be attached to the Artefact 1 bundles, so ADR-012 §5
    custody closure stays open even after a real on-chain attestation of the correct root. A
    published anchor proves the mechanism, not the custody of the demo it was meant to close.
  - Mitigation: Every run intended as demonstration material persists bundles and witness
    outside the volume — in the tree or at a declared path — in the same act that generates
    them. Without that step the run does not count as demonstration material.

## SCITT / substrate compatibility

Measured on 2026-09-03 during the SCITT mini-E0 (`research/scitt-mini-e0-2026-09-03.md`).

- Risk: ACTA's signature algorithm is not universally accepted by Transparency Services. ACTA
  signs **Ed25519**. `scitt-ccf-ledger` accepts `EDDSA` in its default algorithm list
  (`["ES256","ES384","ES512","PS256","PS384","PS512","EDDSA"]`), but the DataTrails quickstart
  signs with **prime256v1/ES256**, and in both cases the accepted list is **configurable by the
  service administrator**. RFC 9943 puts issuer authentication explicitly out of scope, so the
  standard guarantees nothing here: acceptance is decided per service.
  - Impact: a Transparency Service can be chosen on every other merit and then reject ACTA's
    signatures outright. Discovering this after selecting a service would mean either changing
    ACTA's signing algorithm — which touches every receipt and every existing bundle — or
    abandoning the service.
  - Mitigation: the accepted-algorithm list of a candidate service is a **mandatory input** to
    the future SCITT adapter ADR, checked against the live service rather than assumed from its
    documentation. The choice of service is conditioned by the algorithm, not the reverse. No
    service is presented as supported until that check is on record.
