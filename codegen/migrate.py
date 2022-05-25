from codegen.lib.utils import PacketIdentifier, group_packets
import lib.code.utils
import lib.code.version
import lib.code.packet
import lib.download
import sys
import os

old_version_id = lib.code.version.get_version_id()
old_mappings = lib.download.get_mappings_for_version(old_version_id)
old_burger_data = lib.download.get_burger_data_for_version(old_version_id)
old_packet_list = list(old_burger_data[0]['packets']['packet'].values())

new_version_id = sys.argv[1]
new_mappings = lib.download.get_mappings_for_version(new_version_id)
new_burger_data = lib.download.get_burger_data_for_version(new_version_id)
new_packet_list = list(new_burger_data[0]['packets']['packet'].values())


old_packets: dict[PacketIdentifier, str] = {}
new_packets: dict[PacketIdentifier, str] = {}

for packet in old_packet_list:
    assert packet['class'].endswith('.class')
    packet_name = old_mappings.get_class(packet['class'][:-6])
    old_packets[PacketIdentifier(
        packet['id'], packet['direction'], packet['state'])] = packet_name
for packet in new_packet_list:
    assert packet['class'].endswith('.class')
    packet_name = new_mappings.get_class(packet['class'][:-6])
    new_packets[PacketIdentifier(
        packet['id'], packet['direction'], packet['state'])] = packet_name


# find removed packets
removed_packets: list[PacketIdentifier] = []
for packet in old_packets:
    if packet not in new_packets:
        removed_packets.append(packet)
for (direction, state), packets in group_packets(removed_packets).items():
    lib.code.packet.remove_packet_ids(packets, direction, state)

print()

# find packets that changed ids
changed_packets: dict[PacketIdentifier, int] = {}
for old_packet, old_packet_name in old_packets.items():
    for new_packet, new_packet_name in new_packets.items():
        if old_packet == new_packet and old_packet.packet_id != new_packet.packet_id:
            changed_packets[old_packet] = new_packet.packet_id
for (direction, state), packets in group_packets(list(changed_packets.keys())).items():
    lib.code.packet.remove_packet_ids(packets, direction, state)


print()

# find added packets
added_packets: list[PacketIdentifier] = []
for packet in new_packets:
    if packet not in old_packets:
        added_packets.append(packet)
for packet in added_packets:
    lib.code.packet.generate_packet(
        new_burger_data, new_mappings, packet.packet_id, packet.direction, packet.state)
lib.code.utils.fmt()

print('Done!')
