# ACTA Profile Architecture v1.0

Status: internal architectural draft for repository adoption

Authority: subordinate to `ACTA Foundations v1.1`

## 1. Preamble

An ACTA profile is a domain-specific semantic layer that constrains how ACTA events, references, commitments, and lifecycle transitions are used for a class of significant computational acts. A profile exists to make a domain legible without expanding the Core beyond its constitutional limits.

ACTA requires profile discipline because semantic freedom destroys interoperability faster than schema divergence. If profiles are allowed to define names, event meanings, closure semantics, policy linkage, and lifecycle behavior without common constraints, ACTA degrades into unrelated JSON dialects with shared branding but no shared accountability model.

This document defines the architectural rules that every ACTA profile MUST satisfy to remain compatible with the ACTA ecosystem. These rules translate `ACTA Foundations v1.1` into repository governance for profile design, registration, review, evolution, and retirement.

## 2. Central Thesis

An ACTA profile MUST extend domain semantics without weakening attribution, reconstruction, proportionality, minimization, or interoperability.

## 3. Definitions

`Profile`
: A versioned specification that defines domain-scoped event kinds, semantic rules, lifecycle constraints, and validation requirements for a class of significant computational acts using ACTA primitives.

`Namespace`
: A stable identifier space under which a profile defines its event kinds, semantic terms, and profile-specific references.

`EventKind`
: A named event category with stable meaning, admissible fields, lifecycle role, and explicit compatibility expectations.

`Lifecycle`
: The finite set of allowed state transitions and status meanings for entities or processes represented by a profile.

`Semantic invariant`
: A rule that MUST remain true across all valid profile instances, regardless of implementation, transport, or storage substrate.

`Material effect`
: A domain outcome that can trigger, justify, contest, or evidence institutional, contractual, financial, regulatory, or operational consequence outside ACTA.

`Dispute-ready event`
: An event whose actor references, policy references, sequence position, evidence linkage, and commitments are sufficient to support external evaluation or challenge without silent interpretation by the producer.

## 4. Constitutional Principles

### 4.1 Close means close

A profile MUST define closure semantics so that an event labeled as closed, completed, resolved, revoked, rejected, expired, or equivalent has one stable meaning. Closed states MUST NOT be reinterpreted as provisional inactivity or soft pause.

### 4.2 Reopen requires explicit event

A closed lifecycle state MUST NOT be reversed by mutation, replacement, or inferred system behavior. Reopening MUST occur only through an explicit event kind defined by the profile.

### 4.3 Commitments bind to real data

If a profile uses commitments, the profile MUST specify what data is committed, when the commitment is formed, and how the committed object can later be disclosed or verified. Placeholder commitments with no disclosure or verification path MUST be rejected.

### 4.4 Policy references must be resolvable

A profile MUST define a policy reference model that allows reviewers and consumers to determine which policy object governed an event at creation time. A policy reference MAY resolve by immutable identifier, versioned URI, content hash, or registry entry, but it MUST NOT depend on an unversioned label alone.

### 4.5 Event chains must be coherent

A profile MUST define sequencing expectations for materially related events. When order matters, event kinds MUST declare predecessor, successor, or exclusivity rules. A profile MUST NOT rely on human interpretation alone to explain contradictory sequences.

### 4.6 No silent mutation

A profile MUST preserve event immutability at the semantic level. Corrections, supersessions, cancellations, reopenings, and overrides MUST be expressed through explicit events or explicit version transitions, never through hidden field replacement.

### 4.7 Overrides must be explicit

When an event or policy effect supersedes another, the profile MUST define the override mechanism and the target of the override. Implicit precedence rules based on timestamp alone SHOULD be avoided unless the profile states them clearly and proves they are safe.

### 4.8 Material effects must be tagged

Every event kind with potential external consequence MUST declare whether it has material effect. Profiles MUST distinguish informational events from events that can alter institutional posture, eligibility, duty, status, or dispute position.

### 4.9 Names must be stable and unambiguous

Profile names, namespaces, event kinds, and lifecycle labels MUST be domain-clear and semantically narrow enough to survive reuse across implementations. A profile MUST NOT use overloaded names such as `update`, `change`, `review`, or `flag` without profile-level normative definition.

### 4.10 Domain semantics cannot contradict ACTA Core

A profile MUST NOT redefine ACTA Core primitives, imply that ACTA establishes truth material, or embed institutional judgment as if it were a Core fact. Domain semantics may enrich interpretation, but MUST remain subordinate to Core integrity, references, sequence, and receipt rules.

### 4.11 Versioning is mandatory

Every profile MUST expose a stable identifier and semantic version. Breaking semantic changes MUST increment the major version. Registries MUST reject profiles whose meaning changes without version change.

