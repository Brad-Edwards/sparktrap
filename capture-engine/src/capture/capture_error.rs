// capture-engine/src/capture/capture_error.rs
/// Error types used by the capture engine.
use std::error::Error;
use std::fmt;
use std::time::SystemTime;

/// Core error type that represents all possible errors in the capture system
///
/// This error type is used to represent all possible errors that can occur in the capture system.
///
/// # Fields
/// - `kind` - The type of error that occurred
/// - `message` - Description of the error
/// - `timestamp` - The time when the error occurred
/// - `source` - The source error that caused the current error
/// - `context` - Detailed context for error reporting and debugging
#[derive(Debug)]
pub struct CaptureError {
    kind: CaptureErrorKind,
    message: String,
    timestamp: SystemTime,
    source: Option<Box<dyn Error + Send + Sync>>,
    context: Box<ErrorContext>,
}

/// Detailed context for error reporting and debugging
///
/// This struct contains detailed context information for error reporting and debugging.
///
/// # Fields
/// - `instance_id` - The ID of the instance where the error occurred
/// - `region` - The AWS region where the error occurred
/// - `vpc_id` - The ID of the VPC where the error occurred
/// - `operation` - The operation that caused the error
/// - `component` - The component that caused the error
/// - `resource_id` - The ID of the resource that caused the error
/// - `trace_id` - The trace ID for debugging
/// - `retry_count` - The number of retries attempted
/// - `severity` - The severity level of the error
#[derive(Debug, Default)]
pub struct ErrorContext {
    // Cloud context
    instance_id: Option<String>,
    region: Option<String>,
    vpc_id: Option<String>,

    // Operation context
    operation: Option<String>,
    component: Option<String>,
    resource_id: Option<String>,

    // Debug context
    trace_id: Option<String>,
    retry_count: u32,
    severity: ErrorSeverity,
}

impl ErrorContext {
    /// Returns a reference to the component name as a string slice if present, or None if not set.
    ///
    /// # Returns
    /// - `Some(&str)` - A reference to the component name
    /// - `None` - If no component name is set
    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    /// Returns the operation name if set
    ///
    /// # Returns
    /// - `Some(&str)` - A reference to the operation name
    /// - `None` - If no operation is set
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the current retry count
    ///
    /// # Returns
    /// The current retry count
    pub fn retry_count(&self) -> u32 {
        self.retry_count
    }

    /// Sets the retry count and returns self for builder pattern
    ///
    /// # Arguments
    /// * `retry_count` - The number of retries attempted
    ///
    /// # Returns
    /// A mutable reference to the ErrorContext with the retry count set
    pub fn with_retry_count(mut self, retry_count: u32) -> Self {
        self.retry_count = retry_count;
        self
    }

    /// Sets the AWS region and returns self for builder pattern
    ///
    /// # Arguments
    /// * `region` - The AWS region where the error occurred
    ///
    /// # Returns
    /// A mutable reference to the ErrorContext with the region set
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_string());
        self
    }

    /// Returns the resource ID if set
    ///
    /// # Returns
    /// - `Some(&str)` - A reference to the resource ID
    pub fn resource_id(&self) -> Option<&str> {
        self.resource_id.as_deref()
    }

    /// Sets the trace ID for debugging and returns self for builder pattern
    ///
    /// # Arguments
    /// * `trace_id` - The trace ID for the
    ///
    /// # Returns
    /// A mutable reference to the ErrorContext with the trace ID set
    pub fn with_trace_id(mut self, trace_id: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self
    }
}

/// Severity levels for errors
///
/// This enum represents the severity levels for errors in the system.
///
/// # Variants
/// - `Critical` - Critical errors that require immediate attention
/// - `Error` - Standard error conditions
/// - `Warning` - Non-critical warnings
/// - `Info` - Informational messages
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ErrorSeverity {
    Critical,
    #[default]
    Error,
    Warning,
    Info,
}

/// Type alias for commonly used Result type
///
/// This type alias is used to simplify the use of Result types in the system.
///
/// # Type Parameters
/// - `T` - The type of the successful result
///
/// # Returns
/// A Result type with the specified success type and a boxed CaptureError as the error type
pub type CaptureResult<T> = Result<T, Box<CaptureError>>;

