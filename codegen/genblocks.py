import lib.code.version
import lib.code.shapes
import lib.code.packet
import lib.code.blocks
import lib.code.utils
import lib.download
import lib.extract
import lib.utils

def generate():
    version_id = lib.code.version.get_version_id()

    # TODO: pixlyzer is broken so we use old data
    shape_datas = lib.extract.get_pixlyzer_data(
        '1.20.3-pre4', 'shapes')
    pixlyzer_block_datas = lib.extract.get_pixlyzer_data(
        '1.20.3-pre4', 'blocks')


    mappings = lib.download.get_mappings_for_version(version_id)
    block_states_report = lib.extract.get_block_states_report(version_id)

    registries = lib.extract.get_registries_report(version_id)
    ordered_blocks = get_ordered_blocks(registries)

    lib.code.blocks.generate_blocks(
        block_states_report, pixlyzer_block_datas, ordered_blocks, mappings)

    lib.code.shapes.generate_block_shapes(
        pixlyzer_block_datas, shape_datas['shapes'], shape_datas['aabbs'], block_states_report)

    lib.code.utils.fmt()

    print('Done!')

def get_ordered_blocks(registries_report: dict[str, dict]) -> list[str]:
    '''
    Returns a list of block ids (like ['air', 'stone', ...]) ordered by their protocol id.
    '''
    blocks_registry = registries_report['minecraft:block']

    blocks_to_ids = {} 
    for block_id, value in blocks_registry['entries'].items():
        prefix = 'minecraft:'
        assert block_id.startswith(prefix)
        block_id = block_id[len(prefix):]
        protocol_id = value['protocol_id']
        blocks_to_ids[block_id] = protocol_id
    
    ordered_blocks = []
    for block_id in sorted(blocks_to_ids, key=blocks_to_ids.get):
        ordered_blocks.append(block_id)
    return ordered_blocks

generate()
