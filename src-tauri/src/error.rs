use serde::Serialize;

/// Error codes defined by the frontend/backend contract in `docs/API.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    NotFound,
    InvalidInput,
    Conflict,
    UnavailableOffline,
    EngineError,
    PermissionDenied,
    IoError,
    Locked,
    Unsupported,
    Internal,
}

/// Structured, serialisable error returned to the frontend.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for AppError {}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotFound => "not_found",
            Self::InvalidInput => "invalid_input",
            Self::Conflict => "conflict",
            Self::UnavailableOffline => "unavailable_offline",
            Self::EngineError => "engine_error",
            Self::PermissionDenied => "permission_denied",
            Self::IoError => "io_error",
            Self::Locked => "locked",
            Self::Unsupported => "unsupported",
            Self::Internal => "internal",
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::new(ErrorCode::IoError, err.to_string())
    }
}
