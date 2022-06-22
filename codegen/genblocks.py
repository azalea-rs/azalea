import lib.code.version
import lib.code.packet
import lib.code.blocks
import lib.code.utils
import lib.download
import lib.extract
import lib.utils

version_id = lib.code.version.get_version_id()


lib.extract.get_generator_mod_data(version_id, 'blockCollisionShapes')

# lib.download.get_burger()
# lib.download.get_client_jar(version_id)

# print('Generating data with burger')
# os.system(
#     f'cd {lib.utils.get_dir_location("downloads/Burger")} && python munch.py ../client-{version_id}.jar --output ../burger-{version_id}.json --toppings blockstates'
# )
# print('Ok')

mappings = lib.download.get_mappings_for_version(version_id)
block_states_burger = lib.extract.get_block_states_burger(version_id)
ordered_blocks = lib.extract.get_ordered_blocks_burger(version_id)
block_states_report = lib.extract.get_block_states_report(version_id)

lib.code.blocks.generate_blocks(
    block_states_burger, block_states_report, ordered_blocks, mappings)
