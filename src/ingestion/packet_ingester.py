import logging
from typing import Optional

from scapy.all import conf, get_if_list  # Add scapy to requirements.txt

from src.common.messages import ErrorMessages


class PacketIngester:
    """Packet capture and ingestion handler."""

    def __init__(self, interface: Optional[str], queue_manager):
        self.logger = logging.getLogger(__name__)

        # First check interfaces availability
        try:
            available_interfaces = get_if_list()
        except PermissionError as err:
            raise PermissionError(ErrorMessages.PERMISSION_ERROR.format(error=str(err))) from err

        if not available_interfaces:
            raise RuntimeError(ErrorMessages.NO_INTERFACES)

        # Then validate interface type
        if interface is not None and not isinstance(interface, str):
            raise TypeError(ErrorMessages.INVALID_INTERFACE_TYPE)

        # Validate queue manager
        if queue_manager is None:
            raise ValueError(ErrorMessages.NO_QUEUE_MANAGER)

        # Set interface
        if interface is not None:
            if interface not in available_interfaces:
                raise ValueError(
                    ErrorMessages.INVALID_INTERFACE.format(interface=interface, available=available_interfaces)
                )
            self.interface = interface
        else:
            self.interface = conf.iface

        self.queue_manager = queue_manager
        self.logger.info("Initializing packet ingester on interface %s", self.interface)

    def start_capture(self, packet_count=1):
        """Start capturing packets."""
        raise NotImplementedError

    def stop_capture(self):
        """Stop capturing packets."""
        raise NotImplementedError

    def add_filter(self):
        """Add a packet filter."""
        raise NotImplementedError

    def remove_filter(self):
        """Remove a packet filter."""
        raise NotImplementedError

    def get_metrics(self):
        """Get capture metrics."""
        raise NotImplementedError

    def write_to_queue(self, packet):
        """Write packet to queue."""
        raise NotImplementedError


if __name__ == "__main__":
    # Remove the direct instantiation or add required parameters
    raise NotImplementedError("Direct script execution not supported")
