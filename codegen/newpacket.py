import lib.code.version
import lib.code.packet
import lib.code.utils
import lib.download
import lib.extract
import sys

version_id = lib.code.version.get_version_id()

mappings = lib.download.get_mappings_for_version(version_id)
burger_data = lib.extract.get_burger_data_for_version(version_id)

burger_packets_data = burger_data[0]['packets']['packet']
packet_id, direction, state = int(sys.argv[1]), sys.argv[2], sys.argv[3]
print(
    f'Generating code for packet id: {packet_id} with direction {direction} and state {state}')
lib.code.packet.generate_packet(burger_packets_data, mappings,
                                packet_id, direction, state)

lib.code.utils.fmt()

print('Done!')
