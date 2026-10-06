use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorFamily {
    Doc,
    Schema,
    Cat,
    Mig,
    Rule,
    Res,
    Tgt,
    Io,
    Proto,
    Run,
    Pol,
    Auth,
}

impl ErrorFamily {
    pub fn code(self) -> &'static str {
        match self {
            ErrorFamily::Doc => "AM-DOC",
            ErrorFamily::Schema => "AM-SCHEMA",
            ErrorFamily::Cat => "AM-CAT",
            ErrorFamily::Mig => "AM-MIG",
            ErrorFamily::Rule => "AM-RULE",
            ErrorFamily::Res => "AM-RES",
            ErrorFamily::Tgt => "AM-TGT",
            ErrorFamily::Io => "AM-IO",
            ErrorFamily::Proto => "AM-PROTO",
            ErrorFamily::Run => "AM-RUN",
            ErrorFamily::Pol => "AM-POL",
            ErrorFamily::Auth => "AM-AUTH",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Validation,
    Review,
    Conflict,
    Io,
    Canonicalization,
    Catalog,
    Migration,
    Rule,
    Resolution,
    Target,
    Protocol,
    Policy,
    Auth,
    Internal,
}

impl ErrorCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCategory::Validation => "validation",
            ErrorCategory::Review => "review",
            ErrorCategory::Conflict => "conflict",
            ErrorCategory::Io => "io",
            ErrorCategory::Canonicalization => "canonicalization",
            ErrorCategory::Catalog => "catalog",
            ErrorCategory::Migration => "migration",
            ErrorCategory::Rule => "rule",
            ErrorCategory::Resolution => "resolution",
            ErrorCategory::Target => "target",
            ErrorCategory::Protocol => "protocol",
            ErrorCategory::Policy => "policy",
            ErrorCategory::Auth => "auth",
            ErrorCategory::Internal => "internal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }
}

fn serialize_family<S>(family: &ErrorFamily, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(family.code())
}

fn deserialize_family<'de, D>(deserializer: D) -> std::result::Result<ErrorFamily, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    match raw.as_str() {
        "AM-DOC" => Ok(ErrorFamily::Doc),
        "AM-SCHEMA" => Ok(ErrorFamily::Schema),
        "AM-CAT" => Ok(ErrorFamily::Cat),
        "AM-MIG" => Ok(ErrorFamily::Mig),
        "AM-RULE" => Ok(ErrorFamily::Rule),
        "AM-RES" => Ok(ErrorFamily::Res),
        "AM-TGT" => Ok(ErrorFamily::Tgt),
        "AM-IO" => Ok(ErrorFamily::Io),
        "AM-PROTO" => Ok(ErrorFamily::Proto),
        "AM-RUN" => Ok(ErrorFamily::Run),
        "AM-POL" => Ok(ErrorFamily::Pol),
        "AM-AUTH" => Ok(ErrorFamily::Auth),
        other => Err(serde::de::Error::custom(format!(
            "unknown error family {other}"
        ))),
    }
}

fn serialize_category<S>(
    category: &ErrorCategory,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(category.as_str())
}

fn deserialize_category<'de, D>(deserializer: D) -> std::result::Result<ErrorCategory, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    match raw.as_str() {
        "validation" => Ok(ErrorCategory::Validation),
        "review" => Ok(ErrorCategory::Review),
        "conflict" => Ok(ErrorCategory::Conflict),
        "io" => Ok(ErrorCategory::Io),
        "canonicalization" => Ok(ErrorCategory::Canonicalization),
        "catalog" => Ok(ErrorCategory::Catalog),
        "migration" => Ok(ErrorCategory::Migration),
        "rule" => Ok(ErrorCategory::Rule),
        "resolution" => Ok(ErrorCategory::Resolution),
        "target" => Ok(ErrorCategory::Target),
        "protocol" => Ok(ErrorCategory::Protocol),
        "policy" => Ok(ErrorCategory::Policy),
        "auth" => Ok(ErrorCategory::Auth),
        "internal" => Ok(ErrorCategory::Internal),
        other => Err(serde::de::Error::custom(format!(
            "unknown error category {other}"
        ))),
    }
}

fn serialize_severity<S>(severity: &Severity, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(severity.as_str())
}

fn deserialize_severity<'de, D>(deserializer: D) -> std::result::Result<Severity, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    match raw.as_str() {
        "error" => Ok(Severity::Error),
        "warning" => Ok(Severity::Warning),
        "info" => Ok(Severity::Info),
        other => Err(serde::de::Error::custom(format!(
            "unknown severity {other}"
        ))),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreError {
    #[serde(
        serialize_with = "serialize_family",
        deserialize_with = "deserialize_family"
    )]
    pub family: ErrorFamily,
    pub code: String,
    #[serde(
        serialize_with = "serialize_category",
        deserialize_with = "deserialize_category"
    )]
    pub category: ErrorCategory,
    #[serde(
        serialize_with = "serialize_severity",
        deserialize_with = "deserialize_severity"
    )]
    pub severity: Severity,
    pub retryable: bool,
    pub message_key: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cause_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cause: Option<Box<CoreError>>,
}

