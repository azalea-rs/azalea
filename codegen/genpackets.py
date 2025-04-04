import lib.code.version
import lib.code.packet
import lib.code.utils
import lib.download
import lib.extract

def generate():
    version_id = lib.code.version.get_version_id()
    packets_report = lib.extract.get_packets_report(version_id)

    lib.code.packet.set_packets(packets_report)

    lib.code.utils.fmt()

    print('Done!')

if __name__ == '__main__':
    generate()
