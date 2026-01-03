from lib.utils import get_dir_location, to_camel_case

BUILTIN_REGISTRIES_DIR = get_dir_location("../azalea-registry/src/builtin.rs")
DATA_REGISTRIES_DIR = get_dir_location("../azalea-registry/src/data.rs")


def generate_builtin_registries(registries: dict):
    with open(BUILTIN_REGISTRIES_DIR, "r") as f:
        code = f.read().split("\n")

    existing_registry_enum_names = set()

    for registry_name, registry in registries.items():
        # registry!(BlockKind, {
        #     Air => "minecraft:air",
        #     Stone => "minecraft:stone"
        # });

        registry_name = registry_name.split(":")[1]
        registry_enum_name = registry_name_to_enum_name(registry_name)

        existing_registry_enum_names.add(registry_enum_name)

        registry_code = []
        registry_code.append(f"enum {registry_enum_name} {{")
        registry_entries = sorted(
            registry["entries"].items(), key=lambda x: x[1]["protocol_id"]
        )
        for variant_name, _variant in registry_entries:
            # strip out the "minecraft:" prefix
            variant_name = variant_name.split(":")[-1]
            variant_struct_name = to_camel_case(variant_name)
            registry_code.append(f'\t{variant_struct_name} => "{variant_name}",')
        registry_code.append("}")

        # when we find a "registry! {" line, find the next line that starts
        # with "enum <name>" and replace that until we find a line that's "}"
        found = False
        in_registry_macro = False
        for i, line in enumerate(list(code)):
            if not in_registry_macro and line == "registry! {":
                in_registry_macro = True
            elif in_registry_macro and line == registry_code[0]:
                # found it, now delete until we get to "}"
                while code[i] != "}":
                    code.pop(i)
                code[i] = "\n".join(registry_code)
                found = True
                break
        if not found:
            code.append("registry! {")
            code.append("\n".join(registry_code))
            code.append("}")
            code.append("")

    # delete the unused registries
    i = 0
    while i < len(code):
        if code[i] == "registry! {":
            # skip until we get to the enum line
            while not code[i].startswith("enum "):
                i += 1
            enum_name = code[i].split(" ")[1]
            if enum_name not in existing_registry_enum_names:
                i -= 1
                while code[i] != "}":
                    code.pop(i)
                code.pop(i)
                # close the registry! block
                code.pop(i)
        else:
            i += 1

    with open(BUILTIN_REGISTRIES_DIR, "w") as f:
        f.write("\n".join(code))


# data_registries looks like { "enchantment": [ "aqua_affinity", ... ] }
def generate_data_registries(data_registries: dict):
    with open(DATA_REGISTRIES_DIR, "r") as f:
        code = f.read().split("\n")

    existing_registry_struct_names = set()
    for registry_name, registry_entries in data_registries.items():
        registry_enum_name = registry_name_to_enum_name(registry_name.split("/")[-1])
        existing_registry_struct_names.add(registry_enum_name)

    # delete the unused data registries
    i = 0
    while i < len(code):
        if code[i] == "data_registry! {":
            i += 1
            struct_name = code[i].split(" ")[0]
            if struct_name not in existing_registry_struct_names:
                print("removing data registry", struct_name)
                i -= 1
                while code[i] != "}":
                    code.pop(i)
                code.pop(i)
                # close the data_registry! block
                code.pop(i)
        else:
            i += 1

    for registry_name, registry_entries in data_registries.items():
        # data_registry! {
        #     Enchantment => "enchantment",
        #     enum EnchantmentKey {
        #         AquaAffinity => "minecraft:aqua_affinity",
        #     }
        # }

        registry_enum_name = registry_name_to_enum_name(registry_name.split("/")[-1])

        registry_code = []
        registry_code.append(f'{registry_enum_name} => "{registry_name}",')
        registry_code.append(f"enum {registry_enum_name}Key {{")
        registry_entries.sort()
        for variant_name in registry_entries:
            variant_struct_name = to_camel_case(variant_name.split(":")[-1])
            registry_code.append(f'    {variant_struct_name} => "{variant_name}",')
        registry_code.append("}")

        # when we find a "data_registry! {" line, find the next line that starts
        # with "enum <name>" and replace that until we find a line that's "}"
        found = False
        in_registry_macro = False
        for i, line in enumerate(list(code)):
            if not in_registry_macro and line == "data_registry! {":
                in_registry_macro = True
            elif in_registry_macro and line == registry_code[1]:
                # found it, now delete until we get to "}"
                while code[i] != "}":
                    code.pop(i)
                code[i] = "\n".join(registry_code[1:])
                found = True
                break
        if not found:
            code.append("data_registry! {")
            code.append("\n".join(registry_code))
            code.append("}")
            code.append("")

    with open(DATA_REGISTRIES_DIR, "w") as f:
        f.write("\n".join(code))


def registry_name_to_enum_name(registry_name: str) -> str:
    registry_name = registry_name.split(":")[-1]

    if registry_name == "block_type":
        # avoid conflicting with BlockKind
        registry_name = "abstract_block_kind"
    elif registry_name.endswith("_type"):
        # change _type to _kind because that's Rustier (and because _type
        # is a reserved keyword)
        registry_name = registry_name[:-5] + "_kind"
    elif registry_name in {"menu", "block", "item"}:
        registry_name += "_kind"

    return to_camel_case(registry_name)
