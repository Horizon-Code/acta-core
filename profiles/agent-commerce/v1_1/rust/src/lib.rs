//! `agent_commerce` Profile v1.1 — refusal, resolution and countable silence.
//!
//! Implements Accepted ADR-016 under principles 4.16, 4.17 and 4.18. v1.1 adds vocabulary; the
//! seven v1.0 event kinds are consumed unchanged so a v1.0 dossier keeps validating under v1.0
//! rules. ADR-010's refusal of a terminal event is preserved: nothing here claims completion.

use acta_agent_commerce_profile::types::{AgentCommerceDomainEventV1, AGENT_COMMERCE_NAMESPACE_V1};
use acta_core::types::EventKindRefV0;
use serde::{Deserialize, Serialize};

pub mod conditions;
pub mod coverage;

pub const VERSION: &str = "1.1";

/// Who resolved, and of what nature. Recording the nature is description; weighting it is the
/// reader's job, so nothing here ranks these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverNatureV1 {
    Counterparty,
    Arbitrator,
    AutomatedEscrow,
    HumanAdjudicator,
    Authority,
}

impl ResolverNatureV1 {
    /// Whether the resolver is a party to the transaction. A fact, not a verdict: a
    /// counterparty conceding a claim is an ordinary and often honest outcome.
    pub fn is_party_to_the_transaction(&self) -> bool {
        matches!(self, Self::Counterparty)
    }
}

/// What happened to the claim — never what happened to a party. `won`/`lost` would let the
/// emitting party's viewpoint into the evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeOutcomeV1 {
    /// estimada
    Upheld,
    /// desestimada
    Dismissed,
    /// parcialmente estimada
    PartiallyUpheld,
    /// retirada
    Withdrawn,
    /// caducada
    Lapsed,
    /// acordada
    Settled,
}

impl DisputeOutcomeV1 {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Upheld => "upheld",
            Self::Dismissed => "dismissed",
            Self::PartiallyUpheld => "partially_upheld",
            Self::Withdrawn => "withdrawn",
            Self::Lapsed => "lapsed",
            Self::Settled => "settled",
        }
    }

    /// Local-language equivalence, documented per ADR-016 and with no normative force.
    pub fn equivalencia_es(&self) -> &'static str {
        match self {
            Self::Upheld => "estimada",
            Self::Dismissed => "desestimada",
            Self::PartiallyUpheld => "parcialmente estimada",
            Self::Withdrawn => "retirada",
            Self::Lapsed => "caducada",
            Self::Settled => "acordada",
        }
    }
}

/// ADR-016 §1. An attempt a control stopped, bound to the mandate it was checked against —
/// which is what makes it evidence in this domain rather than a bare log line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRefusedAgainstMandatePayloadV1 {
    pub transaction_id: String,
    pub attempted_ref: String,
    pub attempted_input_commitment: String,
    pub control_ref: String,
    pub grounds_commitment: String,
    /// The mandate manifest the attempt was checked against.
    pub mandate_manifest_commitment: String,
    pub refused_at: String,
}

/// ADR-016 §2. Closes a specific `dispute_opened`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisputeResolvedPayloadV1 {
    pub transaction_id: String,
    /// The `dispute_opened` this closes, by event hash: bound, not merely adjacent.
    pub resolved_dispute_event_hash: String,
    pub outcome: DisputeOutcomeV1,
    pub resolver_ref: String,
    pub resolver_nature: ResolverNatureV1,
    /// The norm it was resolved against — here the mandate already committed, so a resolution
    /// cannot silently invoke a norm nobody committed to.
    pub mandate_manifest_commitment: String,
    pub reasoning_commitment: String,
    pub resolved_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCommerceDomainEventV1_1 {
    ActionRefusedAgainstMandate(ActionRefusedAgainstMandatePayloadV1),
    DisputeResolved(DisputeResolvedPayloadV1),
    Inherited(AgentCommerceDomainEventV1),
}

impl AgentCommerceDomainEventV1_1 {
    pub fn kind(&self) -> String {
        match self {
            Self::ActionRefusedAgainstMandate(_) => "action_refused_against_mandate".to_string(),
            Self::DisputeResolved(_) => "dispute_resolved".to_string(),
            Self::Inherited(inner) => inner.to_event_kind_ref().kind,
        }
    }

    pub fn to_event_kind_ref(&self) -> EventKindRefV0 {
        EventKindRefV0 {
            namespace: AGENT_COMMERCE_NAMESPACE_V1.to_string(),
            kind: self.kind(),
            version: VERSION.to_string(),
        }
    }

    pub fn transaction_id(&self) -> &str {
        match self {
            Self::ActionRefusedAgainstMandate(p) => &p.transaction_id,
            Self::DisputeResolved(p) => &p.transaction_id,
            Self::Inherited(inner) => inner.transaction_id(),
        }
    }
}