/// Main error categories
///
/// This enum represents the main error categories in the system.
///
/// # Variants
/// - `Network` - Network-related errors
/// - `System` - System-level errors
/// - `Resource` - Resource management errors
/// - `Configuration` - Configuration errors
/// - `Runtime` - Runtime operational errors
/// - `Cloud` - Cloud-specific errors
/// - `Security` - Security-related errors
#[derive(Debug)]
pub enum CaptureErrorKind {
    // Infrastructure errors
    Network(NetworkErrorKind),
    System(SystemErrorKind),
    Resource(ResourceErrorKind),

    // Operational errors
    Configuration(ConfigErrorKind),
    Runtime(RuntimeErrorKind),

    // Cloud-specific errors
    Cloud(CloudErrorKind),

    // Security errors
    Security(SecurityErrorKind),
}

/// Network-related errors
///
/// This enum represents the different types of network-related errors that can occur in the system.
///
/// # Variants
/// - `InterfaceNotFound` - The network interface was not found
/// - `CaptureFailure` - The network capture failed
/// - `FilterError` - An error occurred while applying a filter
/// - `Timeout` - A network operation timed out
/// - `BufferOverflow` - A buffer overflow occurred
/// - `DriverError` - An error occurred in the network driver
#[derive(Debug)]
pub enum NetworkErrorKind {
    InterfaceNotFound,
    CaptureFailure,
    FilterError,
    Timeout,
    BufferOverflow,
    DriverError,
}

/// System-level errors
///
/// This enum represents the different types of system-level errors that can occur in the system.
///
/// # Variants
/// - `MemoryError` - An error occurred while managing memory
/// - `ThreadError` - An error occurred while managing threads
/// - `IoError` - An I/O operation failed
/// - `TimerError` - An error occurred while managing timers
/// - `ResourceExhausted` - A system resource was exhausted
#[derive(Debug)]
pub enum SystemErrorKind {
    MemoryError,
    ThreadError,
    IoError,
    TimerError,
    ResourceExhausted,
}

/// Resource management errors
///
/// This enum represents the different types of resource management errors that can occur in the system.
///
/// # Variants
/// - `NotAvailable` - The resource is not available
/// - `QuotaExceeded` - The resource quota was exceeded
/// - `AllocationFailed` - Resource allocation failed
/// - `InvalidState` - The resource is in an invalid state
#[derive(Debug)]
pub enum ResourceErrorKind {
    NotAvailable,
    QuotaExceeded,
    AllocationFailed,
    InvalidState,
}

/// Configuration errors
///
/// This enum represents the different types of configuration errors that can occur in the system.
///
/// # Variants
/// - `InvalidValue` - An invalid value was provided
/// - `MissingRequired` - A required value is missing
/// - `ValidationFailed` - Validation of a value failed
/// - `ParseError` - An error occurred while parsing a value
#[derive(Debug)]
pub enum ConfigErrorKind {
    InvalidValue,
    MissingRequired,
    ValidationFailed,
    ParseError,
}

/// Runtime operational errors
///
/// This enum represents the different types of runtime operational errors that can occur in the system.
///
/// # Variants
/// - `EntityNotFound` - An entity was not found
/// - `OperationFailed` - An operation failed
/// - `StateError` - An error occurred due to an invalid state
/// - `ConcurrencyError` - A concurrency error occurred
/// - `Timeout` - An operation timed out
/// - `SyncLockFailure` - A synchronization lock failed
#[derive(Debug)]
pub enum RuntimeErrorKind {
    EntityNotFound,
    OperationFailed,
    StateError,
    ConcurrencyError,
    Timeout,
    SyncLockFailure,
}

/// Cloud-specific errors
///
/// This enum represents the different types of cloud-specific errors that can occur in the system.
///
/// # Variants
/// - `VpcError` - An error occurred while managing VPCs
/// - `EniError` - An error occurred while managing ENIs
/// - `MetadataError` - An error occurred while accessing metadata
/// - `ScalingError` - An error occurred while scaling resources
/// - `ApiError` - An error occurred while calling an API
#[derive(Debug)]
pub enum CloudErrorKind {
    VpcError,
    EniError,
    MetadataError,
    ScalingError,
    ApiError,
}

