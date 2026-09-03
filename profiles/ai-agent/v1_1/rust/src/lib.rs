//! `ai_agent` Profile v1.1 — recording what did not happen.
//!
//! Implements Accepted ADR-015. v1.1 adds vocabulary; it reinterprets nothing. The six v1.0
//! event kinds are consumed from the v1.0 crate unchanged, so a v1.0 dossier keeps validating
//! under v1.0 rules and keeps telling the truth about itself.
//!
//! Core and Protocol v0 are untouched: this crate maps to `ActaEventV0` through the same public
//! surface any profile uses.

use acta_ai_agent_profile::types::AiAgentDomainEventV0;
use acta_core::types::EventKindRefV0;
use serde::{Deserialize, Serialize};

pub mod conditions;
pub mod lifecycle;

pub const NAMESPACE: &str = "ai_agent";
pub const VERSION: &str = "1.1";

/// Who or what refused. Naming the nature is description; weighting it is the reader's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlNatureV1 {
    /// A human decided.
    Human,
    /// A model reviewed and decided.
    ReviewerModel,
    /// A deterministic rule, allowlist or hard floor.
    Policy,
}

/// ADR-015 §1. What was attempted and refused.
///
/// It carries no verdict about whether refusing was correct, and it is not a failure: a denial
/// exists only where some control evaluated the conduct and decided against it. Timeouts,
/// retries, malformed input and transport errors are plumbing and do not belong here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionDeniedPayloadV1 {
    pub run_id: String,
    pub session_id: String,
    /// What was attempted.
    pub attempted_ref: String,
    pub attempted_input_commitment: String,
    /// Which control refused it, and of what nature.
    pub control_ref: String,
    pub control_nature: ControlNatureV1,
    /// Commitment over the stated reasoning, whoever stated it.
    pub reasoning_commitment: String,
    pub denied_at: String,
}

/// ADR-015 §3. Same shape as `PolicySnapshotV0`, deliberately its own struct: a policy and a
/// coverage manifest are different objects, and putting a coverage hash into `policy_hash`
/// would be the overloading ADR-012 §3 refused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageSnapshotV1 {
    pub coverage_id: String,
    pub coverage_hash: String,
    /// The event kinds this producer commits to emitting while the interval is open.
    pub declared_kinds: Vec<String>,
    pub effective_from: String,
    pub effective_to: Option<String>,
}

/// ADR-015 §2 and §4. Opens a declared interval of coverage, hung from an agent identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionOpenedPayloadV1 {
    pub session_id: String,
    /// The level above `process_ref`: which agent this session belongs to.
    pub agent_identity_ref: String,
    pub agent_identity_commitment: String,
    pub coverage: CoverageSnapshotV1,
    pub opened_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionClosedPayloadV1 {
    pub session_id: String,
    pub closed_at: String,
}

/// v1.1 = the six v1.0 kinds, unchanged, plus three.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiAgentDomainEventV1 {
    SessionOpened(SessionOpenedPayloadV1),
    SessionClosed(SessionClosedPayloadV1),
    ActionDenied(ActionDeniedPayloadV1),
    /// A v1.0 event kind, carried verbatim.
    Inherited(AiAgentDomainEventV0),
}

impl AiAgentDomainEventV1 {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::SessionOpened(_) => "session_opened",
            Self::SessionClosed(_) => "session_closed",
            Self::ActionDenied(_) => "action_denied",
            Self::Inherited(_) => "inherited",
        }
    }

    pub fn to_event_kind_ref(&self) -> EventKindRefV0 {
        match self {
            Self::Inherited(inner) => EventKindRefV0 {
                namespace: NAMESPACE.to_string(),
                kind: inner.to_event_kind_ref().kind,
                version: VERSION.to_string(),
            },
            other => EventKindRefV0 {
                namespace: NAMESPACE.to_string(),
                kind: other.kind().to_string(),
                version: VERSION.to_string(),
            },
        }
    }

    /// The kind name as it appears in a lifecycle, flattening `Inherited`.
    pub fn lifecycle_kind(&self) -> String {
        match self {
            Self::Inherited(inner) => inner.to_event_kind_ref().kind,
            other => other.kind().to_string(),
        }
    }

    pub fn occurred_at(&self) -> &str {
        match self {
            Self::SessionOpened(p) => &p.opened_at,
            Self::SessionClosed(p) => &p.closed_at,
            Self::ActionDenied(p) => &p.denied_at,
            Self::Inherited(inner) => inner.occurred_at(),
        }
    }

    pub fn session_id(&self) -> Option<&str> {
        match self {
            Self::SessionOpened(p) => Some(&p.session_id),
            Self::SessionClosed(p) => Some(&p.session_id),
            Self::ActionDenied(p) => Some(&p.session_id),
            Self::Inherited(_) => None,
        }
    }
}

/// Map a v1.1 domain event onto a Core event.
///
/// Inherited kinds are delegated to the v1.0 mapper and then stamped with the v1.1 version, so
/// the commitment semantics of v1.0 are reused byte for byte instead of being reimplemented.
/// The three new kinds are built here.
pub fn map_v1_1_event_to_core_v0(
    event: &AiAgentDomainEventV1,
    event_id: String,
    run_id: &str,
    policy_snapshot: &acta_core::types::PolicySnapshotV0,
) -> Result<acta_core::types::ActaEventV0, String> {
    use acta_core::types::{
        ActaEventV0, ActorRefV0, CommitmentsV0, ProcessRefV0, PROTOCOL_VERSION,
    };

    if let AiAgentDomainEventV1::Inherited(inner) = event {
        let mut mapped =
            acta_ai_agent_profile::map_ai_agent_event_to_core_v0(inner, event_id, policy_snapshot)
                .map_err(|e| e.to_string())?;
        mapped.event_kind = event.to_event_kind_ref();
        return Ok(mapped);
    }

    let (actor, commitment) = match event {
        AiAgentDomainEventV1::SessionOpened(p) => (
            p.agent_identity_ref.clone(),
            p.agent_identity_commitment.clone(),
        ),
        AiAgentDomainEventV1::SessionClosed(p) => (
            format!("ref:session:{}", p.session_id),
            format!("sha256:{}", "0".repeat(64)),
        ),
        AiAgentDomainEventV1::ActionDenied(p) => {
            (p.control_ref.clone(), p.reasoning_commitment.clone())
        }
        AiAgentDomainEventV1::Inherited(_) => unreachable!("handled above"),
    };

    let mapped = ActaEventV0 {
        protocol: PROTOCOL_VERSION.to_string(),
        event_id,
        issued_at: event.occurred_at().to_string(),
        process_ref: ProcessRefV0 {
            process_id: run_id.to_string(),
            process_type: "ai_agent_run".to_string(),
        },
        event_kind: event.to_event_kind_ref(),
        commitments: CommitmentsV0 {
            inputs_commitment: commitment.clone(),
            outputs_commitment: commitment.clone(),
            artifact_commitment: commitment,
        },
        policy_snapshot: policy_snapshot.clone(),
        actor_ref: ActorRefV0 {
            actor_id: actor,
            actor_type: "service".to_string(),
        },
    };
    acta_core::types::validate_event_v0_shape(&mapped).map_err(|e| e.to_string())?;
    Ok(mapped)
}
