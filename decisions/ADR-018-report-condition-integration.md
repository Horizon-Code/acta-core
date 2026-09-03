# ADR-018: Integrating profile conditions into the machine report

- Status: **Proposed**
- Date: 2026-09-03
- Decision authority requested: operator under ADR-001
- Depends on: Proposed ADR-017 (v1.1 implementation submission)
- Requires: a refresh of the **ADR-013** hash table and, as a consequence, of the **ADR-012**
  table
- Implementation: **none yet**. This is the refresh proposal, deliberately written before any
  pinned file is touched.

## Why this is a proposal and not a change

ADR-017 implemented the residual-trust conditions of both v1.1 profiles, but nothing surfaces
them: a machine dossier still says nothing about denials, coverage, disputes or mandate gaps.
Surfacing them means editing `tools/acta-verifier`, which is fixed by the ADR-013 table.

The rule established when the ADR-012 table was refreshed applies here: editing a file fixed by
a hash table breaks that part's ratification, and **the refresh is never made by the session
that caused the change**. So this session states what would move and why, and stops.

## The consequence that is easy to miss

`Cargo.lock` appears in **both** the ADR-012 and the ADR-013 tables, with the same hash
`7c1fe659…31f1`. `acta-verifier` is a workspace member, so giving it a dependency on the two
v1.1 crates rewrites the root lock file.

**Refreshing ADR-013 alone is therefore not sufficient.** ADR-012's table would be left with a
stale `Cargo.lock` entry, and by its own rule the EAS adapter's ratification would silently
break — for a change that has nothing to do with EAS. Both tables must be refreshed in the same
operator act, and ADR-012's affected part re-reviewed even though no adapter behaviour changes.

Verified while writing this: the two entries are the same hash today, so they move together or
one of them is wrong.

## What would change

The report envelope needs **no new fields**. `MachineReportV1` already carries
`structural_conditions`, `detected_conditions` and `evidence_profile` with its namespace and
version, which is exactly the key the conditions are selected by. That matters: the integration
does not reopen the ratified ADR-013 envelope shape, only what fills it.

| File | In table | What changes |
|---|---|---|
| `tools/acta-verifier/rust/src/lib.rs` | ADR-013 | Append the profile-version conditions to the two existing vectors, keyed on `evidence_profile` |
| `tools/acta-verifier/rust/Cargo.toml` | ADR-013 | Path dependencies on the two v1.1 crates |
| `tools/acta-verifier/rust/tests/cli_v0.rs` | ADR-013 | Tests that a v1.0 dossier still declares its `*-UNSUPPORTED` codes and a v1.1 one does not |
| `docs/machine-verification-report-v1.md` | ADR-013 | Document the codes a consumer may now see |
| `Cargo.lock` | **ADR-012 and ADR-013** | New path dependencies |

Five files, one of them in two tables.

## Rules the integration must respect

- **Retirement is per producing version and never retroactive.** A v1.0 dossier keeps declaring
  `TR-DENIAL-UNSUPPORTED`, `TR-REFUSAL-UNSUPPORTED` and `TR-DISPUTE-RESOLUTION-UNSUPPORTED`
  forever. The conditions are selected from `evidence_profile.version` carried in the bundle,
  never from the verifier's own version — otherwise upgrading the verifier would silently
  rewrite what an old dossier says about itself.
- **An unrecognised version assumes the weaker claim.** Already implemented in ADR-017 and
  preserved here.
- **No code without its check.** `TR-COVERAGE-GAP` stays unimplemented and unemitted until
  something can compare declared intervals against activity evidenced outside ACTA.
- **Conditions do not change technical status.** They are honest information for the reader, not
  verdicts, so they must not move `status` and must not by themselves produce exit code `2`.
  Whether an unmet condition may be named in `acta.machine-trust-requirements.v1`, and so reach
  exit `3`, is a separate question this ADR does not answer.

## What this does not decide

It does not implement the integration, does not change the ADR-013 envelope shape, does not
introduce new condition codes, and does not extend the requirements vocabulary.

## Requested from the operator

1. Authorisation to make the change described above.
2. A refresh of the **ADR-013** table and of the `Cargo.lock` entry in the **ADR-012** table,
   made after the change and by a later session, with the previous hashes recorded as the
   ADR-012 refresh history already does.
3. Re-review of ADR-012's affected part, which is bookkeeping only: no adapter file other than
   the shared lock file moves.
