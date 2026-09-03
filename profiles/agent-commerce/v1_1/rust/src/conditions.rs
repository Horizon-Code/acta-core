//! Residual-trust conditions for `agent_commerce`.
//!
//! Structural conditions are declared by the version in use. Detected conditions are emitted
//! only where the check that makes them detectable lives in this crate — never before.

use crate::coverage::{compare_mandate_to_record, DeclaredMilestonesV1};
use crate::{AgentCommerceDomainEventV1_1, ResolverNatureV1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionV1 {
    pub code: String,
    pub detail: String,
}

/// Declared by the producing version, not detected in the data.
///
/// A v1.0 dossier keeps reporting both `*-UNSUPPORTED` codes forever. Retiring them for v1.1
/// producers must never retire them retroactively: a record produced under a version that could
/// not express a refusal has to keep saying so.
pub fn structural_conditions_for(profile_version: &str) -> Vec<ConditionV1> {
    if profile_version == "1.1" {
        return Vec::new();
    }
    vec![
        ConditionV1 {
            code: "TR-REFUSAL-UNSUPPORTED".into(),
            detail: "La versión de perfil en uso no puede expresar acciones rechazadas contra el \
                     mandato, así que su ausencia no prueba nada."
                .into(),
        },
        ConditionV1 {
            code: "TR-DISPUTE-RESOLUTION-UNSUPPORTED".into(),
            detail: "La versión de perfil en uso no puede expresar desenlaces de disputa, así \
                     que una disputa abierta pudo resolverse de forma irregistrable."
                .into(),
        },
    ]
}

/// Detected in the record. Each code here has its check in this crate.
pub fn detected_conditions(
    events: &[AgentCommerceDomainEventV1_1],
    declared: Option<&DeclaredMilestonesV1>,
    at: &str,
) -> Vec<ConditionV1> {
    let mut conditions = Vec::new();

    // TR-DISPUTE-OPEN: a dispute with no resolution bound to it.
    let resolved: Vec<&str> = events
        .iter()
        .filter_map(|e| match e {
            AgentCommerceDomainEventV1_1::DisputeResolved(p) => {
                Some(p.resolved_dispute_event_hash.as_str())
            }
            _ => None,
        })
        .collect();
    let opened = events
        .iter()
        .filter(|e| e.kind() == "dispute_opened")
        .count();
    if opened > resolved.len() {
        conditions.push(ConditionV1 {
            code: "TR-DISPUTE-OPEN".into(),
            detail: format!(
                "{} disputa(s) abierta(s) sin resolución vinculada. Distingue «sigue abierta» de \
                 «se resolvió y no se registró» solo si el productor es v1.1.",
                opened - resolved.len()
            ),
        });
    }

    // TR-RESOLVER-SELF-INTERESTED: states a fact and stops.
    for event in events {
        if let AgentCommerceDomainEventV1_1::DisputeResolved(p) = event {
            if p.resolver_nature.is_party_to_the_transaction() {
                conditions.push(ConditionV1 {
                    code: "TR-RESOLVER-SELF-INTERESTED".into(),
                    detail: format!(
                        "La disputa se resolvió como «{}» por {}, que es parte de la \
                         transacción. Es constatación, no invalidación: una contraparte que \
                         reconoce una reclamación es un desenlace ordinario.",
                        p.outcome.as_str(),
                        p.resolver_ref
                    ),
                });
            }
        }
    }

    // TR-MANDATE-COVERAGE-GAP: the check lives in `coverage`.
    if let Some(declared) = declared {
        for discrepancy in compare_mandate_to_record(declared, events, at) {
            conditions.push(ConditionV1 {
                code: "TR-MANDATE-COVERAGE-GAP".into(),
                detail: discrepancy.statement(),
            });
        }
    }

    conditions
}

/// Nothing here ranks resolvers; this exists so a report can name the nature it observed.
pub fn resolver_nature_label(nature: ResolverNatureV1) -> &'static str {
    match nature {
        ResolverNatureV1::Counterparty => "counterparty",
        ResolverNatureV1::Arbitrator => "arbitrator",
        ResolverNatureV1::AutomatedEscrow => "automated_escrow",
        ResolverNatureV1::HumanAdjudicator => "human_adjudicator",
        ResolverNatureV1::Authority => "authority",
    }
}
