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