/// Security-related errors
///
/// This enum represents the different types of security-related errors that can occur in the system.
///
/// # Variants
/// - `AccessDenied` - Access to a resource was denied
/// - `AuthenticationFailed` - Authentication failed
/// - `EncryptionError` - An error occurred while encrypting data
/// - `InvalidCredentials` - Invalid credentials were provided
#[derive(Debug)]
pub enum SecurityErrorKind {
    AccessDenied,
    AuthenticationFailed,
    EncryptionError,
    InvalidCredentials,
}

impl From<Box<CaptureError>> for CaptureError {
    /// Converts a boxed CaptureError to a CaptureError
    ///
    /// # Arguments
    /// * `boxed` - The boxed CaptureError to convert
    ///
    /// # Returns
    /// The unboxed CaptureError
    fn from(boxed: Box<CaptureError>) -> Self {
        *boxed
    }
}

impl CaptureError {
    /// Creates a new boxed CaptureError with the specified error kind and message
    ///
    /// # Arguments
    /// * `kind` - The type of error that occurred
    /// * `message` - Description of the error
    ///
    /// # Returns
    /// A boxed CaptureError with default context and no source error
    pub fn new(kind: CaptureErrorKind, message: &str) -> Box<Self> {
        Box::new(CaptureError {
            kind,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            source: None,
            context: Box::new(ErrorContext::default()),
        })
    }

    /// Creates a new error with cloud context
    ///
    /// # Arguments
    /// * `kind` - The type of error that occurred
    /// * `message` - Description of the error
    /// * `instance_id` - The ID of the instance where the error occurred
    /// * `region` - The AWS region where the error occurred
    ///
    /// # Returns
    /// A boxed CaptureError with cloud context and no source error
    pub fn with_cloud_context(
        kind: CaptureErrorKind,
        message: &str,
        instance_id: &str,
        region: &str,
    ) -> Box<Self> {
        let context = ErrorContext {
            instance_id: Some(instance_id.to_string()),
            region: Some(region.to_string()),
            ..Default::default()
        };

        Box::new(CaptureError {
            kind,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            source: None,
            context: Box::new(context),
        })
    }

    /// Adds source error
    ///
    /// # Arguments
    /// * `source` - The source error that caused the current error
    ///
    /// # Returns
    /// A mutable reference to the CaptureError with the source error added
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Gets the error kind
    ///
    /// # Returns
    /// The type of error that occurred
    pub fn kind(&self) -> &CaptureErrorKind {
        &self.kind
    }

    /// Gets the error context
    ///
    /// # Returns
    /// The context for the error
    pub fn context(&self) -> &ErrorContext {
        &self.context
    }

    /// Gets error severity
    ///
    /// # Returns
    /// The severity level of the error
    pub fn severity(&self) -> ErrorSeverity {
        self.context.severity
    }

    /// Builds the final error
    ///
    /// # Returns
    /// A boxed CaptureError with the specified values
    pub fn build(self) -> Box<CaptureError> {
        Box::new(self)
    }
}

impl Default for CaptureError {
    /// Creates a default CaptureError with a generic runtime error
    ///
    /// # Returns
    /// A CaptureError with default values
    fn default() -> Self {
        CaptureError {
            kind: CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
            message: String::from("Default error"),
            timestamp: SystemTime::now(),
            source: None,
            context: Box::new(ErrorContext::default()),
        }
    }
}

impl fmt::Display for CaptureError {
    /// Formats the error for display
    ///
    /// # Arguments
    /// * `f` - The formatter to write the error to
    ///
    /// # Returns
    /// A Result indicating success or failure
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (kind: {:?}, timestamp: {:?})",
            self.message, self.kind, self.timestamp
        )
    }
}

impl Error for CaptureError {
    /// Gets the source error if present
    ///
    /// # Returns
    /// The source error if present, or None if not set
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

/// Error builder errors
///
/// This enum represents the different types of errors that can occur while building errors.
///
/// # Variants
/// - `MissingKind` - The error kind is missing
/// - `MissingMessage` - The error message is missing
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("missing error kind")]
    MissingKind,
    #[error("missing error message")]
    MissingMessage,
}

