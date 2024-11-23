# PackIngester.py

from scapy.all import conf, get_if_list, sniff


class PacketIngester:
    def __init__(self, interface=None):
        if interface is None:
            interface = conf.iface

        available_interfaces = get_if_list()
        if interface not in available_interfaces:
            raise ValueError(f"Interface {interface} not found. Available interfaces: {available_interfaces}")

        self.interface = interface
        conf.verb = 0
        conf.sniff_promisc = True

    def start_capture(self, packet_count=10):
        print(f"Starting capture on interface {self.interface}")
        packets = sniff(iface=self.interface, count=packet_count)
        print(f"Captured {len(packets)} packets")
        for packet in packets:
            print(packet.summary())

    def stop_capture(self):
        raise NotImplementedError

    def add_filter(self):
        raise NotImplementedError

    def remove_filter(self):
        raise NotImplementedError

    def get_stats(self):
        raise NotImplementedError


if __name__ == "__main__":
    pi = PacketIngester()
    pi.start_capture()
