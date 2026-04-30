# Failure Taxonomy

- Status: draft architecture note
- Authority: subordinate to ACTA Foundations v1.2
- Canonical reference: `constitution/ACTA_Foundations_v1.2_consolidado.pdf`
- Purpose: translate the Foundations v1.2 localized failure clause into technical architecture responsibilities

## Localized Failure Definition

Localized failure means identifying which specific jurisdiction failed, instead of collapsing all issues into a single generic invalid state.

A failed component must not be silently replaced by metadata from another component.

Examples:
- PoE/existence cannot substitute Chronos/continuity.
- DID/identity cannot substitute TAP/technical agency.
- Evidence commitments cannot substitute Policy/normative reference.
- Resolution cannot rewrite history.
- Anchor metadata cannot prove internal bundle consistency.
- A valid hash cannot prove material truth.

## Failure Jurisdictions

### Existence failure
- What failed: integrity/existence binding for a claimed object.
- Example: event hash recomputation does not match claimed `event_hash`.
- Who can detect it: Core.
- Who cannot detect it: Profiles and adapters cannot override Core integrity mismatch.
- Consequence: affected object is structurally invalid for that jurisdiction.
- Should Core detect it: yes.

### Continuity failure
- What failed: supplied sequence linkage (`prev_event_hash`) and/or continuity binding across related objects.
- Example: `prev_event_hash` mismatch, or receipt Chronos ref mismatch against bundle Chronos ref.
- Who can detect it: Core (local linkage), profiles (domain lifecycle gaps).
- Who cannot detect it: adapters cannot repair local sequence linkage.
- Consequence: continuity claims degrade or fail for the supplied sequence.
- Should Core detect it: yes (for supplied local chain and bundle-level consistency).

### Technical agency failure
- What failed: actor technical agency requirements.
- Example: empty `actor_ref.actor_id` (shape failure) vs revoked DID credential (external failure).
- Who can detect it: Core detects only shape-level emptiness; adapters can detect external identity/credential status; profiles can define role constraints.
- Who cannot detect it: Core cannot resolve external DID/registry state.
- Consequence: shape-level failure blocks event validity; external agency disputes remain external.
- Should Core detect it: yes for shape; no for external authorization state.

### Normative reference failure
- What failed: normative/policy reference presence or external validity.
- Example: missing `policy_hash` vs policy later judged invalid under law.
- Who can detect it: Core (presence/shape), profiles (domain-required policy scope), adapters (artifact retrieval), institutions (legal validity).
- Who cannot detect it: Core cannot decide legal applicability.
- Consequence: missing structural reference blocks shape; legal validity remains adjudication scope.
- Should Core detect it: yes for structural presence/shape only.

### Evidence failure
- What failed: evidence commitment syntax/integrity linkage or material evidence quality.
- Example: invalid commitment syntax vs fraudulent content correctly committed.
- Who can detect it: Core (syntax/integrity mismatch); profiles (required evidence categories); adapters (retrieval/disclosure path); institutions (material truth review).
- Who cannot detect it: Core cannot decide material truth.
- Consequence: syntax/integrity failure blocks structural validity; material disputes remain external.
- Should Core detect it: yes for syntax/integrity checks in scope.

### Bundle/anchor consistency failure
- What failed: internal binding among event, receipt, Merkle proof, epoch root, and optional anchor root.
- Example: `AnchorRootMismatch`, `InvalidMerkleProof`, or receipt/event hash mismatch inside bundle.
- Who can detect it: Core.
- Who cannot detect it: profiles/adapters cannot reinterpret internal inconsistency as valid.
- Consequence: bundle cannot be accepted as internally consistent evidence object.
- Should Core detect it: yes.

### Profile/domain semantic failure
- What failed: domain lifecycle rules, allowed transitions, required domain evidence.
- Example: syntactically valid event violates a profile state transition rule.
- Who can detect it: profile logic and profile-aware consumers.
- Who cannot detect it: Core (by design).
- Consequence: domain decision may be invalid even if Core structure is valid.
- Should Core detect it: no.

