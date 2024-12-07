// security/traits.rs
/// `SecurityManager` handles zero-trust authentication, authorization, and policy enforcement.
use async_trait::async_trait;
use std::collections::HashMap;
use std::net::IpAddr;

use crate::traits::{Error, EventHandler, HealthCheck, Lifecycle};

/// Events related to security
#[derive(Debug)]
pub enum SecurityEvent {
    AuthenticationAttempt(AuthAttempt),
    AuthorizationRequest(AuthRequest),
    PolicyUpdate(PolicyUpdate),
    CertificateEvent(CertificateEvent),
    AccessDenied(AccessDeniedEvent),
    SecurityAlert(SecurityAlert),
}

/// Trait for security management with zero trust considerations
#[async_trait]
pub trait SecurityManager:
    Lifecycle + EventHandler<SecurityEvent> + HealthCheck + Send + Sync
{
    /// Authenticates a request, ensuring no implicit trust
    async fn authenticate(&self, request: AuthRequest) -> Result<AuthToken, Error>;

    /// Authorizes an action, enforcing least privilege access
    async fn authorize(&self, token: &AuthToken, action: &Action) -> Result<AuthzDecision, Error>;

    /// Validates the identity of components or services
    async fn validate_identity(&self, identity: &Identity) -> Result<(), Error>;

    /// Continuously monitors and verifies security status
    async fn continuous_verification(&self) -> Result<(), Error>;

    /// Applies security policies and updates
    async fn apply_policy(&mut self, policy: SecurityPolicy) -> Result<(), Error>;

    /// Handles security alerts and potential breaches
    async fn handle_security_alert(&mut self, alert: SecurityAlert) -> Result<(), Error>;

    /// Rotates encryption keys or certificates
    async fn rotate_keys(&mut self) -> Result<(), Error>;
}

/// Represents an authentication attempt
#[derive(Debug, Clone)]
pub struct AuthAttempt {
    pub identity: Identity,
    pub credentials: Credentials,
    pub context: AuthContext,
    pub timestamp: u64,
}

/// Identity of a user or service
#[derive(Debug, Clone)]
pub struct Identity {
    pub id: String,
    pub attributes: HashMap<String, String>,
}

/// Request for authentication, with zero trust in mind
#[derive(Debug, Clone)]
pub struct AuthRequest {
    pub identity: Identity,
    pub credentials: Credentials,
    pub context: AuthContext,
}

/// Represents credentials for authentication
#[derive(Debug, Clone)]
pub enum Credentials {
    Password(String),
    Token(String),
    Certificate(Vec<u8>),
    ApiKey(String),
}

/// Context for authentication or authorization
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub source_ip: IpAddr,
    pub user_agent: Option<String>,
    pub device_info: Option<String>,
}

/// Authentication token, including expiry and scope
#[derive(Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: u64,
    pub scopes: Vec<String>,
    pub issued_at: u64,
    pub issuer: String,
}

/// Represents an action to be authorized
#[derive(Debug, Clone)]
pub struct Action {
    pub resource: String,
    pub operation: String,
    pub context: HashMap<String, String>,
}

/// Decision for an authorization request, with explicit deny reasons
#[derive(Debug, Clone)]
pub enum AuthzDecision {
    Allow,
    Deny { reason: String },
}

/// Security policy for the system, supporting micro-segmentation and least privilege
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub rules: Vec<PolicyRule>,
}

/// A rule within a security policy
#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub identity: IdentityMatch,
    pub action: ActionMatch,
    pub effect: PolicyEffect,
}

/// Criteria for matching an identity in a policy rule
#[derive(Debug, Clone)]
pub struct IdentityMatch {
    pub id: Option<String>,
    pub attributes: HashMap<String, String>,
}

/// Criteria for matching an action in a policy rule
#[derive(Debug, Clone)]
pub struct ActionMatch {
    pub resource: Option<String>,
    pub operation: Option<String>,
}

/// Effect of a policy rule
#[derive(Debug, Clone)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// Represents a security alert or potential breach
#[derive(Debug, Clone)]
pub struct SecurityAlert {
    pub alert_id: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub detected_at: u64,
    pub source: String,
    pub additional_info: HashMap<String, String>,
}

/// Severity levels for security alerts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Event when access is denied to a resource
#[derive(Debug)]
pub struct AccessDeniedEvent {
    pub identity: Identity,
    pub action: Action,
    pub reason: String,
    pub timestamp: u64,
}

/// Event related to policy updates
#[derive(Debug, Clone)]
pub struct PolicyUpdate {
    pub policy: SecurityPolicy,
    pub updated_at: u64,
    pub updated_by: String,
}

/// Event related to certificates
#[derive(Debug, Clone)]
pub struct CertificateEvent {
    pub cert_id: String,
    pub event_type: CertificateEventType,
    pub timestamp: u64,
}

/// Types of certificate events
#[derive(Debug, Clone)]
pub enum CertificateEventType {
    Created,
    Updated,
    Revoked,
}
