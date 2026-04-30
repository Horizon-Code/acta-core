# Core Boundaries

Canonical constitutional reference: `constitution/ACTA_Foundations_v1.2_consolidado.pdf`.

ACTA Core remains small, neutral, auditable, and substrate-independent.

## Core Can Answer

- Is this event structurally valid?
- Is this event canonically encoded?
- Does this hash match this event?
- Does this receipt bind to this event and Chronos ref?
- Does this bundle internally bind event, receipt, Merkle proof, epoch root, and optional anchor?
- Does this local sequence link correctly?
- Is this commitment syntactically valid?

## Core Cannot Answer

- Was the decision fair?
- Was the law satisfied?
- Was the actor institutionally authorized?
- Should the account be frozen?
- Should someone be sanctioned?
- Did Cardano really include this tx?
- Did Midnight really verify this proof?
- Was the AML policy materially correct?
- Is the external institution right?

## Responsibility Split

- Core: primitives and structural verification.
- Protocol: interoperable formats and versioning.
- Profiles: domain semantics and lifecycle rules.
- Adapters: external ledgers, identity, storage, registries, and proof systems.
- Institutions/auditors: adjudication, enforcement, sanctions, and remedies.

## Chronos Scope

Core Chronos provides local continuity primitives (`epoch_id`, `prev_event_hash`, and sequence consistency for supplied events/hashes). It does not by itself prove global anti-omission.

Stronger anti-omission requires additional layers such as event hash chaining over publication scope, epoch closure discipline, receipt issuance practice, Merkle root anchoring, and external/indexer/auditor expectations, plus profile lifecycle rules.
