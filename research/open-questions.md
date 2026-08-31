# Open Questions

1. Should parts of `ACTA_Foundations_v1.2_consolidado.pdf` be distilled into architecture-level canonical documents?
2. What minimum registry/identity assumptions are required for independent receipt signature verification?
3. What profile conformance checks are mandatory before marking a profile stable?

## From E0 (`roadmap/E0-protocolo-y-enmiendas.md`)

4. Is the import closure of `lib_omegaclaw.metta` statically enumerable, or does loading happen
   lazily at first use? (E0 capture 2 — decides whether the lockfile is generated ahead of
   execution or captured at runtime.)
5. Is the provenance of files loaded from the PeTTa installation (`lib_patrick`, `lib_llm`,
   `lib_vector`, `lib_combinatorics`, `lib_he`) recoverable from the running process, or does the
   manifest only give integrity without provenance? (E0 capture 2.)
6. Where does JeTTa live? The directive cites it (v0.9.0, MIT, Kotlin) without an organization
   and a quick search does not resolve it. It is future differential-verification material, not
   E0, so it blocks nothing — look under `trueagi-io` or `patham9` rather than trusting a
   reconstructed link.
7. Where is `acta-estado-consolidado.md`? The construction directive declares it complements
   that document and depends on it for §1.2 (vocabulary scale), §3.1 (canonicalization
   decisions), §6.1 and §6.7 (forensic levels, anchor backend), §8.1 (unresolved questions) and
   §11 (communication risk). Two cross-references in `roadmap/E0-protocolo-y-enmiendas.md` (E-3
   and E-7) land there rather than in the directive.
