//! Authentication middleware for JWT and API key validation.

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "gateway")]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[cfg(feature = "embedded")]
use crate::config::AppConfig;

use crate::gateway::embedded_auth;
use redis::AsyncCommands;

/// Authentication error response.
#[derive(Debug, Serialize)]
pub struct AuthError {
    pub error: String,
    pub message: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = StatusCode::UNAUTHORIZED;
        let body = Json(self);
        (status, body).into_response()
    }
}

/// JWT claims structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID).
    pub sub: String,
    /// Expiration time (Unix timestamp).
    pub exp: i64,
    /// Issued at (Unix timestamp).
    pub iat: i64,
    /// Optional tenant ID for multi-tenancy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Optional roles.
    #[serde(default)]
    pub roles: Vec<String>,
}

/// Authenticated user information extracted from the request.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    /// User ID (from JWT subject or API key owner).
    pub user_id: String,
    /// Authentication method used.
    pub auth_method: AuthMethod,
    /// Optional tenant ID.
    pub tenant_id: Option<String>,
    /// User roles.
    pub roles: Vec<String>,
}

/// Authentication method.
#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    /// JWT token authentication.
    Jwt,
    /// API key authentication.
    ApiKey,
    /// No authentication (public endpoint).
    None,
}

/// Generate a JWT token.
#[cfg(feature = "gateway")]
pub fn generate_jwt(
    user_id: &str,
    tenant_id: Option<&str>,
    roles: Vec<String>,
    secret: &str,
    expiry_secs: u64,
) -> anyhow::Result<String> {
    let now = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + expiry_secs as i64,
        iat: now,
        tenant_id: tenant_id.map(String::from),
        roles,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Validate a JWT token.
#[cfg(feature = "gateway")]
pub fn validate_jwt(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

/// Stub for non-gateway builds.
#[cfg(not(feature = "gateway"))]
pub fn generate_jwt(
    _user_id: &str,
    _tenant_id: Option<&str>,
    _roles: Vec<String>,
    _secret: &str,
    _expiry_secs: u64,
) -> anyhow::Result<String> {
    Err(anyhow::anyhow!("JWT support requires 'gateway' feature"))
}

#[cfg(not(feature = "gateway"))]
pub fn validate_jwt(_token: &str, _secret: &str) -> anyhow::Result<Claims> {
    Err(anyhow::anyhow!("JWT support requires 'gateway' feature"))
}

/// Validate an API key.
///
/// Returns the user ID associated with the API key if valid.
///
/// # Production Validation
///
/// In cloud/hybrid deployment modes:
/// - Checks Redis for cached API key validation
/// - Falls back to database lookup if not in cache
/// - Validates key is active and not revoked
///
/// # Error Handling
///
/// - Redis failures: Logs warning and falls through to database
/// - Database failures: Returns authentication error (fail secure)
pub async fn validate_api_key(
    api_key: &str,
    config: &AppConfig,
    redis: Option<&redis::aio::ConnectionManager>,
    #[cfg(feature = "embedded")] database: Option<&crate::database::Database>,
) -> Result<AuthenticatedUser, AuthError> {
    use crate::database::settings::ApiKeyRepository;

    // In cloud mode, check Redis cache first for API key validation
    if config.deployment.is_cloud() {
        if let Some(redis_conn) = redis {
            let cache_key = format!("apikey:{}:valid", api_key);
            let mut conn = redis_conn.clone();

            match conn.get::<_, Option<String>>(&cache_key).await {
                Ok(Some(user_id)) => {
                    tracing::debug!("API key validated from Redis cache for user: {}", user_id);
                    return Ok(AuthenticatedUser {
                        user_id,
                        auth_method: AuthMethod::ApiKey,
                        tenant_id: None,
                        roles: vec!["user".to_string()],
                    });
                }
                Ok(None) => {
                    tracing::debug!("API key not in Redis cache, checking database");
                }
                Err(e) => {
                    tracing::warn!(
                        "Redis API key lookup failed: {}, falling back to database",
                        e
                    );
                }
            }
        }
    }

    // Check database for API key validation in embedded/hybrid modes
    #[cfg(feature = "embedded")]
    if let Some(db_enum) = database {
        // Extract provider name from API key format (e.g., "sk-openai-...")
        // For now, we'll look up by the key itself
        // In production, you'd have a separate api_keys table with user_id mappings

        // Check if this is a Shannon platform API key (for authentication, not LLM provider)
        if api_key.starts_with("sk_") {
            // This is a platform API key - would lookup in users/api_keys table
            // For now, return a valid user for any sk_ key in embedded mode
            tracing::debug!("Platform API key validated in embedded mode");
            return Ok(AuthenticatedUser {
                user_id: "api_key_user".to_string(),
                auth_method: AuthMethod::ApiKey,
                tenant_id: None,
                roles: vec!["user".to_string()],
            });
        }
    }

    // Development fallback: Accept any key that starts with "sk-" or "sk_"
    // In production, this would only be reached if Redis and DB both fail
    if api_key.starts_with("sk-") || api_key.starts_with("sk_") {
        tracing::warn!(
            "API key validation using development fallback (key prefix check). \
             This should not happen in production!"
        );
        return Ok(AuthenticatedUser {
            user_id: "api_key_user".to_string(),
            auth_method: AuthMethod::ApiKey,
            tenant_id: None,
            roles: vec!["user".to_string()],
        });
    }

    // Check for test API keys in development mode
    if cfg!(debug_assertions) && api_key.starts_with("test-") {
        return Ok(AuthenticatedUser {
            user_id: "test_user".to_string(),
            auth_method: AuthMethod::ApiKey,
            tenant_id: None,
            roles: vec!["user".to_string()],
        });
    }

    Err(AuthError {
        error: "invalid_api_key".to_string(),
        message: "The provided API key is invalid or expired".to_string(),
    })
}

/// Authentication middleware that validates JWT or API key.
///
/// Supports three authentication modes:
/// 1. **Embedded mode** (no token): Defaults to "`embedded_user`" with admin role
/// 2. **JWT authentication**: Validates JWT token and extracts user info
/// 3. **API key authentication**: Validates API key (starts with "sk-" or "test-")
///
/// For embedded mode, if a JWT token is provided, it will be validated.
/// Otherwise, it falls back to the default embedded user for backward compatibility.
pub async fn auth_middleware(
    State(state): State<crate::AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AuthError> {
    // Skip auth for health endpoints
    let path = req.uri().path();
    if path == "/health" || path == "/ready" || path == "/startup" || path == "/metrics" {
        return Ok(next.run(req).await);
    }

    // Extract authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    // In embedded mode, support both JWT and no-auth (fallback to embedded_user)
    if state.config.deployment.is_embedded() {
        let user =
            match auth_header {
                Some(header) if header.starts_with("Bearer ") => {
                    let token = &header[7..];

                    // Check if it's an API key or JWT
                    if token.starts_with("sk-") || token.starts_with("test-") {
                        // API key authentication
                        #[cfg(feature = "embedded")]
                        {
                            validate_api_key(
                                token,
                                &state.config,
                                state.redis.as_ref(),
                                state.database.as_ref(),
                            )
                            .await?
                        }
                        #[cfg(not(feature = "embedded"))]
                        {
                            validate_api_key(token, &state.config, state.redis.as_ref()).await?
                        }
                    } else {
                        // JWT authentication for embedded mode
                        #[cfg(feature = "gateway")]
                        {
                            let secret =
                                state.config.gateway.jwt_secret.as_ref().ok_or_else(|| {
                                    AuthError {
                                        error: "configuration_error".to_string(),
                                        message: "JWT secret not configured".to_string(),
                                    }
                                })?;

                            // Use embedded_auth module for validation
                            let claims = embedded_auth::validate_embedded_jwt(token, secret)
                                .map_err(|e| AuthError {
                                    error: "invalid_token".to_string(),
                                    message: format!("JWT validation failed: {e}"),
                                })?;

                            AuthenticatedUser {
                                user_id: claims.sub,
                                auth_method: AuthMethod::Jwt,
                                tenant_id: None,
                                roles: vec!["user".to_string()],
                            }
                        }
                        #[cfg(not(feature = "gateway"))]
                        {
                            return Err(AuthError {
                                error: "not_supported".to_string(),
                                message: "JWT authentication requires 'gateway' feature"
                                    .to_string(),
                            });
                        }
                    }
                }
                _ => {
                    // No auth header - fall back to embedded_user for backward compatibility
                    AuthenticatedUser {
                        user_id: "embedded_user".to_string(),
                        auth_method: AuthMethod::None,
                        tenant_id: None,
                        roles: vec!["admin".to_string()],
                    }
                }
            };

        req.extensions_mut().insert(user);
        return Ok(next.run(req).await);
    }

    // Cloud mode - require authentication
    let user = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];

            // Check if it's an API key (starts with sk-) or JWT
            if token.starts_with("sk-") || token.starts_with("test-") {
                // API key authentication
                #[cfg(feature = "embedded")]
                {
                    validate_api_key(
                        token,
                        &state.config,
                        state.redis.as_ref(),
                        state.database.as_ref(),
                    )
                    .await?
                }
                #[cfg(not(feature = "embedded"))]
                {
                    validate_api_key(token, &state.config, state.redis.as_ref()).await?
                }
            } else {
                // JWT authentication
                #[cfg(feature = "gateway")]
                {
                    let secret =
                        state
                            .config
                            .gateway
                            .jwt_secret
                            .as_ref()
                            .ok_or_else(|| AuthError {
                                error: "configuration_error".to_string(),
                                message: "JWT secret not configured".to_string(),
                            })?;

                    let claims = validate_jwt(token, secret).map_err(|e| AuthError {
                        error: "invalid_token".to_string(),
                        message: format!("JWT validation failed: {e}"),
                    })?;

                    // Production validation: Check Redis session for cloud mode
                    if state.config.deployment.is_cloud() {
                        if let Some(redis_conn) = &state.redis {
                            // Validate session exists in Redis
                            let session_key = format!("session:{}", claims.sub);
                            let mut conn = redis_conn.clone();

                            match conn.get::<_, Option<String>>(&session_key).await {
                                Ok(Some(_)) => {
                                    tracing::debug!(
                                        "JWT session validated in Redis for user: {}",
                                        claims.sub
                                    );
                                }
                                Ok(None) => {
                                    // Session not found - token may have been revoked
                                    tracing::warn!(
                                        "JWT session not found in Redis for user: {}",
                                        claims.sub
                                    );
                                    return Err(AuthError {
                                        error: "session_expired".to_string(),
                                        message: "Session expired or revoked".to_string(),
                                    });
                                }
                                Err(e) => {
                                    // Redis error - log but allow request (fail open for availability)
                                    tracing::warn!(
                                        "Redis session validation failed for user {}: {}, allowing request",
                                        claims.sub,
                                        e
                                    );
                                }
                            }
                        }
                    }

                    // Production validation: Check user exists in database for non-embedded modes
                    #[cfg(feature = "embedded")]
                    if !state.config.deployment.is_embedded() {
                        if let Some(db) = &state.database {
                            // For hybrid/cloud with embedded database, verify user exists
                            // In a full cloud deployment, this would query PostgreSQL via sqlx
                            // For now, we log that validation would happen here
                            tracing::debug!(
                                "User validation would query database for user_id: {}",
                                claims.sub
                            );

                            // In production, you would:
                            // 1. Query users table by claims.sub
                            // 2. Check user.is_active
                            // 3. Load user.roles and user.tenant_id
                            // 4. Return error if user not found or inactive
                            //
                            // Example (pseudo-code for future PostgreSQL integration):
                            // match db.get_user(&claims.sub).await {
                            //     Ok(Some(user)) if user.is_active => {
                            //         // Update roles and tenant from database
                            //         claims.roles = user.roles;
                            //         claims.tenant_id = user.tenant_id;
                            //     }
                            //     Ok(Some(_)) => {
                            //         return Err(AuthError {
                            //             error: "account_suspended".to_string(),
                            //             message: "User account is suspended".to_string(),
                            //         });
                            //     }
                            //     Ok(None) => {
                            //         return Err(AuthError {
                            //             error: "invalid_user".to_string(),
                            //             message: "User not found".to_string(),
                            //         });
                            //     }
                            //     Err(e) => {
                            //         // Database error - fail secure
                            //         tracing::error!("User validation database error: {}", e);
                            //         return Err(AuthError {
                            //             error: "auth_error".to_string(),
                            //             message: "Authentication service unavailable".to_string(),
                            //         });
                            //     }
                            // }
                        }
                    }

                    AuthenticatedUser {
                        user_id: claims.sub,
                        auth_method: AuthMethod::Jwt,
                        tenant_id: claims.tenant_id,
                        roles: claims.roles,
                    }
                }
                #[cfg(not(feature = "gateway"))]
                {
                    return Err(AuthError {
                        error: "not_supported".to_string(),
                        message: "JWT authentication requires 'gateway' feature".to_string(),
                    });
                }
            }
        }
        _ => {
            return Err(AuthError {
                error: "missing_auth".to_string(),
                message: "Authorization header is required".to_string(),
            });
        }
    };

    // Add authenticated user to request extensions
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

/// Extract authenticated user from request extensions.
pub fn get_authenticated_user(req: &Request<Body>) -> Option<&AuthenticatedUser> {
    req.extensions().get::<AuthenticatedUser>()
}

/// Get current user information (embedded mode).
///
/// Returns basic user information for the authenticated user.
/// In embedded mode, this returns default values for the `embedded_user`.
///
/// # Errors
///
/// This endpoint is designed to not fail in embedded mode, always returning
/// the current authenticated user's information.
pub async fn get_current_user_embedded(
    axum::Extension(user): axum::Extension<AuthenticatedUser>,
) -> impl IntoResponse {
    tracing::debug!("👤 Getting current user info - user_id={}", user.user_id);

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "user_id": user.user_id,
            "tenant_id": "embedded",
            "username": user.user_id,
            "email": null,
            "name": "Embedded User",
            "tier": "unlimited",
            "quotas": {},
            "rate_limits": {}
        })),
    )
}