/// Builder for creating errors with detailed context
///
/// This struct is used to build errors with detailed context information.
///
/// # Fields
/// - `kind` - The type of error that occurred
/// - `message` - Description of the error
/// - `context` - Detailed context for error reporting and debugging
/// - `source` - The source error that caused the current error
pub struct ErrorBuilder {
    kind: Option<CaptureErrorKind>,
    message: Option<String>,
    context: ErrorContext,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ErrorBuilder {
    /// Sets the retry count
    ///
    /// # Arguments
    /// * `retry_count` - The number of retries attempted
    ///
    /// # Returns
    /// A mutable reference to the ErrorBuilder with the retry count set
    pub fn retry_count(mut self, retry_count: u32) -> Self {
        self.context.retry_count = retry_count;
        self
    }

    /// Creates a new ErrorBuilder
    ///
    /// # Returns
    /// A new ErrorBuilder with default values
    pub fn new() -> Self {
        ErrorBuilder {
            kind: None,
            message: None,
            source: None,
            context: ErrorContext::default(),
        }
    }

    /// Sets the error kind
    ///
    /// # Arguments
    /// * `kind` - The type of error that occurred
    ///
    /// # Returns
    /// A mutable reference to the ErrorBuilder with the kind set
    pub fn kind(mut self, kind: CaptureErrorKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the error message
    ///
    /// # Arguments
    /// * `message` - Description of the error
    ///
    /// # Returns
    /// A mutable reference to the ErrorBuilder with the message set
    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the source error
    ///
    /// # Arguments
    /// * `source` - The source error that caused the current error
    ///
    /// # Returns
    /// A mutable reference to the ErrorBuilder with the source error set
    pub fn source<E: Error + Send + Sync + 'static>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Sets error severity
    ///
    /// # Arguments
    /// * `severity` - The severity level of the error
    ///
    /// # Returns
    /// A mutable reference to the ErrorBuilder with the severity set
    pub fn severity(mut self, severity: ErrorSeverity) -> Self {
        self.context.severity = severity;
        self
    }

    /// Sets the cloud context
    ///
    /// # Arguments
    /// * `instance_id` - The ID of the instance where the error occurred
    /// * `region` - The AWS region where the error occurred
    /// * `vpc_id` - The ID of the VPC where the error occurred
    ///
    /// # Returns
    /// A mutable reference to the ErrorBuilder with the cloud context set
    pub fn cloud_context(mut self, instance_id: &str, region: &str, vpc_id: &str) -> Self {
        self.context.instance_id = Some(instance_id.to_string());
        self.context.region = Some(region.to_string());
        self.context.vpc_id = Some(vpc_id.to_string());
        self
    }

    /// Builds the final error
    ///
    /// # Returns
    /// A Result containing the built CaptureError or a BuilderError if validation fails
    pub fn build(self) -> Result<CaptureError, BuilderError> {
        let kind = self.kind.ok_or(BuilderError::MissingKind)?;
        let message = self.message.ok_or(BuilderError::MissingMessage)?;

        if message.trim().is_empty() {
            return Err(BuilderError::MissingMessage);
        }

        Ok(CaptureError {
            timestamp: SystemTime::now(),
            kind,
            message,
            source: self.source,
            context: Box::new(self.context),
        })
    }
}

impl Default for ErrorBuilder {
    /// Creates a new ErrorBuilder with default values
    ///
    /// # Returns
    /// A new ErrorBuilder with default values
    fn default() -> Self {
        Self {
            kind: None,
            message: None,
            source: None,
            context: ErrorContext {
                instance_id: None,
                region: None,
                vpc_id: None,
                operation: None,
                component: None,
                resource_id: None,
                trace_id: None,
                retry_count: 0,
                severity: ErrorSeverity::Error, // Changed from Warning to Error
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::time::SystemTime;

    // CaptureError Tests
    #[test]
    fn test_capture_error_new() {
        let error = CaptureError::new(
            CaptureErrorKind::Network(NetworkErrorKind::Timeout),
            "Connection timed out",
        );
        assert!(matches!(
            error.kind(),
            CaptureErrorKind::Network(NetworkErrorKind::Timeout)
        ));
        assert_eq!(error.message, "Connection timed out");
        assert!(error.source.is_none());
        assert!(error.context.instance_id.is_none());
    }

    #[test]
    fn test_capture_error_with_cloud_context() {
        let error = CaptureError::with_cloud_context(
            CaptureErrorKind::Cloud(CloudErrorKind::VpcError),
            "VPC not found",
            "i-1234567890",
            "us-west-2",
        );
        assert!(matches!(
            error.kind(),
            CaptureErrorKind::Cloud(CloudErrorKind::VpcError)
        ));
        assert_eq!(error.message, "VPC not found");
        assert_eq!(error.context.instance_id.as_deref(), Some("i-1234567890"));
        assert_eq!(error.context.region.as_deref(), Some("us-west-2"));
    }

    #[test]
    fn test_capture_error_with_source() {
        let source_error = std::io::Error::new(std::io::ErrorKind::Other, "Source error");
        let error = CaptureError::new(
            CaptureErrorKind::System(SystemErrorKind::IoError),
            "IO operation failed",
        )
        .with_source(source_error);

        assert!(error.source().is_some());
        assert!(matches!(
            error.kind(),
            CaptureErrorKind::System(SystemErrorKind::IoError)
        ));
        assert_eq!(error.message, "IO operation failed");
    }

    #[test]
    fn test_error_builder_basic() {
        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Configuration(
                ConfigErrorKind::InvalidValue,
            ))
            .message("Invalid configuration")
            .build();

        let error = error.expect("Failed to build error");
        assert!(matches!(
            error.kind(),
            CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue)
        ));
        assert_eq!(error.message, "Invalid configuration");
        assert!(error.source.is_none());
        assert_eq!(error.context.retry_count, 0);
    }

    #[test]
    fn test_error_builder_with_cloud_context() {
        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Cloud(CloudErrorKind::EniError))
            .message("ENI attachment failed")
            .cloud_context("i-1234567890", "us-east-1", "vpc-12345")
            .build();

        let error = error.expect("Failed to build error");
        assert!(matches!(
            error.kind(),
            CaptureErrorKind::Cloud(CloudErrorKind::EniError)
        ));
        assert_eq!(error.context.instance_id.as_deref(), Some("i-1234567890"));
        assert_eq!(error.context.region.as_deref(), Some("us-east-1"));
        assert_eq!(error.context.vpc_id.as_deref(), Some("vpc-12345"));
    }

    #[test]
    fn test_error_builder_with_severity() {
        let result = ErrorBuilder::new()
            .kind(CaptureErrorKind::Security(SecurityErrorKind::AccessDenied))
            .message("Access denied")
            .severity(ErrorSeverity::Critical)
            .build();

        let error = result.expect("Failed to build error");
        assert_eq!(error.severity(), ErrorSeverity::Critical);
        assert!(matches!(
            error.kind(),
            CaptureErrorKind::Security(SecurityErrorKind::AccessDenied)
        ));
        assert_eq!(error.message, "Access denied");
    }

    #[test]
    fn test_error_context_default() {
        let context = ErrorContext::default();
        assert_eq!(context.retry_count, 0);
        assert_eq!(context.severity, ErrorSeverity::Error);
        assert!(context.instance_id.is_none());
        assert!(context.region.is_none());
        assert!(context.vpc_id.is_none());
        assert!(context.operation.is_none());
        assert!(context.component.is_none());
        assert!(context.resource_id.is_none());
        assert!(context.trace_id.is_none());
    }

    #[test]
    fn test_network_error_kinds() {
        let test_cases = vec![
            (NetworkErrorKind::InterfaceNotFound, "Interface not found"),
            (NetworkErrorKind::CaptureFailure, "Capture failed"),
            (NetworkErrorKind::FilterError, "Filter error"),
            (NetworkErrorKind::Timeout, "Network timeout"),
            (NetworkErrorKind::BufferOverflow, "Buffer overflow"),
            (NetworkErrorKind::DriverError, "Driver error"),
        ];

        for (kind, message) in test_cases {
            let error = CaptureError::new(CaptureErrorKind::Network(kind), message);
            assert!(matches!(error.kind(), CaptureErrorKind::Network(_)));
            assert_eq!(error.message, message);
        }
    }

    #[test]
    fn test_error_display_formatting() {
        let msg = "Operation failed";
        let error = CaptureError::new(
            CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
            msg,
        );
        let display_string = format!("{}", error);

        // Test only that essential information is present
        assert!(!display_string.is_empty());
        assert!(display_string.contains(msg));
    }

    #[test]
    fn test_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error = CaptureError::new(
            CaptureErrorKind::System(SystemErrorKind::IoError),
            "IO error occurred",
        )
        .with_source(io_error);

        let source = error.source().unwrap();
        assert!(source.to_string().contains("File not found"));
    }

    #[test]
    fn test_error_timestamp() {
        let error = CaptureError::new(
            CaptureErrorKind::Configuration(ConfigErrorKind::ParseError),
            "Parse error",
        );

        // Just verify timestamp is present and not in the future
        let now = SystemTime::now();
        assert!(error.timestamp <= now);
    }

    #[test]
    fn test_complex_error_scenario() {
        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Cloud(CloudErrorKind::ApiError))
            .message("API call failed")
            .cloud_context("i-abcdef", "eu-west-1", "vpc-xyz789")
            .severity(ErrorSeverity::Critical)
            .build();

        let error = error.expect("Failed to build error"); // Unwrap the Result first

        assert!(matches!(
            error.kind(),
            CaptureErrorKind::Cloud(CloudErrorKind::ApiError)
        ));
        assert_eq!(error.context.severity, ErrorSeverity::Critical); // Access severity through context
        assert_eq!(error.context.instance_id.as_deref(), Some("i-abcdef"));
        assert_eq!(error.context.region.as_deref(), Some("eu-west-1"));
        assert_eq!(error.context.vpc_id.as_deref(), Some("vpc-xyz789"));
        assert_eq!(error.message, "API call failed");
    }

