import lib.code.version
import lib.code.packet
import lib.code.blocks
import lib.code.utils
import lib.download
import lib.extract
import sys

version_id = lib.code.version.get_version_id()

block_states_data = lib.extract.get_block_states(version_id)

lib.code.blocks.generate_blocks(block_states_data)