### 4.12 Backward compatibility rules are mandatory

Each profile version MUST declare compatibility expectations with prior versions, including whether events remain readable, transformable, deprecated, or invalid. Silence on backward compatibility MUST be treated as incomplete specification.

### 4.13 Proportionality is mandatory

A profile MUST model only acts whose external relevance justifies structured accountability. Event kinds for operational noise, routine telemetry, or trivial system chatter MUST be excluded unless the profile proves material effect or dispute utility.

### 4.14 Minimization is mandatory

A profile MUST capture only the structure necessary for attribution, reconstruction, and dispute-readiness. Profiles MUST prefer references, commitments, and selective disclosure over full payload inclusion when equivalent accountability can be preserved.

### 4.15 Profiles must not become surveillance systems

A profile MUST NOT use ACTA as general monitoring infrastructure. Continuous behavioral capture, broad user profiling, and evidentiary accumulation without a bounded accountable act are incompatible with ACTA and MUST be rejected.

## 5. Profile Lifecycle Model

### 5.1 `draft`

Entry requirements:
- Problem statement exists.
- Namespace proposal exists.
- Initial event inventory exists.

Requirements:
- The profile MUST state scope, non-scope, and target material effects.
- The profile MUST identify expected tensions with Foundations.

Restrictions:
- MUST NOT be advertised as interoperable.
- MUST NOT be used as a reference profile for other profiles.

### 5.2 `experimental`

Entry requirements:
- Initial specification text exists.
- At least one example flow exists.
- Basic validation rules exist.

Requirements:
- The profile MUST warn that semantics may change.
- The profile MUST expose unresolved design risks.

Restrictions:
- SHOULD NOT be used for durable production evidence without explicit accept-risk documentation.
- MUST NOT claim stable backward compatibility.

### 5.3 `provisional`

Entry requirements:
- Registry submission package is complete.
- Validation checklist passes with limited exceptions.
- Naming and lifecycle semantics have maintainer review.

Requirements:
- The profile MUST define migration expectations.
- The profile MUST define policy reference and commitment semantics.
- The profile SHOULD have at least one reference implementation or example bundle.

Restrictions:
- MAY be used in bounded production contexts.
- MUST disclose known semantic risks.

### 5.4 `stable`

Entry requirements:
- No unresolved constitutional contradictions.
- Lifecycle model is coherent.
- Backward compatibility policy is documented.
- At least one interoperable consumer or verified use case exists.

Requirements:
- Breaking changes MUST follow version policy.
- Deprecations MUST be announced with migration notes.
- Validation artifacts MUST be maintained.

Restrictions:
- Semantic changes without versioning are forbidden.
- Registry removal requires deprecation or retirement process.

### 5.5 `deprecated`

Entry requirements:
- A replacement, sunset rationale, or incompatibility record exists.

Requirements:
- The profile MUST document the deprecation reason.
- The profile MUST define whether old events remain valid for reading, verification, and dispute use.

Restrictions:
- New implementations SHOULD NOT start from a deprecated profile.
- Registries MUST preserve historical discoverability.

### 5.6 `retired`

Entry requirements:
- The profile is no longer approved for new use.
- Historical handling policy is documented.

Requirements:
- The registry MUST preserve identifier, version history, and retirement rationale.
- The profile MUST state whether historical events remain interpretable or only archival.

Restrictions:
- No new events SHOULD be produced under a retired profile version.

## 6. Profile Registry Rules

### 6.1 Submission

A new profile MAY be proposed by any contributor able to supply the required specification package. Proposal alone does not imply acceptance.

### 6.2 Approval

Until the repository defines a stricter governance body, approval MUST require:
- one maintainer approval for repository fit, and
- one substantive semantic review confirming compatibility with `ACTA Foundations v1.1`.

If the repo later defines a formal architecture or protocol council, that body MAY replace the second role.

### 6.3 Minimum submission package

Every profile registration MUST include:
- profile identifier
- namespace
- semantic version
- lifecycle status
- maintainer or owning group
- scope statement
- explicit non-scope statement
- list of event kinds
- lifecycle model
- material effect model
- policy reference model
- commitment model if commitments are used
- compatibility statement with Core and Protocol
- backward compatibility policy
- migration notes if superseding another profile
- at least one normative example flow

### 6.4 Mandatory artifacts

Each profile submission MUST provide:
- a markdown specification
- machine-readable schema or equivalent validation artifact for profile metadata
- example events or bundles
- acceptance checklist outcome

### 6.5 Rejection conditions

