# conftest.py
import pytest
from unittest.mock import Mock
from scapy.all import IP, TCP, UDP, Ether, Raw

@pytest.fixture
def mock_queue_manager():
    return Mock()

@pytest.fixture
def mock_interface_list():
    return ['eth0', 'lo', 'wlan0']

@pytest.fixture
def sample_packets():
    return {
        'tcp': Ether()/IP(src="192.168.1.1", dst="192.168.1.2")/TCP(sport=80, dport=443),
        'udp': Ether()/IP(src="192.168.1.1", dst="192.168.1.2")/UDP(sport=53, dport=53),
        'malformed': Raw(load=b'malformed packet data'),
        'oversized': Ether()/IP()/Raw(load='X' * 65536)
    }