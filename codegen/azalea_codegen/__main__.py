import logging

from azalea_codegen.download_and_extract.burger import get_burger_data
from azalea_codegen.download_and_extract.data_generators import get_report
from azalea_codegen.download_and_extract.launcher_meta import get_client_mappings
from azalea_codegen.generator.blocks import generate_blocks

logging.basicConfig(level=logging.INFO)

mappings = get_client_mappings('1.20.1')
burger_data = get_burger_data('1.20.1')
block_report = get_report('1.20.1', 'blocks')
generate_blocks(mappings, burger_data['blocks']['block'], burger_data['blocks']['ordered_blocks'], block_report)