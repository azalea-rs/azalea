import lib.code.version
import lib.code.shapes
import lib.code.packet
import lib.code.blocks
import lib.code.utils
import lib.download
import lib.extract
import lib.utils

def generate(version_id):
    # TODO: pixlyzer is broken so we use old data
    shape_datas = lib.extract.get_pixlyzer_data(
        '1.20.3-pre4', 'shapes')
    pixlyzer_block_datas = lib.extract.get_pixlyzer_data(
        '1.20.3-pre4', 'blocks')

    block_states_report = lib.extract.get_block_states_report(version_id)
    registries = lib.extract.get_registries_report(version_id)
    ordered_blocks = lib.code.blocks.get_ordered_blocks(registries)

    lib.code.blocks.generate_blocks(
        block_states_report, pixlyzer_block_datas, ordered_blocks)

    lib.code.shapes.generate_block_shapes(
        pixlyzer_block_datas, shape_datas['shapes'], shape_datas['aabbs'], block_states_report)

    lib.code.utils.fmt()

    print('Done!')


if __name__ == '__main__':
    generate(lib.code.version.get_version_id())
