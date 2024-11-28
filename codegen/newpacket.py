import lib.code.version
import lib.code.packet
import lib.code.utils
import lib.download
import lib.extract
import sys

def generate():
    version_id = lib.code.version.get_version_id()

    packets_report = lib.extract.get_packets_report(version_id)

    packet_id, direction, state = sys.argv[1], sys.argv[2], sys.argv[3]
    print(
        f'Generating code for packet id: {packet_id} with direction {direction} and state {state}')
    lib.code.packet.generate_packet(packets_report, packet_id, direction, state)
    lib.code.packet.set_packets(packets_report)

    lib.code.utils.fmt()

    print('Done!')

if __name__ == '__main__':
    generate()
