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

    lib.code.tags.generate_tags(builtin_registries, block_tags, "blocks", "block")
    lib.code.tags.generate_tags(builtin_registries, item_tags, "items", "item")
    lib.code.tags.generate_tags(builtin_registries, fluid_tags, "fluids", "fluid")
    lib.code.tags.generate_tags(
        builtin_registries, entity_tags, "entities", "entity_type"
    )

    lib.code.utils.fmt()

    print("Done!")


if __name__ == "__main__":
    version_id = lib.code.version.get_version_id()
    generate(version_id)