    #[test]
    fn test_error_chaining() {
        let base_error = std::io::Error::new(std::io::ErrorKind::Other, "Base error");

        let mid_error = CaptureError::new(
            CaptureErrorKind::System(SystemErrorKind::IoError),
            "Middle error",
        )
        .with_source(base_error);

        let top_error = CaptureError::new(
            CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
            "Top error",
        )
        .with_source(mid_error);

        let mut error_chain = vec![];
        let mut current_error = top_error.source();
        while let Some(error) = current_error {
            error_chain.push(error.to_string());
            current_error = error.source();
        }

        assert_eq!(error_chain.len(), 2);
        assert!(error_chain[0].contains("Middle error"));
        assert!(error_chain[1].contains("Base error"));
    }

    #[test]
    fn test_error_default() {
        let error = CaptureError::default();
        assert!(matches!(
            error.kind,
            CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed)
        ));
        assert!(!error.message.is_empty());
        assert!(error.source.is_none());
        assert_eq!(error.context.retry_count, 0);
        assert_eq!(error.context.severity, ErrorSeverity::Error); // Updated assertion
    }

    #[test]
    fn test_error_builder_default() {
        let builder = ErrorBuilder::default();
        assert!(builder.kind.is_none());
        assert!(builder.message.is_none());
        assert!(builder.source.is_none());
        assert_eq!(builder.context.retry_count, 0);
        assert_eq!(builder.context.severity, ErrorSeverity::Error); // Updated assertion
    }

    #[test]
    fn test_error_builder_missing_kind() {
        let builder = ErrorBuilder::new();
        let result = builder.message("Test message").build();
        assert!(matches!(result, Err(BuilderError::MissingKind)));
    }

    #[test]
    fn test_error_builder_missing_message() {
        let builder = ErrorBuilder::new();
        let error = builder
            .kind(CaptureErrorKind::Configuration(
                ConfigErrorKind::InvalidValue,
            ))
            .build();

        assert!(matches!(error, Err(BuilderError::MissingMessage)));
    }

    #[test]
    fn test_all_network_error_variants() {
        let variants = vec![
            NetworkErrorKind::InterfaceNotFound,
            NetworkErrorKind::CaptureFailure,
            NetworkErrorKind::FilterError,
            NetworkErrorKind::Timeout,
            NetworkErrorKind::BufferOverflow,
            NetworkErrorKind::DriverError,
        ];

        for variant in variants {
            let error = CaptureError::new(CaptureErrorKind::Network(variant), "test message");
            assert!(matches!(error.kind(), CaptureErrorKind::Network(_)));
        }
    }

    #[test]
    fn test_error_builder_validation() {
        let result = ErrorBuilder::new().message("Test message").build();

        assert!(matches!(result, Err(BuilderError::MissingKind)));
    }

    #[test]
    fn test_error_context_with_max_retries() {
        let mut context = ErrorContext::default();
        context.retry_count = u32::MAX;
        assert_eq!(context.retry_count, u32::MAX);
    }

    #[test]
    fn test_error_creation_with_invalid_data() {
        let result = CaptureError::new(
            CaptureErrorKind::Security(SecurityErrorKind::AccessDenied),
            "", // empty message
        );
        assert_eq!(result.message, ""); // verify it handles empty messages
    }

    #[test]
    fn test_error_context_builder() {
        let context = ErrorContext::default()
            .with_retry_count(3)
            .with_region("us-east-1")
            .with_trace_id("trace-123");

        assert_eq!(context.retry_count, 3);
        assert_eq!(context.region.unwrap(), "us-east-1");
        assert_eq!(context.trace_id.unwrap(), "trace-123");
    }

    #[test]
    fn test_error_builder_with_empty_message() {
        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed))
            .message("")
            .build();
        assert!(matches!(error, Err(BuilderError::MissingMessage)));
    }

    #[test]
    fn test_error_builder_with_excessive_retries() {
        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed))
            .message("Test message")
            .retry_count(u32::MAX)
            .build();
        assert!(matches!(error, Ok(_)));
        let error = error.unwrap();
        assert_eq!(error.context.retry_count, u32::MAX);
    }

    #[test]
    fn test_error_context_boundary_conditions() {
        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed))
            .message("Test message")
            .retry_count(0)
            .build()
            .unwrap();
        assert_eq!(error.context.retry_count, 0);

        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed))
            .message("Test message")
            .retry_count(1)
            .build()
            .unwrap();
        assert_eq!(error.context.retry_count, 1);
    }

    #[test]
    fn test_builder_method_chaining() {
        let error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Configuration(
                ConfigErrorKind::InvalidValue,
            ))
            .message("Test message")
            .severity(ErrorSeverity::Warning)
            .retry_count(3)
            .source(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Source error",
            ))
            .build()
            .unwrap();

        assert!(matches!(error.kind(), CaptureErrorKind::Configuration(_)));
        assert_eq!(error.message, "Test message");
        assert_eq!(error.context.severity, ErrorSeverity::Warning);
        assert_eq!(error.context.retry_count, 3);
        assert!(error.source.is_some());
    }

    #[test]
    fn test_comprehensive_default_implementation() {
        let error_context = ErrorContext::default();
        assert_eq!(error_context.retry_count, 0);
        assert_eq!(error_context.severity, ErrorSeverity::Error);

        let builder = ErrorBuilder::default();
        assert!(builder.kind.is_none());
        assert!(builder.message.is_none());
        assert!(builder.source.is_none());
        assert_eq!(builder.context.retry_count, 0);
        assert_eq!(builder.context.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_builder_validation_requirements() {
        // Missing kind
        let result = ErrorBuilder::new().message("Test message").build();
        assert!(matches!(result, Err(BuilderError::MissingKind)));

        // Missing message
        let result = ErrorBuilder::new()
            .kind(CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed))
            .build();
        assert!(matches!(result, Err(BuilderError::MissingMessage)));

        // Valid minimal build
        let result = ErrorBuilder::new()
            .kind(CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed))
            .message("Test message")
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_severity_transitions() {
        let mut error = ErrorBuilder::new()
            .kind(CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed))
            .message("Test message")
            .severity(ErrorSeverity::Warning)
            .build()
            .unwrap();

        assert_eq!(error.context.severity, ErrorSeverity::Warning);
        error.context.severity = ErrorSeverity::Error;
        assert_eq!(error.context.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_error_source_chain() {
        let source_error = std::io::Error::new(std::io::ErrorKind::Other, "Inner error");
        let wrapped_error = CaptureError::new(
            CaptureErrorKind::System(SystemErrorKind::IoError),
            "Middle error",
        )
        .with_source(source_error);

        let final_error = CaptureError::new(
            CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
            "Outer error",
        )
        .with_source(wrapped_error);

        assert!(final_error.source().is_some());
        assert!(final_error.source().unwrap().source().is_some());
    }
}
