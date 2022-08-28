from lib.code.packet import fix_state
from lib.utils import PacketIdentifier, group_packets
import lib.code.version
import lib.code.blocks
import lib.code.packet
import lib.code.utils
import lib.download
import lib.extract
import sys

lib.download.clear_version_cache()

if len(sys.argv) == 1:
    print('\033[91mYou must provide a version to migrate to.\033[m')
    version_manifest = lib.download.get_version_manifest()
    newest_version = version_manifest['latest']['snapshot']
    print(f'Hint: newest version is \033[1m{newest_version}\033[m')
    exit()


old_version_id = lib.code.version.get_version_id()
old_mappings = lib.download.get_mappings_for_version(old_version_id)
old_burger_data = lib.extract.get_burger_data_for_version(old_version_id)
old_packet_list = list(old_burger_data[0]['packets']['packet'].values())

new_version_id = sys.argv[1]
new_mappings = lib.download.get_mappings_for_version(new_version_id)
new_burger_data = lib.extract.get_burger_data_for_version(new_version_id)
new_packet_list = list(new_burger_data[0]['packets']['packet'].values())


old_packets: dict[PacketIdentifier, str] = {}
old_packets_data: dict[PacketIdentifier, dict] = {}
new_packets: dict[PacketIdentifier, str] = {}
new_packets_data: dict[PacketIdentifier, dict] = {}

for packet in old_packet_list:
    assert packet['class'].endswith('.class')
    packet_name = old_mappings.get_class(packet['class'][:-6])
    packet_ident = PacketIdentifier(
        packet['id'], packet['direction'].lower(), fix_state(packet['state']))
    old_packets[packet_ident] = packet_name
    old_packets_data[packet_ident] = packet
for packet in new_packet_list:
    assert packet['class'].endswith('.class')
    packet_name = new_mappings.get_class(packet['class'][:-6])
    packet_ident = PacketIdentifier(
        packet['id'], packet['direction'].lower(), fix_state(packet['state']))
    new_packets[packet_ident] = packet_name
    new_packets_data[packet_ident] = packet

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

# find added/changed packets
added_or_changed_packets: list[PacketIdentifier] = []
for new_packet, packet_name in new_packets.items():
    old_packet = None
    for old_packet_tmp, old_packet_name in old_packets.items():
        if old_packet_name == packet_name:
            old_packet = old_packet_tmp
            break

    if packet_name not in old_packets.values():
        added_or_changed_packets.append(new_packet)
        print('Added packet:', new_packet, packet_name)
    elif old_packet and not lib.code.packet.are_packet_instructions_identical(new_packets_data[new_packet].get('instructions'), old_packets_data[old_packet].get('instructions')):
        added_or_changed_packets.append(new_packet)
        print('Changed packet:', new_packet, packet_name)
for packet in added_or_changed_packets:
    lib.code.packet.generate_packet(
        new_burger_data[0]['packets']['packet'], new_mappings, packet.packet_id, packet.direction, packet.state)

lib.code.version.set_protocol_version(
    new_burger_data[0]['version']['protocol'])
lib.code.version.set_version_id(new_version_id)

print('Updated protocol!')


old_ordered_blocks = lib.extract.get_ordered_blocks_burger(old_version_id)
new_ordered_blocks = lib.extract.get_ordered_blocks_burger(new_version_id)
if old_ordered_blocks != new_ordered_blocks:
    print('Blocks changed, updating...')
    block_states_burger = lib.extract.get_block_states_burger(new_version_id)
    block_states_report = lib.extract.get_block_states_report(new_version_id)

    lib.code.blocks.generate_blocks(
        block_states_burger, block_states_report, old_ordered_blocks, new_mappings)


lib.code.utils.fmt()

print('Done!')
