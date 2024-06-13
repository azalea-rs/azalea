import lib.code.inventory
import lib.code.registry
import lib.code.version
import lib.code.packet
import lib.code.utils
import lib.code.tags
import lib.download
import lib.extract
import lib.utils

def generate(version_id: str):
    registries = lib.extract.get_registries_report(version_id)

    lib.code.registry.generate_registries(registries)
    lib.code.inventory.update_menus(registries['minecraft:menu']['entries'])


    block_tags = lib.extract.get_registry_tags(version_id, 'block')
    item_tags = lib.extract.get_registry_tags(version_id, 'item')
    fluid_tags = lib.extract.get_registry_tags(version_id, 'fluid')

    lib.code.tags.generate_tags(block_tags, 'blocks', 'Block')
    lib.code.tags.generate_tags(item_tags, 'items', 'Item')
    lib.code.tags.generate_tags(fluid_tags, 'fluids', 'Fluid')

    lib.code.utils.fmt()

    print('Done!')

if __name__ == '__main__':
    version_id = lib.code.version.get_version_id()
    generate(version_id)
