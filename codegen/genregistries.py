import lib.code.inventory
import lib.code.registry
import lib.code.version
import lib.code.utils
import lib.code.tags
import lib.extract


def generate(version_id: str):
    builtin_registries = lib.extract.get_builtin_registries_report(version_id)
    data_registries = lib.extract.get_data_registries(version_id)

    lib.code.registry.generate_builtin_registries(builtin_registries)
    lib.code.registry.generate_data_registries(data_registries)
    lib.code.inventory.update_menus(builtin_registries["minecraft:menu"]["entries"])

    block_tags = lib.extract.get_registry_tags(version_id, "block")
    item_tags = lib.extract.get_registry_tags(version_id, "item")
    fluid_tags = lib.extract.get_registry_tags(version_id, "fluid")
    entity_tags = lib.extract.get_registry_tags(version_id, "entity_type")

    lib.code.tags.generate_tags(block_tags, "blocks", "BlockKind")
    lib.code.tags.generate_tags(item_tags, "items", "ItemKind")
    lib.code.tags.generate_tags(fluid_tags, "fluids", "Fluid")
    lib.code.tags.generate_tags(entity_tags, "entities", "EntityKind")

    lib.code.utils.fmt()

    print("Done!")


if __name__ == "__main__":
    version_id = lib.code.version.get_version_id()
    generate(version_id)
