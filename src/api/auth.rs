use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const OAUTH_AUTHORIZE_URL: &str = "https://oauth.semrush.com/oauth2/authorize";
const OAUTH_TOKEN_URL: &str = "https://oauth.semrush.com/oauth2/access_token";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
}

impl OAuthTokens {
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now().timestamp() >= expires_at
        } else {
            false
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), AppError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| AppError::CacheError {
                message: format!("Failed to create token dir: {e}"),
            })?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| AppError::ParseError {
            message: format!("Failed to serialize tokens: {e}"),
        })?;
        std::fs::write(path, json).map_err(|e| AppError::CacheError {
            message: format!("Failed to save tokens: {e}"),
        })
    }

    pub fn load(path: &PathBuf) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }
}

/// Start the OAuth2 authorization code flow.
/// Prints the authorization URL and instructions.
pub fn print_oauth_setup_instructions(client_id: &str) {
    let url = format!(
        "{OAUTH_AUTHORIZE_URL}?response_type=code&client_id={client_id}&redirect_uri=urn:ietf:wg:oauth:2.0:oob"
    );
    println!("OAuth2 Setup for Semrush v4 API");
    println!("================================");
    println!();
    println!("1. Register your app with Semrush to get client_id and client_secret");
    println!("   (Contact Semrush Tech Support if needed)");
    println!();
    println!("2. Visit this URL in your browser to authorize:");
    println!("   {url}");
    println!();
    println!("3. After approving, you'll receive an authorization code.");
    println!("   Exchange it by running:");
    println!();
    println!("   semrush account auth exchange-token \\");
    println!("     --client-id YOUR_CLIENT_ID \\");
    println!("     --client-secret YOUR_CLIENT_SECRET \\");
    println!("     --code THE_AUTH_CODE");
}

/// Exchange an authorization code for tokens.
pub async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
) -> Result<OAuthTokens, AppError> {
    let http = reqwest::Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = http
        .post(OAUTH_TOKEN_URL)
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::NetworkError {
            message: format!("OAuth token exchange failed: {e}"),
        })?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::AuthFailed {
            message: format!("OAuth token exchange failed: {body}"),
        });
    }

    #[derive(Deserialize)]
    struct TokenResponse {
        access_token: String,
        refresh_token: Option<String>,
        expires_in: Option<i64>,
    }

    let token_resp: TokenResponse = response.json().await.map_err(|e| AppError::ParseError {
        message: format!("Failed to parse token response: {e}"),
    })?;

    let expires_at = token_resp
        .expires_in
        .map(|secs| chrono::Utc::now().timestamp() + secs);

    Ok(OAuthTokens {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
        expires_at,
    })
}

/// Refresh an expired access token.
pub async fn refresh_token(
    client_id: &str,
    client_secret: &str,
    refresh_tok: &str,
) -> Result<OAuthTokens, AppError> {
    let http = reqwest::Client::new();
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_tok),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = http
        .post(OAUTH_TOKEN_URL)
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::NetworkError {
            message: format!("OAuth token refresh failed: {e}"),
        })?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::AuthFailed {
            message: format!("OAuth token refresh failed: {body}"),
        });
    }

    #[derive(Deserialize)]
    struct TokenResponse {
        access_token: String,
        refresh_token: Option<String>,
        expires_in: Option<i64>,
    }

    let token_resp: TokenResponse = response.json().await.map_err(|e| AppError::ParseError {
        message: format!("Failed to parse refresh response: {e}"),
    })?;

    let expires_at = token_resp
        .expires_in
        .map(|secs| chrono::Utc::now().timestamp() + secs);

    Ok(OAuthTokens {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
        expires_at,
    })
}