### Adjudication/consumption failure
- What failed: external adjudication, enforcement, remedy, or institutional consumption quality.
- Example: authority never acts, delays action, or reaches a disputed external conclusion.
- Who can detect it: institutions/auditors and governance processes outside Core.
- Who cannot detect it: Core/profiles/adapters cannot force final adjudication quality.
- Consequence: external trust/compliance outcome may fail despite technically valid evidence.
- Should Core detect it: no.

## Responsibility Matrix

| Failure jurisdiction | Example | Core responsibility | Profile responsibility | Adapter responsibility | Institution/auditor responsibility | Should another layer fake it? |
| --- | --- | --- | --- | --- | --- | --- |
| Existence failure | `EventHashMismatch` | Detect hash mismatch and reject claim | Do not redefine hash validity | May carry/anchor data, not reinterpret mismatch | Consume and report integrity failure | No |
| Continuity failure | `prev_event_hash` mismatch | Detect local chain/link mismatch | Detect lifecycle continuity gaps | Do not patch continuity with metadata | Adjudicate consequence of continuity break | No |
| Technical agency failure | empty `actor_ref` vs revoked DID | Detect shape emptiness only | Define required technical roles | Resolve DID/registry credential state | Decide authority consequence | No |
| Normative reference failure | missing `policy_hash` vs invalid law applicability | Detect structural presence/shape only | Define policy scope requirements | Fetch policy artifacts/registries | Decide legal validity/applicability | No |
| Evidence failure | invalid commitment syntax vs fraudulent data | Validate commitment syntax/integrity binding | Define domain evidence requirements | Retrieve/disclose external evidence artifacts | Evaluate material truth and sufficiency | No |
| Bundle/anchor consistency failure | `AnchorRootMismatch`, `InvalidMerkleProof`, tx absent | Verify internal bundle consistency and anchor root equality | Do not override internal mismatch | Verify external inclusion (ledger/proof systems) | Consume combined internal/external result | No |
| Profile/domain semantic failure | lifecycle transition invalid for domain | Out of scope | Enforce domain semantics and lifecycle | Provide profile-required external data paths | Evaluate profile-level compliance result | No |
| Adjudication/consumption failure | authority does not act or acts wrongly | Out of scope | Out of scope | Out of scope | Own adjudication/enforcement/remedy | No |

## Severity Vocabulary

- blocking: prevents structurally valid ACTA object in that jurisdiction.
- degrading: object remains usable but weaker or incomplete.
- disputable: object remains structurally valid but requires external review.
- informational: recorded as context but not validity-affecting.

Severity is contextual and may be refined by profiles or institutional consumers.

This document does not implement severity logic in Core.

## Architectural Rule: No Assurance Upgrade by Naming

No layer may upgrade a lower-assurance fact into a higher-assurance fact by naming or metadata.

Examples:
- A timestamp string is not PoE.
- A DID string is not TAP authorization.
- A `policy_id` string is not legal applicability.
- A commitment string is not material truth.
- A Cardano `tx_id` string is not verified anchoring unless checked by an adapter.
- A dispute status string is not adjudication unless issued by the relevant external authority/process.

## Mapping Current Core Checks to Failure Taxonomy

Current Core checks and likely failure jurisdictions:
- `EventHashMismatch` => existence/integrity failure.
- `ReceiptEventHashMismatch` => bundle consistency failure.
- `ReceiptChronosRefMismatch` => continuity/bundle consistency failure.
- `ReceiptBodyHashMismatch` => bundle/receipt integrity failure.
- `InvalidMerkleProof` => epoch inclusion failure.
- `AnchorRootMismatch` => bundle/anchor consistency failure.
- `EventValidationError::InvalidCommitment` => evidence shape failure.
- `EventValidationError::MissingField("policy_snapshot.policy_hash")` => normative reference shape failure.
- `ChronosError::PrevHashMismatch` / `MissingPrevHash` => local continuity failure.

Notes:
- This mapping is descriptive for current v0 code and may be refined in later versions.
- Any future taxonomy refinements must remain subordinate to Foundations v1.2.
