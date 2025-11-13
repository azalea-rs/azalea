from typing import Any, Optional
import lib.code.utils
import lib.extract
import lib.utils


DATA_COMPONENTS_DIR = "azalea-inventory/src/components/mod.rs"
DEFAULT_DATA_COMPONENTS_DIR = "azalea-inventory/src/default_components/generated.rs"


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
    registries = lib.extract.get_registries_report(version_id)

    registry = registries["minecraft:data_component_type"]
    registry_entries = sorted(
        registry["entries"].items(), key=lambda x: x[1]["protocol_id"]
    )
    for variant_name, _variant in registry_entries:
        variant_struct_name = lib.utils.to_camel_case(variant_name.split(":")[-1])
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
        elif line == "}":
            line_after_struct = i + 1
            break
    if line_before_struct is None:
        raise ValueError(f"Couldn't find struct {variant}")
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

    code = """// This file was @generated by codegen/lib/code/components.py, don't edit it
// manually!

#![allow(clippy::all)]

use std::collections::HashMap;

use azalea_chat::translatable_component::TranslatableComponent;
use azalea_registry::{Attribute, Block, EntityKind, HolderSet, Item, MobEffect, SoundEvent};
use simdnbt::owned::NbtCompound;

use crate::{
    ItemStack, components::*, default_components::DefaultableComponent,
    item::consume_effect::ConsumeEffect,
};

""".splitlines()

    #  { max_stack_size: { air: 64, ... } }
    components_to_item_defaults = {}

    for item_resource_id, data in items.items():
        item_resource_id = item_resource_id.split(":")[1]
        components = data["components"]
        for component_resource_id, component_value in components.items():
            component_resource_id = component_resource_id.split(":")[1]
            if component_resource_id not in components_to_item_defaults:
                components_to_item_defaults[component_resource_id] = {}
            components_to_item_defaults[component_resource_id][item_resource_id] = (
                component_value
            )

    registries = lib.extract.get_registries_report(version_id)
    item_resource_id_to_protocol_id = {}
    item_resource_ids = [None] * len(registries["minecraft:item"]["entries"])
    for item_resource_id, item_data in registries["minecraft:item"]["entries"].items():
        item_resource_id = item_resource_id.split(":")[-1]
        item_protocol_id = item_data["protocol_id"]
        item_resource_id_to_protocol_id[item_resource_id] = item_protocol_id
        item_resource_ids[item_protocol_id] = item_resource_id

    enum_and_struct_fields = get_enum_and_struct_fields()
    # a few types that exist elsewhere
    enum_and_struct_fields["ConsumeEffect::ApplyEffects"] = {
        "effects": "Vec<MobEffectInstance>",
        "probability": "f32",
    }
    enum_and_struct_fields["ConsumeEffect::RemoveEffects"] = {
        "effects": "HolderSet<MobEffect, Identifier>",
    }
    enum_and_struct_fields["ConsumeEffect::ClearAllEffects"] = {}
    enum_and_struct_fields["ConsumeEffect::TeleportRandomly"] = {
        "diameter": "f32",
    }
    enum_and_struct_fields["ConsumeEffect::PlaySound"] = {
        "sound": "SoundEvent",
    }

    # we can't call ::new() on enum variants, so define the defaults manually here
    enum_variant_defaults = {
        "ConsumeEffect::ApplyEffects": {
            "effects": [],
            "probability": 1.0,
        },
        "ConsumeEffect::TeleportRandomly": {
            "diameter": 16.0,
        },
    }

    def python_to_rust_value(python_value: Any, target_rust_type: Optional[str]):
        # manual implementations
        if isinstance(python_value, dict) and len(python_value) > 0:
            if target_rust_type == "ConsumeEffect":
                variant = lib.utils.to_camel_case(python_value["type"].split(":")[-1])
                type_with_variant = f"ConsumeEffect::{variant}"
                details_without_type = python_value.copy()
                del details_without_type["type"]
                return python_to_rust_value(details_without_type, type_with_variant)
            elif target_rust_type == "MobEffectInstance":
                effect_id = python_value["id"]
                details_without_id = python_value.copy()
                del details_without_id["id"]
                return (
                    "MobEffectInstance {"
                    + f"id: {python_to_rust_value(effect_id, 'MobEffect')},"
                    + f"details: {python_to_rust_value(details_without_id, 'MobEffectDetails')}"
                    + "}"
                )
            elif target_rust_type == "AttributeModifiersEntry":
                attribute = python_value["type"]
                amount = python_value["amount"]
                display_type = python_value.get("display", {}).get("type") or "default"
                id = python_value["id"]
                operation = python_value["operation"]

                del python_value["amount"]
                del python_value["type"]
                python_value["kind"] = attribute
                del python_value["id"]
                del python_value["operation"]
                if display_type is not None:
                    python_value["display"] = display_type
                python_value["modifier"] = {
                    "id": id,
                    "amount": amount,
                    "operation": operation,
                }

        if target_rust_type is None:
            return "None"

        if target_rust_type.startswith("Option<"):
            if python_value is None:
                return "None"
            inner_type = target_rust_type.split("<", 1)[1].rsplit(">", 1)[0]
            return f"Some({python_to_rust_value(python_value, inner_type)})"
        elif target_rust_type.startswith("HashMap<"):
            hashmap_key, hashmap_value = (
                target_rust_type.split("<", 1)[1].rsplit(">", 1)[0].split(",", 1)
            )
            hashmap_key = hashmap_key.strip()
            hashmap_value = hashmap_value.strip()

            # HashMap::from_iter([("honey_level".to_string(), "0".to_string())])
            t = "HashMap::from_iter(["
            for k, v in python_value.items():
                t += f"({python_to_rust_value(k, hashmap_key)}, {python_to_rust_value(v, hashmap_value)}),"
            t = t.rstrip(",") + "])"
            return t
        elif target_rust_type == "String":
            return f'"{python_value}".to_string()'
        elif target_rust_type == "&str":
            if isinstance(python_value, dict):
                return python_to_rust_value(
                    list(python_value.values())[0], target_rust_type
                )
            return f'"{python_value}"'
        elif target_rust_type in {'i64', 'u64', 'f64', 'i32', 'u32', 'f32', 'i16', 'u16', 'i8', 'u8'}:  # fmt: skip
            if isinstance(python_value, dict) and len(python_value) == 1:
                return python_to_rust_value(
                    list(python_value.values())[0], target_rust_type
                )
            return str(python_value)
        elif target_rust_type == "EntityKind":
            # Special handling for EntityKind - can be from NBT compound id field or direct string
            entity_id = None
            if isinstance(python_value, dict) and "id" in python_value:
                entity_id = python_value["id"]
            elif isinstance(python_value, str):
                entity_id = python_value

            if entity_id and entity_id.startswith("minecraft:"):
                entity_name = entity_id[10:]  # Remove "minecraft:" prefix
                entity_name_camel = lib.utils.to_camel_case(entity_name)
                return f"EntityKind::{entity_name_camel}"
            raise ValueError(f"Unknown or missing EntityKind: {python_value}")
        elif target_rust_type == "NbtCompound":
            # NbtCompound::from_values([
            #     ("id".into(), "minecraft:allay".into()),
            # ]),
            t = "NbtCompound::from_values(vec!["
            for k, v in python_value.items():
                if isinstance(v, str):
                    t += f'("{k}".into(), "{v}".into()),'
                else:
                    t += f'("{k}".into(), {python_to_rust_value(v, "FIXME_UNKNOWN_NBT")}),'
            t = t.rstrip(",") + "])"
            return t

        if isinstance(python_value, dict):
            if target_rust_type == "Identifier" and len(python_value) == 1:
                return python_to_rust_value(
                    list(python_value.values())[0], target_rust_type
                )
            elif target_rust_type.startswith("HolderSet<") and len(python_value) == 1:
                return python_to_rust_value(
                    list(python_value.values())[0], target_rust_type
                )
            elif target_rust_type.startswith("Vec<") and len(python_value) == 1:
                return python_to_rust_value(
                    list(python_value.values())[0], target_rust_type
                )
            elif target_rust_type == "ItemStack":
                item_rust_value = python_to_rust_value(python_value["id"], "Item")
                count = python_value["count"]
                if count == 1:
                    return f"ItemStack::from({item_rust_value})"
                else:
                    return f"ItemStack::new({item_rust_value}, {python_to_rust_value(python_value['count'], 'i32')})"

            if "::" in target_rust_type and target_rust_type in enum_variant_defaults:
                # we can't call ::new() on enum variants, so extend the python type with the defaults
                python_value = {
                    **enum_variant_defaults[target_rust_type],
                    **python_value,
                }

            # the :: check is so we don't do this for enum variants
            if len(python_value) == 0 and "::" not in target_rust_type:
                if (
                    target_rust_type in enum_and_struct_fields
                    and len(enum_and_struct_fields[target_rust_type]) == 0
                ):
                    # don't do ::new() for structs with no fields (like `Glider`)
                    return f"{target_rust_type}"
                # this ::new has to be implemented manually for these types
                t = f"{target_rust_type.split('<')[0]}::new()"
            else:
                # create a struct based on the defaults
                t = f"{target_rust_type} {{"
                for k, v in python_value.items():
                    # get the type of the fields
                    inner_type = enum_and_struct_fields.get(target_rust_type, {}).get(
                        k, "FIXME_UNKNOWN_TYPE"
                    )
                    t += f"{k}: {python_to_rust_value(v, inner_type)},"

                # add ..Struct::new(), unless we already know that all of the fields are there
                if len(python_value) < len(
                    enum_and_struct_fields.get(target_rust_type, [])
                ):
                    t += f"..{target_rust_type}::new()"

                t += "}"
            return t
        if isinstance(python_value, bool):
            return str(python_value).lower()
        if isinstance(python_value, str):
            fields_for_rust_type = enum_and_struct_fields.get(target_rust_type, [])
            if "Referenced(Identifier)" in fields_for_rust_type:
                return f"{target_rust_type}::Referenced({python_to_rust_value(python_value, 'Identifier')})"
            elif "Registry(registry::Instrument)" in fields_for_rust_type:
                return f"{target_rust_type}::Registry({python_to_rust_value(python_value, 'azalea_registry::Instrument')})"
            elif target_rust_type.startswith("HolderSet<"):
                holderset_type = target_rust_type.split("<", 1)[1].split(",", 1)[0]
                main_vec = python_to_rust_value(
                    [python_value], f"Vec<{holderset_type}>"
                )
                return f"HolderSet::Direct {{ contents: {main_vec} }}"
            elif target_rust_type.startswith("azalea_registry::Holder<"):
                holder_type = target_rust_type.split("<", 1)[1].split(",", 1)[0]
                inner_type = python_to_rust_value(python_value, holder_type)
                return f"azalea_registry::Holder::Reference({inner_type})"
            elif target_rust_type == "Identifier":
                # convert minecraft:air into Identifier::from_static("minecraft:air")
                return f'"{python_value}".into()'
            else:
                # enum variant
                return f"{target_rust_type}::{lib.utils.to_camel_case(python_value.split(':')[-1])}"
        if isinstance(python_value, list):
            # convert Vec<Thing> into Thing
            main_vec = "vec!["
            inner_type = (
                target_rust_type.split("<", 1)[1]
                .rsplit(">", 1)[0]
                .split(",")[0]
                .strip()
                if (target_rust_type and "<" in target_rust_type)
                else None
            )
            # convert [Thing; 2] into Thing
            if target_rust_type.startswith("[") and target_rust_type.endswith("]"):
                inner_type = target_rust_type.split(";")[0].strip("[]")
                main_vec = "["

            if inner_type is None:
                # if the only field is a Vec, use that as the type
                rust_type_fields = enum_and_struct_fields.get(target_rust_type, {})
                if len(rust_type_fields) == 1 and isinstance(rust_type_fields, dict):
                    _field_name, field_type = list(rust_type_fields.items())[0]
                    return python_to_rust_value(python_value, field_type)

            vectors = []
            for v in python_value:
                # handle tags correctly
                if isinstance(v, str) and v.startswith("#minecraft:"):
                    tag_name = lib.utils.to_snake_case(v.split(":")[-1]).upper()
                    if inner_type == "EntityKind":
                        tag_module = "entities"
                    elif inner_type == "Item":
                        tag_module = "items"
                    elif inner_type == "Block":
                        tag_module = "blocks"
                    else:
                        tag_module = "FIXME_UNKNOWN_MODULE"
                    vectors.append(
                        f"azalea_registry::tags::{tag_module}::{tag_name}.clone().into_iter().collect()"
                    )
                    continue
                main_vec += python_to_rust_value(v, inner_type) + ","
            main_vec = main_vec.rstrip(",") + "]"
            if len(vectors) == 0 or main_vec != "vec![]":
                vectors.append(main_vec)

            if len(vectors) == 1:
                as_vec = vectors[0]
            else:
                # concat
                as_vec = f"[{','.join(vectors)}].concat()"

            if target_rust_type.startswith("HolderSet<"):
                return f"HolderSet::Direct {{ contents: {as_vec} }}"
            return as_vec

        return str(python_value)

    for component_resource_id, item_defaults in components_to_item_defaults.items():
        component_struct_name = lib.utils.to_camel_case(component_resource_id)
        component_struct_fields = enum_and_struct_fields[component_struct_name]

        if len(component_struct_fields) == 1 and isinstance(
            component_struct_fields, dict
        ):
            field_name, field_type = list(component_struct_fields.items())[0]

            # if field_type not in ["i32", "u32", "f32", "bool"]:
            #     continue

            def transform_value_fn(rust_value: str):
                return f"{component_struct_name} {{ {field_name}: {rust_value} }}"
        else:
            field_type = component_struct_name
            # if component_resource_id != "rarity":
            #     continue

            def transform_value_fn(rust_value: str):
                return rust_value

        rust_value = "value"
        if component_resource_id == "item_name":
            rust_value = f"TranslatableComponent::from({rust_value}).into()"
            field_type = "&str"
        elif component_resource_id == "item_model":
            rust_value = f"{rust_value}.into()"
            field_type = "&str"

        elif component_resource_id == "entity_data":
            # Special handling for EntityData to use EntityData structure
            # Keep rust_value as "value" so it gets processed correctly
            field_type = "EntityKind"

            def transform_value_fn(rust_value: str):
                return f"{component_struct_name} {{ kind: {rust_value}, data: NbtCompound::new() }}"

        item_defaults_original = item_defaults
        item_defaults = {}
        for k, v in item_defaults_original.items():
            item_defaults[k] = python_to_rust_value(v, field_type)

        default_values_frequency = {}
        for value in item_defaults.values():
            if value not in default_values_frequency:
                default_values_frequency[value] = 0
            default_values_frequency[value] += 1
        most_common_default_value = max(
            default_values_frequency.items(), key=lambda x: x[1]
        )[0]
        default_values_count_except_most_common = (
            len(item_defaults) - default_values_frequency[most_common_default_value]
        )

        # if it looks like there's a default (like, vec![]) that's used for most items, then we
        # always use a match statement with a default handler
        includes_every_item_but_mostly_same_values = (
            len(item_resource_ids) == len(item_defaults)
            and default_values_count_except_most_common <= 128
        )

        # use a lookup table for some components to avoid big match statements
        if len(item_defaults) > 128 and not includes_every_item_but_mostly_same_values:
            static_values_name = component_resource_id.upper() + "_VALUES"

            values_set = set(item_defaults.values())
            if len(values_set) == 1:
                # always returns the same value
                code.append(f"impl DefaultableComponent for {component_struct_name} {{")
                code.append("    fn default_for_item(_item: Item) -> Option<Self> {")
                value = next(iter(values_set))
                code.append(f"        Some({transform_value_fn(value)})")
                code.append("    }")
                code.append("}")
                continue

            # find a sentinel value that isn't already being used
            none_value = 0
            while none_value in values_set:
                none_value += 1
            none_value_is_used = False

            static_def_line = f"static {static_values_name}: [{field_type}; {len(item_resource_ids)}] = ["
            for item_protocol_id, item_resource_id in enumerate(item_resource_ids):
                value = item_defaults.get(item_resource_id, none_value)
                static_def_line += f"{value},"
                if value == none_value:
                    none_value_is_used = True
            static_def_line = static_def_line.rstrip(",")
            static_def_line += "];"

            code.append("#[rustfmt::skip]")
            code.append(static_def_line)

            code.append(f"impl DefaultableComponent for {component_struct_name} {{")
            code.append("    fn default_for_item(item: Item) -> Option<Self> {")
            code.append(f"        let value = {static_values_name}[item as usize];")
            if none_value_is_used:
                code.append(f"        if value == {none_value} {{")
                code.append("            return None;")
                code.append("        }")
            code.append(f"        Some({transform_value_fn(rust_value)})")
            code.append("    }")
            code.append("}")
        elif includes_every_item_but_mostly_same_values:
            code.append(f"impl DefaultableComponent for {component_struct_name} {{")
            if default_values_count_except_most_common > 0:
                code.append("    fn default_for_item(item: Item) -> Option<Self> {")
                code.append("        let value = match item {")
                for item_resource_id, value in item_defaults.items():
                    if value == most_common_default_value:
                        continue
                    item_variant_name = lib.utils.to_camel_case(item_resource_id)
                    code.append(f"            Item::{item_variant_name} => {value},")
                code.append(f"            _ => {most_common_default_value},")
                code.append("        };")
                code.append(f"        Some({transform_value_fn('value')})")
            else:
                code.append("    fn default_for_item(_item: Item) -> Option<Self> {")
                code.append(
                    f"        Some({transform_value_fn(most_common_default_value)})"
                )
            code.append("    }")
            code.append("}")
        else:
            code.append(f"impl DefaultableComponent for {component_struct_name} {{")
            code.append("    fn default_for_item(item: Item) -> Option<Self> {")
            code.append("        let value = match item {")
            for item_resource_id, value in item_defaults.items():
                item_variant_name = lib.utils.to_camel_case(item_resource_id)
                code.append(f"            Item::{item_variant_name} => {value},")
            code.append("            _ => return None,")
            code.append("        };")
            code.append(f"        Some({transform_value_fn('value')})")
            code.append("    }")
            code.append("}")

    with open(DEFAULT_DATA_COMPONENTS_DIR, "w") as f:
        f.write("\n".join(code))


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
