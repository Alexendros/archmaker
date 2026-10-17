//! Procedencia y confianza de observaciones (backend emite verificationStatus).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProbeStatus {
    Detected,
    Unknown,
    Unavailable,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ObservationOrigin {
    LocalProbe,
    ImportedReport,
    ManualDeclaration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetBinding {
    CurrentHostConfirmed,
    OtherTarget,
    Unbound,
}

/// Resultado de confianza emitido por el backend (no por un clic de UI).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationStatus {
    ObservedBound,
    DeclaredOnly,
    ImportedUnverified,
    InsufficientEvidence,
    AmbiguousIdentity,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProbeObservation<T> {
    pub status: ProbeStatus,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<T>,
    pub source: String,
    pub observed_at: String,
    pub origin: ObservationOrigin,
    pub target_binding: TargetBinding,
    pub verification_status: VerificationStatus,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub diagnostic: Option<String>,
}

impl<T> ProbeObservation<T> {
    pub fn detected(
        value: T,
        source: impl Into<String>,
        observed_at: impl Into<String>,
        origin: ObservationOrigin,
        binding: TargetBinding,
        verification: VerificationStatus,
    ) -> Self {
        Self {
            status: ProbeStatus::Detected,
            value: Some(value),
            source: source.into(),
            observed_at: observed_at.into(),
            origin,
            target_binding: binding,
            verification_status: verification,
            diagnostic: None,
        }
    }

    pub fn unknown(
        source: impl Into<String>,
        observed_at: impl Into<String>,
        origin: ObservationOrigin,
        diagnostic: impl Into<String>,
    ) -> Self {
        Self {
            status: ProbeStatus::Unknown,
            value: None,
            source: source.into(),
            observed_at: observed_at.into(),
            origin,
            target_binding: TargetBinding::Unbound,
            verification_status: VerificationStatus::InsufficientEvidence,
            diagnostic: Some(diagnostic.into()),
        }
    }

    pub fn unavailable(
        source: impl Into<String>,
        observed_at: impl Into<String>,
        origin: ObservationOrigin,
        diagnostic: impl Into<String>,
    ) -> Self {
        Self {
            status: ProbeStatus::Unavailable,
            value: None,
            source: source.into(),
            observed_at: observed_at.into(),
            origin,
            target_binding: TargetBinding::Unbound,
            verification_status: VerificationStatus::Unavailable,
            diagnostic: Some(diagnostic.into()),
        }
    }
}
