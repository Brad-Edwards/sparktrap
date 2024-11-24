import pytest
from unittest.mock import Mock, patch
from src.common.messages import ErrorMessages
from src.ingestion.packet_ingester import PacketIngester

class TestPacketIngesterInit:
    @pytest.fixture
    def valid_qm(self):
        return Mock()

    def test_valid_inputs(self, valid_qm):
        """Happy path - valid interface and queue manager"""
        with patch('scapy.all.get_if_list', return_value=['eth0']):
            ingester = PacketIngester('eth0', valid_qm)
            assert ingester.interface == 'eth0'
            assert ingester.queue_manager == valid_qm

    def test_interface_none(self, valid_qm):
        """None interface should default to conf.iface"""
        with patch('scapy.all.get_if_list', return_value=['eth0']):
            with patch('scapy.all.conf') as mock_conf:
                mock_conf.iface = 'eth0'
                ingester = PacketIngester(None, valid_qm)
                assert ingester.interface == 'eth0'

    def test_interface_invalid_types(self, valid_qm):
        """Non-string interface types should raise TypeError"""
        invalid_interfaces = [123, [], {}, True, object()]
        with patch('scapy.all.get_if_list', return_value=['eth0']):
            for invalid_if in invalid_interfaces:
                with pytest.raises(TypeError, match=ErrorMessages.INVALID_INTERFACE_TYPE):
                    PacketIngester(invalid_if, valid_qm)

    def test_interface_invalid_values(self, valid_qm):
        """Invalid interface string values"""
        invalid_interfaces = ['', 'nonexistent', 'eth0' * 100, '!@#$%^&*()', ' ', '\x00']
        
        # Patch where the function is used, not where it's defined
        with patch('src.ingestion.packet_ingester.get_if_list', return_value=['eth0']):
            for invalid_if in invalid_interfaces:
                with pytest.raises(ValueError) as exc_info:
                    PacketIngester(invalid_if, valid_qm)
                expected_msg = ErrorMessages.INVALID_INTERFACE.format(
                    interface=invalid_if, 
                    available=['eth0']
                )
                assert str(exc_info.value) == expected_msg

    def test_queue_manager_none(self):
        """None queue_manager should raise ValueError"""
        with patch('scapy.all.get_if_list', return_value=['eth0']):
            with pytest.raises(ValueError, match=ErrorMessages.NO_QUEUE_MANAGER):
                PacketIngester('eth0', None)
