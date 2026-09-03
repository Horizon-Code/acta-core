//! ADR-016 §3 under principle 4.18: declared expectation makes silence countable.
//!
//! No terminal event and no completeness claim. The output is never "this transaction is
//! incomplete", which would be a verdict about the world. It is "the committed mandate declared
//! a delivery and the record contains none", a fact about two committed artifacts.

use crate::AgentCommerceDomainEventV1_1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneV1 {
    Delivery,
    Settlement,
}

impl MilestoneV1 {
    /// The v1.0 event kind that evidences the milestone actually happening. Delivery is
    /// evidenced by receipt, not by the commitment to deliver: a party committing to deliver is
    /// not the same as delivery having been received.
    pub fn observed_by(&self) -> &'static str {
        match self {
            Self::Delivery => "delivery_received",
            Self::Settlement => "settlement_observed",
        }
    }
}

/// What the committed mandate declared the transaction would produce.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredMilestonesV1 {
    pub transaction_id: String,
    pub mandate_manifest_commitment: String,
    pub expected: Vec<MilestoneV1>,
    /// From `MandateEntryV1::expires_at`. Bounds when the comparison is meaningful.
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscrepancyV1 {
    pub milestone: MilestoneV1,
    pub declared_by_mandate: bool,
    pub present_in_record: bool,
    /// Whether the mandate's own window has closed at the moment of comparison.
    pub mandate_window_closed: bool,
}

impl DiscrepancyV1 {
    /// A sentence a report can print without adding a verdict.
    pub fn statement(&self) -> String {
        let milestone = match self.milestone {
            MilestoneV1::Delivery => "a delivery",
            MilestoneV1::Settlement => "a settlement",
        };
        if self.mandate_window_closed {
            format!(
                "the committed mandate declared {milestone}, its window has closed, and the \
                 record contains none"
            )
        } else {
            format!(
                "the committed mandate declared {milestone} and the record contains none; the \
                 mandate window is still open"
            )
        }
    }
}

/// Compare what the mandate declared against what the record contains.
///
/// `at` is the moment the comparison is made, used only to decide whether the mandate window has
/// closed. An unexpired mandate missing its declared delivery is a different statement from one
/// that expired without it, and the report must be able to tell them apart.
pub fn compare_mandate_to_record(
    declared: &DeclaredMilestonesV1,
    events: &[AgentCommerceDomainEventV1_1],
    at: &str,
) -> Vec<DiscrepancyV1> {
    let observed: Vec<String> = events
        .iter()
        .filter(|e| e.transaction_id() == declared.transaction_id)
        .map(|e| e.kind())
        .collect();

    let window_closed = declared
        .expires_at
        .as_deref()
        .is_some_and(|expiry| at >= expiry);

    declared
        .expected
        .iter()
        .filter(|milestone| !observed.iter().any(|k| k == milestone.observed_by()))
        .map(|milestone| DiscrepancyV1 {
            milestone: *milestone,
            declared_by_mandate: true,
            present_in_record: false,
            mandate_window_closed: window_closed,
        })
        .collect()
}
