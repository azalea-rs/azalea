import lib.code.version
import lib.code.shapes
import lib.code.packet
import lib.code.blocks
import lib.code.utils
import lib.download
import lib.extract
import lib.utils

def generate(version_id):
    pumpkin_block_datas = lib.extract.get_pumpkin_data(version_id, 'blocks')
    burger_data = lib.extract.get_burger_data_for_version(version_id)

    block_states_report = lib.extract.get_block_states_report(version_id)
    registries = lib.extract.get_registries_report(version_id)
    ordered_blocks = lib.code.blocks.get_ordered_blocks(registries)

    lib.code.blocks.generate_blocks(block_states_report, pumpkin_block_datas, ordered_blocks, burger_data)
    lib.code.shapes.generate_block_shapes(pumpkin_block_datas, block_states_report)

    lib.code.utils.fmt()

    print('Done!')


if __name__ == '__main__':
    generate(lib.code.version.get_version_id())
