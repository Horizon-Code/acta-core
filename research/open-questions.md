# Open Questions

1. Should parts of `ACTA_Foundations_v1.2_consolidado.pdf` be distilled into architecture-level canonical documents?
2. What minimum registry/identity assumptions are required for independent receipt signature verification?
3. What profile conformance checks are mandatory before marking a profile stable?

## From E0 (`roadmap/E0-protocolo-y-enmiendas.md`)

4. Where does JeTTa live? The directive cites it (v0.9.0, MIT, Kotlin) without an organization
   and a quick search does not resolve it. It is future differential-verification material, not
   E0, so it blocks nothing — look under `trueagi-io` or `patham9` rather than trusting a
   reconstructed link.

## From the SCITT mini-E0 (2026-09-03)

5. **Where is the research brain that holds `H-011`?** The mini-E0 brief says the measured
   numbers should return there and turn `H-011` from a reading into a fact, noting it was
   recorded as a dependency in session 03. `H-011` does not appear anywhere in this repository,
   so that register lives outside it. The measured result is written in
   `research/scitt-mini-e0-2026-09-03.md`; it still needs routing to whatever holds `H-011`.
6. **Which Transparency Service does ACTA target?** The mini-E0 measured that issuer acceptance
   is per-operator, not per-standard, and that accepted signature algorithms differ between
   implementations — `scitt-ccf-ledger` accepts `EDDSA` by default while the DataTrails
   quickstart signs with ES256. ACTA signs Ed25519, so the choice of service determines whether
   the current signing algorithm is usable at all. Not a blocker until a service is chosen.
7. **Does a SCITT receipt (RFC 9942) survive the same round-trip discipline as the envelope?**
   The mini-E0 measured only the Signed Statement that is sent, never a receipt returned by a
   real service. The retention rule proposed in ADR-014 assumes the receipt is preserved
   byte-for-byte like any other returned artifact; that assumption is unmeasured.
