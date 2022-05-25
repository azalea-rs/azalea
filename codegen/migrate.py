import lib.code.utils
import lib.code.version
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

old_packet_ids = {}
new_packet_ids = {}

for packet in old_packet_list:
    assert packet['class'].endswith('.class')
    packet_name = old_mappings.get_class(packet['class'][:-6])
    old_packet_ids[packet_name] = packet['id']
for packet in new_packet_list:
    assert packet['class'].endswith('.class')
    packet_name = new_mappings.get_class(packet['class'][:-6])
    new_packet_ids[packet_name] = packet['id']

# find packets that changed ids
for packet_name in old_packet_ids:
    if packet_name in new_packet_ids:
        if old_packet_ids[packet_name] != new_packet_ids[packet_name]:
            print(packet_name, 'id changed from',
                  old_packet_ids[packet_name], 'to', new_packet_ids[packet_name])

print()

# find removed packets
for packet_name in old_packet_ids:
    if packet_name not in new_packet_ids:
        print(packet_name, 'removed')

print()

# find added packets
for packet_name in new_packet_ids:
    if packet_name not in old_packet_ids:
        print(packet_name, 'added')

lib.code.utils.fmt()

print('Done!')
