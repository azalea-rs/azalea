from lib import download, packetcodegen  # type: ignore
import sys
import os

mappings = download.get_mappings_for_version('1.18.2')
burger_data = download.get_burger_data_for_version('1.18.2')

burger_packets_data = burger_data[0]['packets']['packet']
packet_id, direction, state = int(sys.argv[1]), sys.argv[2], sys.argv[3]
print(
    f'Generating code for packet id: {packet_id} with direction {direction} and state {state}')
packetcodegen.generate(burger_packets_data, mappings,
                       packet_id, direction, state)

os.system('cd .. && cargo fmt')

print('Done!')
