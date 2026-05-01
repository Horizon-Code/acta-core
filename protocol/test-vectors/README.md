# ACTA Protocol v0 Test Vectors

Purpose:
- Provide fixed interoperability vectors for ACTA Protocol v0 canonical hashing and verification.

Rules:
- Vectors are deterministic and non-sensitive.
- Hash-critical values use strict lexical forms.
- Canonicalization/hash output changes are protocol-breaking unless versioned.
- Expected hashes are checked-in fixed values, not generated ad hoc by tests.

Current vectors:
- `event-v0.json` + `event-v0.hash`
- `receipt-v0.json` + `receipt-body-v0.hash`
- `bundle-v0.json`
- `merkle-v0-nontrivial.json` + `merkle-v0-nontrivial.root`
