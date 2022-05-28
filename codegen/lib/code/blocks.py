BLOCKS_RS_DIR = '../azalea-blocks/src/blocks.rs'


def generate_blocks(blocks: dict):
    with open(BLOCKS_RS_DIR, 'r') as f:
        existing_code = f.read().splitlines()

    new_make_block_states_macro_code = []
    new_make_block_states_macro_code.append('make_block_states! {')

    properties = {}
    for block_name, block_data in blocks.items():
        block_properties = block_data['properties']

        properties.update(block_properties)

    print(properties)
