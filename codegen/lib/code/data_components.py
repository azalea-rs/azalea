import json
from typing import Any, Optional
import lib.code.utils
import lib.extract
from lib.utils import identifier_to_path, to_camel_case, to_snake_case


DATA_COMPONENTS_DIR = "azalea-registry/src/item/components/mod.rs"
DEFAULT_DATA_COMPONENTS_DIR = "azalea-registry/src/item/default_components/item.json"


def generate(version_id: str):
    expected_variants = get_expected_variants(version_id)
    actual_variants = get_actual_variants()

    new_variants = []
    removed_variants = []

    for variant in expected_variants:
        if variant not in actual_variants:
            new_variants.append(variant)
    for variant in actual_variants:
        if variant not in expected_variants:
            removed_variants.append(variant)

    print("New variants:")
    for variant in new_variants:
        print("-", variant)
    print()
    print("Removed variants:")
    for variant in removed_variants:
        print("-", variant)
    print()

    for variant in removed_variants:
        print(f"Removing {variant}...")
        remove_variant(variant)
    for variant in new_variants:
        print(f"Adding {variant}...")
        add_variant(variant)

    update_default_variants(version_id)

    lib.code.utils.fmt()

    print("Done!")


def get_expected_variants(version_id: str):
    expected_variants = []
    registries = lib.extract.get_builtin_registries_report(version_id)

    registry = registries["minecraft:data_component_type"]
    registry_entries = sorted(
        registry["entries"].items(), key=lambda x: x[1]["protocol_id"]
    )
    for variant_name, _variant in registry_entries:
        variant_struct_name = to_camel_case(identifier_to_path(variant_name))
        expected_variants.append(variant_struct_name)

    return expected_variants


def get_actual_variants():
    actual_variants = []
    with open(DATA_COMPONENTS_DIR, "r") as f:
        code = f.read().split("\n")

    in_define_macro = False
    for line in code:
        if in_define_macro:
            if line == ");":
                break
            if line.startswith("    "):
                variant_name = line.strip(" ,").split()[0]
                if variant_name[0] in "#/":
                    # skip comments
                    continue
                actual_variants.append(variant_name)
        elif line == "define_data_components!(":
            in_define_macro = True

    return actual_variants


def remove_variant(variant: str):
    with open(DATA_COMPONENTS_DIR, "r") as f:
        code = f.read().split("\n")

    first_line_with_variant = None
    line_after_variant = None

    in_define_macro = False
    for i, line in enumerate(list(code)):
        if in_define_macro:
            if line == ");":
                line_after_variant = i
                break
            if line.startswith("    "):
                if first_line_with_variant is not None:
                    line_after_variant = i
                    break
                variant_name = line.strip().split()[0].strip(",")
                if variant_name[0] in "#/":
                    # skip comments
                    continue
                if variant_name == variant:
                    first_line_with_variant = i
        elif line == "define_data_components!(":
            in_define_macro = True

    if first_line_with_variant is None:
        raise ValueError(f"Variant {variant} not found")
    if line_after_variant is None:
        raise ValueError(f"Couldn't find end of variant {variant}")

    code = code[:first_line_with_variant] + code[line_after_variant:]

    # now remove the struct
    line_before_struct = None  # this is the #[derive] line
    line_after_struct = None  # impl DataComponent for ... {\n...\n}
    for i, line in enumerate(list(code)):
        if line == f"pub struct {variant} {{" or line == f"pub struct {variant};":
            line_before_struct = i - 1
        elif line == "}" and line_before_struct:
            line_after_struct = i + 1
            break
    if line_before_struct is None:
        input(f"Warning: Couldn't find struct {variant}. (press enter to continue)")
    if line_after_struct is None:
        raise ValueError(f"Couldn't find impl DataComponent for {variant}")

    code = code[:line_before_struct] + code[line_after_struct:]

    with open(DATA_COMPONENTS_DIR, "w") as f:
        f.write("\n".join(code))


def add_variant(variant: str):
    with open(DATA_COMPONENTS_DIR, "r") as f:
        code = f.read().split("\n")

    in_define_macro = False
    last_line_in_define_macro = None
    for i, line in enumerate(list(code)):
        if in_define_macro:
            if line == ");":
                last_line_in_define_macro = i
                break
        elif line == "define_data_components!(":
            in_define_macro = True

    if last_line_in_define_macro is None:
        raise ValueError("Couldn't find end of match")

    code = (
        code[:last_line_in_define_macro]
        + [f"    {variant},"]
        + code[last_line_in_define_macro:]
    )

    # now insert the struct
    code.append("")
    code.append("#[derive(Clone, PartialEq, AzBuf, Debug, Serialize)]")
    code.append(f"pub struct {variant} {{")
    code.append("   pub todo: todo!(), // see DataComponents.java")
    code.append("}")

    with open(DATA_COMPONENTS_DIR, "w") as f:
        f.write("\n".join(code))


def update_default_variants(version_id: str):
    items = lib.extract.get_items_report(version_id)

    with open(DEFAULT_DATA_COMPONENTS_DIR, "w") as f:
        f.write(json.dumps(items, indent=2))


def get_enum_and_struct_fields():
    """
    Returns a map like map like `{ "MaxStackSize": { "count": i32 }, "Rarity": [ "common", ... ], ... }`
    with an entry for each struct in components/mod.rs.
    """

    with open(DATA_COMPONENTS_DIR, "r") as f:
        code = f.read().split("\n")

    # we copy from here to `metadatas` if we find a DataComponent impl for the struct
    all_enum_and_struct_fields = {}

    i = 0
    while i < len(code):
        line = code[i]
        if line.startswith("pub struct "):
            struct_name = line.split()[2].strip(":;")
            # map like { "count": i32 }
            this_struct_fields = {}
            if line[-1] not in "};":
                while line != "}":
                    i += 1
                    line = code[i].strip()
                    if line.startswith("pub "):
                        field_name = line.split(" ")[1].strip(":")
                        field_type = line.split(" ", 2)[2].strip(",")
                        this_struct_fields[field_name] = field_type
            all_enum_and_struct_fields[struct_name] = this_struct_fields
        elif line.startswith("pub enum "):
            enum_name = line.split()[2].strip(":")
            # list of string values
            this_enum_variants = []
            if line[-1] not in "};":
                while line != "}":
                    i += 1
                    line = code[i].strip()
                    variant_name = line.split()[0].strip(",")
                    if not variant_name.startswith("#"):
                        this_enum_variants.append(variant_name)
            all_enum_and_struct_fields[enum_name] = this_enum_variants

        i += 1

    return all_enum_and_struct_fields