A profile MUST be rejected if any of the following holds:
- it contradicts `ACTA Foundations v1.1`
- it attempts to move institutional judgment into ACTA semantics
- it defines event kinds with ambiguous names and no stable meaning
- it captures broad monitoring data without bounded material effect
- it lacks resolvable policy references where policy governs interpretation
- it uses commitments that cannot later be validated
- it duplicates an existing profile with only cosmetic renaming

## 7. Validation Checklist

Use this checklist to classify a profile as `accept`, `request changes`, or `reject`.

1. Does the profile define a stable identifier, namespace, and version?
2. Does the profile clearly define scope and non-scope?
3. Are event kinds named narrowly enough to avoid overloaded interpretation?
4. Does each lifecycle label have one stable meaning?
5. Are close, reopen, revoke, supersede, and cancel semantics explicit where relevant?
6. Are material-effect events identified?
7. Does the profile capture significant acts rather than operational noise?
8. Does the profile satisfy minimization and avoid surveillance-style overcollection?
9. Are policy references resolvable at the time of event interpretation?
10. Are commitments tied to real committed objects and later verification paths?
11. Are event ordering and contradiction rules explicit where domain logic requires them?
12. Does the profile avoid redefining Core facts as institutional judgment?
13. Is backward compatibility policy documented?
14. Are migration notes present when replacing or revising prior semantics?
15. Are examples sufficient to validate dispute-readiness?

Decision rule:
- `accept` if all mandatory constitutional checks pass and any remaining gaps are editorial only
- `request changes` if the profile is directionally valid but underspecified
- `reject` if constitutional incompatibility, ambiguity, or surveillance-pattern design remains

## 8. Failure Modes

### 8.1 Semantic drift

A profile dies when names remain the same but meaning changes across implementations or versions. Drift is usually caused by weak definitions, undocumented exceptions, and domain pressure to overload existing terms.

### 8.2 Overloaded names

A profile dies when generic labels such as `reviewed`, `updated`, or `flagged` absorb too many meanings and force consumers to infer semantics from context.

### 8.3 Fake commitments

A profile dies when commitments are treated as accountability theater: hashes exist, but nobody can determine what was committed, by whom, under which rules, or how disclosure would validate the claim.

### 8.4 Vague policy references

A profile dies when policy references point to mutable wikis, informal playbooks, or unversioned labels. If a consumer cannot determine the governing rule set at event time, the event is not dispute-ready.

### 8.5 Contradictory lifecycle states

A profile dies when closure, cancellation, rejection, suspension, and reopening are partially overlapping or depend on implementer folklore.

### 8.6 Excessive granularity

A profile dies when it records every technical action instead of the bounded acts that matter externally. Granularity inflation raises cost, reduces clarity, and pushes ACTA toward surveillance.

### 8.7 Under-specified events

A profile dies when event kinds exist without actor rules, sequence rules, evidence expectations, or material-effect meaning. Such events become decorative rather than accountable.

### 8.8 Surveillance collapse

A profile dies when it justifies broad behavioral capture in the name of future auditability. This violates proportionality, minimization, and anti-vigilance constraints from Foundations.

### 8.9 Domain enforcement masquerading as ACTA semantics

A profile dies when it encodes judgments such as guilt, compliance finality, legitimacy, or sanction authority as if ACTA itself produced them. ACTA prepares objects for external judgment; it does not contain judgment.

### 8.10 Evidentiary overload without dispute utility

A profile dies when it stores more and more data without improving attribution, reconstruction, or challengeability. More evidence is not automatically better evidence.

## 9. Final Clause

This architecture protects semantic interoperability, disciplined profile evolution, bounded extensibility, and dispute-ready accountability for significant computational acts.

This architecture does not protect or guarantee justice, truth material, legitimacy of external policy, institutional adoption, or correct action by external authorities. It does not authorize ACTA profiles to convert traceability into sanction, nor does it permit semantic sprawl to be justified by domain convenience alone.

When a profile-specific interpretation conflicts with `ACTA Foundations v1.1`, Foundations prevails.

## 10. Core Implications

### 10.1 Strictly necessary

The Core SHOULD expose enough stable primitives to let profiles reference:
- event kind identifiers
- lifecycle-relevant links
- policy references
- actor references
- evidence references
- commitments

If any of these primitives are absent or too weak to support dispute-ready profiles, the gap MUST be resolved at Core or Protocol level before stable profile governance is claimed.

### 10.2 Recommended

The repository SHOULD add:
- a profile registry document or manifest format
- profile metadata schema validation
- naming and lifecycle lint rules
- compatibility declaration templates

### 10.3 Must remain outside Core

The following MUST remain outside Core unless a future constitutional reform says otherwise:
- sector-specific ontologies
- institutional enforcement logic
- approval authority semantics
- broad domain taxonomies
- surveillance-oriented observability models
