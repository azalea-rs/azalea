import logging

from azalea_codegen.download_and_extract.data_generators import get_registry_report
from azalea_codegen.generator.registries import generate_registries

logging.basicConfig(level=logging.INFO)

generate_registries(get_registry_report('1.20.1'))
