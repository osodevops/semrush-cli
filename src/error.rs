use std::process;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Authentication failed: {message}")]
    AuthFailed { message: String },

    #[error("Rate limited: retry after {retry_after_ms}ms")]
    RateLimited {
        retry_after_ms: u64,
        api_status_code: u16,
    },

    #[error("Insufficient API units: {message}")]
    InsufficientUnits { message: String },

    #[error("Invalid parameters: {message}")]
    InvalidParams { message: String },

    #[error("API error ({status_code}): {message}")]
    ApiError { status_code: u16, message: String },

    #[error("Parse error: {message}")]
    ParseError { message: String },

    #[error("Cache error: {message}")]
    CacheError { message: String },

    #[error("Network error: {message}")]
    NetworkError { message: String },
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::AuthFailed { .. } => 1,
            AppError::RateLimited { .. } => 2,
            AppError::InvalidParams { .. } => 3,
            AppError::InsufficientUnits { .. }
            | AppError::ApiError { .. }
            | AppError::ParseError { .. }
            | AppError::CacheError { .. }
            | AppError::NetworkError { .. } => 4,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::AuthFailed { .. } => "AUTH_FAILED",
            AppError::RateLimited { .. } => "RATE_LIMITED",
            AppError::InsufficientUnits { .. } => "INSUFFICIENT_UNITS",
            AppError::InvalidParams { .. } => "INVALID_PARAMS",
            AppError::ApiError { .. } => "API_ERROR",
            AppError::ParseError { .. } => "PARSE_ERROR",
            AppError::CacheError { .. } => "CACHE_ERROR",
            AppError::NetworkError { .. } => "NETWORK_ERROR",
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut error = serde_json::json!({
            "code": self.error_code(),
            "message": self.to_string(),
        });

        if let AppError::RateLimited {
            retry_after_ms,
            api_status_code,
        } = self
        {
            error["retry_after_ms"] = serde_json::json!(retry_after_ms);
            error["api_status_code"] = serde_json::json!(api_status_code);
        }

        serde_json::json!({ "error": error })
    }

    pub fn print_and_exit(self) -> ! {
        let code = self.exit_code();
        eprintln!("{}", self.to_json());
        process::exit(code);
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::NetworkError {
                message: format!("Request timed out: {err}"),
            }
        } else if err.is_connect() {
            AppError::NetworkError {
                message: format!("Connection failed: {err}"),
            }
        } else {
            AppError::NetworkError {
                message: err.to_string(),
            }
        }
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::ParseError {
            message: format!("CSV parse error: {err}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_codes_match_documented_contract() {
        assert_eq!(
            AppError::AuthFailed {
                message: String::new()
            }
            .exit_code(),
            1
        );
        assert_eq!(
            AppError::RateLimited {
                retry_after_ms: 1000,
                api_status_code: 429
            }
            .exit_code(),
            2
        );
        assert_eq!(
            AppError::InvalidParams {
                message: String::new()
            }
            .exit_code(),
            3
        );
        assert_eq!(
            AppError::NetworkError {
                message: String::new()
            }
            .exit_code(),
            4
        );
    }
}
