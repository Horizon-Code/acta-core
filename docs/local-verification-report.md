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

## Conditions Recorded By The Offline Attestation Adapter

`adapters/attestation-single-signer/rust` composes its checks over the Core report:

- `TR-KEY-SELF-ASSERTED` in `VersionStructural` whenever the v0 verifiable bundle carries its
  verification keys inline. The cryptographic signature is independently checkable, but the
  claimed real-world identity remains self-asserted.
- `TR-SIGNER-SELF` in `DossierDetected` when a cryptographically verified `attestor_id` equals
  the event's `actor_ref.actor_id`.

It also adds `cryptographic_signatures_valid` as an executed check. Failure changes the
combined report to `Fail`; the Core-only report remains unchanged and contains no key logic.

## Conditions Recorded By The Offline CLI

`tools/acta-verifier/rust` adds the conditions it can determine without external access:

- `TR-TIME-DECLARED` as a v0 structural condition;
- `TR-NO-ANCHOR` when the dossier has no anchor reference; or
- `TR-ANCHOR-UNVERIFIED` when an anchor is declared but has not been queried on its substrate.

The CLI emits deterministic structured JSON and a plain-text rendering. It performs no network
request; external ledger verification remains a separate adapter concern.

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
- external anchor and identity-registry verification layers.
- expanded machine-readable error taxonomy when needed.
