import lib.code.version
import lib.code.packet
import lib.code.blocks
import lib.code.utils
import lib.download
import lib.extract
import lib.utils
import sys
import os

version_id = lib.code.version.get_version_id()

lib.download.get_burger()
lib.download.get_client_jar(version_id)

print('Generating data with burger')
os.system(
    f'cd {lib.utils.get_dir_location("downloads/Burger")} && python munch.py ../client-{version_id}.jar --output ../burger-{version_id}.json --toppings blockstates'
)
print('Ok')

mappings = lib.download.get_mappings_for_version(version_id)
block_states_data = lib.extract.get_block_states(version_id)

lib.code.blocks.generate_blocks(block_states_data, mappings)
