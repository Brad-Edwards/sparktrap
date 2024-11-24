# src/common/messages.py
class ErrorMessages:
    """Error messages used throughout the application."""

    NO_QUEUE_MANAGER = "Queue manager cannot be None"
    INVALID_QM_TYPE = "Queue manager must implement required interface"
    INVALID_INTERFACE_TYPE = "Interface must be a string or None"
    INVALID_INTERFACE = "Interface {interface} not found. Available interfaces: {available}"
    NO_INTERFACES = "No network interfaces available"
    PERMISSION_ERROR = "Unable to access network interfaces: {error}"
