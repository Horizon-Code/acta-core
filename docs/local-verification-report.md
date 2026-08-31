# Local Verification Report v0

## Status

- Local Verification Report v0.
- Technical/local output.
- Not UI.
- Not product final.
- Not institutional judgment.

## What It Verifies

- event hash validation.
- receipt body validation.
- receipt shape validation.
- receipt/bundle event-hash binding.
- receipt-body-hash/bundle binding.
- chronos_ref/bundle binding.
- epoch root reference consistency.
- Merkle proof validity.
- bundle internal consistency via local Core checks.
- anchor root internal consistency when `anchor` is present.
- check status semantics are explicit:
  - `Pass`: check executed and passed.
  - `Fail`: check executed and failed.
  - `NotChecked`: check was not executed because a required prior check failed (or not applicable, e.g. missing optional anchor).

## What It Does Not Verify

- material truth.
- legality.
- justice.
- sanctions.
- substantive compliance.
- institutional authority.
- institutional adoption.
- correctness of external policy.
- truth of committed evidence.
- Cardano/Midnight/DID/ZK/storage external truth.

## Trust Conditions: Two Registers

The report carries trust conditions in two distinct registers, separated at type level by
`VerificationConditionRegisterV0`:

- `VersionStructural` — what this protocol version never guarantees, independently of the
  dossier. Declared once, at the top. This register is a scale, not a disclaimer list: closing
  a phase deletes a line from it.
- `DossierDetected` — what was detected while verifying this specific dossier. Varies between
  dossiers, which is why it carries signal.

The split exists so the reader never learns to skip lines.

## Condition Codes Are Data, Not Variants

- `VerificationWarningV0::code` is an open string. Adding a code must never require touching
  `acta-core`.
- The Core defines the mechanism that transports codes, and at most codes for its own
  substrate. Codes carrying domain semantics belong to the Profile that defines them.
- The Core never matches on a code and never validates it against a known set.
- Protected by `report_transports_unknown_condition_codes_intact`, which does not compile
  against a closed enum.

## Conditions Recorded By Core v0

None. The Core records no condition it cannot itself evaluate: a code whose condition the
verifier cannot evaluate is not a report line, it is a promise in the source. Each code lands
with the functionality that makes its condition detectable. Calendar in
`roadmap/next-milestones.md`.

## How It Relates To Core

- Built on existing Core local verification primitives.
- Does not change Core verification semantics.
- Does not add domain semantics.
- Does not replace `verify_bundle_v0`.
- Adds a structured explanation layer for local technical checks.

## How Profiles Use It Later

- AML can consume this report to explain Core verification of AML-generated bundles.
- AI Agents workflows can consume the same report for agent-generated bundles later.
- Domain interpretation remains outside Core.
- The report model remains domain-neutral.

## Accepted Deferred Debt

- richer report formatting.
- human/regulator/auditor report variants.
- localization.
- UI presentation.
- external adapter verification layers.
- expanded machine-readable error taxonomy when needed.
