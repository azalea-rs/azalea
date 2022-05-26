from lib.code.packet import fix_state
from lib.utils import PacketIdentifier, group_packets
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
        packet['id'], packet['direction'].lower(), fix_state(packet['state']))] = packet_name
for packet in new_packet_list:
    assert packet['class'].endswith('.class')
    packet_name = new_mappings.get_class(packet['class'][:-6])
    new_packets[PacketIdentifier(
        packet['id'], packet['direction'].lower(), fix_state(packet['state']))] = packet_name

# find removed packets
removed_packets: list[PacketIdentifier] = []
for packet, packet_name in old_packets.items():
    if packet_name not in new_packets.values():
        removed_packets.append(packet)
        print('Removed packet:', packet, packet_name)
for (direction, state), packets in group_packets(removed_packets).items():
    lib.code.packet.remove_packet_ids(packets, direction, state)

print()

# find packets that changed ids
changed_packets: dict[PacketIdentifier, int] = {}
for old_packet, old_packet_name in old_packets.items():
    for new_packet, new_packet_name in new_packets.items():
        if old_packet_name == new_packet_name and old_packet.direction == new_packet.direction and old_packet.state == new_packet.state and old_packet.packet_id != new_packet.packet_id:
            changed_packets[old_packet] = new_packet.packet_id
            print('Changed packet id:', old_packet, '->',
                  new_packet, f'({new_packet_name})')
            break
for (direction, state), packets in group_packets(list(changed_packets.keys())).items():
    id_map: dict[int, int] = {}
    for old_packet_id in packets:
        new_packet_id = changed_packets[PacketIdentifier(
            old_packet_id, direction, state)]
        id_map[old_packet_id] = new_packet_id
    lib.code.packet.change_packet_ids(id_map, direction, state)


print()

# find added packets
added_packets: list[PacketIdentifier] = []
for packet, packet_name in new_packets.items():
    if packet_name not in old_packets.values():
        added_packets.append(packet)
        print('Added packet:', packet, packet_name)
for packet in added_packets:
    lib.code.packet.generate_packet(
        new_burger_data[0]['packets']['packet'], new_mappings, packet.packet_id, packet.direction, packet.state)
lib.code.utils.fmt()

print('Done!')
