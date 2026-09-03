//! Structural residual-trust conditions for the `ai_agent` profile.
//!
//! Only conditions whose landing schedule says "with v1.1 ratification" are here. Structural
//! conditions are *declared* by the version in use, not detected in the data, so they need no
//! detection logic — which is why they can land now.
//!
//! `TR-COVERAGE-GAP` is deliberately **absent**. It is a detected condition and needs report
//! integration; emitting the code before the functionality that makes it detectable would be a
//! claim without a check behind it.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralConditionV1 {
    pub code: &'static str,
    pub detail: &'static str,
}

const COVERAGE_DECLARED: StructuralConditionV1 = StructuralConditionV1 {
    code: "TR-COVERAGE-DECLARED",
    detail: "Los intervalos de cobertura son afirmación del productor, no hecho atestiguado \
             externamente. Están encadenados y no son reescribibles después, pero siguen siendo \
             afirmación.",
};

const DENIAL_UNSUPPORTED: StructuralConditionV1 = StructuralConditionV1 {
    code: "TR-DENIAL-UNSUPPORTED",
    detail: "La versión de perfil en uso no puede expresar rechazos, así que su ausencia no \
             prueba nada: no se puede distinguir «no se rechazó nada» de «los rechazos son \
             irregistrables».",
};

const AGENT_IDENTITY_SELF_ASSERTED: StructuralConditionV1 = StructuralConditionV1 {
    code: "TR-AGENT-IDENTITY-SELF-ASSERTED",
    detail: "La identidad de agente de la que cuelgan las sesiones es autoafirmada; nada impide \
             acuñar una nueva por sesión. El techo se mantiene hasta que exista identidad \
             verificable externamente.",
};

/// Structural conditions a dossier carries because of the profile version that produced it.
///
/// A v1.0 dossier keeps reporting `TR-DENIAL-UNSUPPORTED` forever. Retiring it for v1.1
/// producers must never retire it retroactively for records that really were produced under a
/// version that could not express a refusal — an old dossier has to keep telling the truth
/// about itself.
pub fn structural_conditions_for(profile_version: &str) -> Vec<StructuralConditionV1> {
    match profile_version {
        "1.1" => vec![COVERAGE_DECLARED, AGENT_IDENTITY_SELF_ASSERTED],
        // Every version before 1.1, including anything unrecognised: assume the weaker claim.
        _ => vec![DENIAL_UNSUPPORTED],
    }
}