pub type Result<T> = std::result::Result<T, CoreError>;

impl CoreError {
    fn new(
        code: &str,
        family: ErrorFamily,
        category: ErrorCategory,
        severity: Severity,
        retryable: bool,
        message_key: &str,
        source: &str,
    ) -> Self {
        Self {
            code: code.to_string(),
            family,
            category,
            severity,
            retryable,
            message_key: message_key.to_string(),
            source: source.to_string(),
            cause_code: None,
            cause: None,
        }
    }

    pub fn with_cause(mut self, cause: CoreError) -> Self {
        self.cause_code = Some(cause.code.clone());
        self.cause = Some(Box::new(cause));
        self
    }

    pub fn doc_limit(source: &str) -> Self {
        Self::new(
            "AM-DOC-001",
            ErrorFamily::Doc,
            ErrorCategory::Validation,
            Severity::Error,
            false,
            "error.document.limitExceeded",
            source,
        )
    }

    pub fn doc_conflict(source: &str) -> Self {
        Self::new(
            "AM-DOC-002",
            ErrorFamily::Doc,
            ErrorCategory::Conflict,
            Severity::Error,
            true,
            "error.draft.revisionConflict",
            source,
        )
    }

    pub fn doc_canonical(source: &str) -> Self {
        Self::new(
            "AM-DOC-003",
            ErrorFamily::Doc,
            ErrorCategory::Canonicalization,
            Severity::Error,
            false,
            "error.canonicalization.rejected",
            source,
        )
    }

    pub fn schema_invalid(source: &str) -> Self {
        Self::new(
            "AM-SCHEMA-001",
            ErrorFamily::Schema,
            ErrorCategory::Validation,
            Severity::Error,
            false,
            "error.schema.invalidDocument",
            source,
        )
    }

    pub fn cat_not_found(source: &str) -> Self {
        Self::new(
            "AM-CAT-001",
            ErrorFamily::Cat,
            ErrorCategory::Catalog,
            Severity::Error,
            false,
            "error.catalog.notFound",
            source,
        )
    }

    pub fn cat_digest_mismatch(source: &str) -> Self {
        Self::new(
            "AM-CAT-002",
            ErrorFamily::Cat,
            ErrorCategory::Catalog,
            Severity::Error,
            false,
            "error.catalog.digestMismatch",
            source,
        )
    }

    pub fn res_unresolved(source: &str) -> Self {
        Self::new(
            "AM-RES-001",
            ErrorFamily::Res,
            ErrorCategory::Resolution,
            Severity::Error,
            false,
            "error.resolution.valueUnresolved",
            source,
        )
    }

    pub fn res_blocked(source: &str) -> Self {
        Self::new(
            "AM-RES-002",
            ErrorFamily::Res,
            ErrorCategory::Resolution,
            Severity::Error,
            false,
            "error.resolution.blocked",
            source,
        )
    }

    pub fn res_capability_missing(source: &str) -> Self {
        Self::new(
            "AM-RES-004",
            ErrorFamily::Res,
            ErrorCategory::Resolution,
            Severity::Error,
            false,
            "diagnostic.capabilityMissing",
            source,
        )
    }

    pub fn tgt_unsupported(source: &str) -> Self {
        Self::new(
            "AM-TGT-001",
            ErrorFamily::Tgt,
            ErrorCategory::Target,
            Severity::Error,
            false,
            "error.target.unsupported",
            source,
        )
    }

    pub fn io_failed(source: &str) -> Self {
        Self::new(
            "AM-IO-001",
            ErrorFamily::Io,
            ErrorCategory::Io,
            Severity::Error,
            true,
            "error.io.failed",
            source,
        )
    }

    pub fn proto_denied(source: &str) -> Self {
        Self::new(
            "AM-PROTO-001",
            ErrorFamily::Proto,
            ErrorCategory::Protocol,
            Severity::Error,
            false,
            "error.protocol.invalidMessage",
            source,
        )
    }

    pub fn proto_unknown(source: &str) -> Self {
        Self::new(
            "AM-PROTO-002",
            ErrorFamily::Proto,
            ErrorCategory::Protocol,
            Severity::Error,
            false,
            "error.protocol.unsupportedOperation",
            source,
        )
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] {} ({})",
            self.code,
            self.message_key,
            self.category.as_str(),
            self.source
        )
    }
}

impl std::error::Error for CoreError {}
