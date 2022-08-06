
from lib.mappings import Mappings
from lib.utils import to_camel_case
from lib.utils import get_dir_location
from typing import Optional
import os

# utilities specifically for codegen


def burger_type_to_rust_type(burger_type, field_name: Optional[str] = None, instruction=None, mappings: Optional[Mappings] = None, obfuscated_class_name: Optional[str] = None):
    is_var = False
    uses = set()
    # extra code, like enum definitions
    extra_code = []

    should_be_unsigned = field_name is not None and (
        field_name == 'id' or field_name.endswith('_id'))

    if burger_type == 'byte':
        field_type_rs = 'u8' if should_be_unsigned else 'i8'
    elif burger_type == 'short':
        field_type_rs = 'u16' if should_be_unsigned else 'i16'
    elif burger_type == 'int':
        field_type_rs = 'u32' if should_be_unsigned else 'i32'
    elif burger_type == 'long':
        field_type_rs = 'u64' if should_be_unsigned else 'i64'
    elif burger_type == 'float':
        field_type_rs = 'f32'
    elif burger_type == 'double':
        field_type_rs = 'f64'

    elif burger_type == 'varint':
        is_var = True
        field_type_rs = 'u32' if should_be_unsigned else 'i32'
    elif burger_type == 'varlong':
        is_var = True
        field_type_rs = 'u64' if should_be_unsigned else 'i64'

    elif burger_type == 'boolean':
        field_type_rs = 'bool'
    elif burger_type == 'string':
        field_type_rs = 'String'

    elif burger_type == 'chatcomponent':
        field_type_rs = 'Component'
        uses.add('azalea_chat::component::Component')
    elif burger_type == 'identifier':
        field_type_rs = 'ResourceLocation'
        uses.add('azalea_core::resource_location::ResourceLocation')
    elif burger_type == 'uuid':
        field_type_rs = 'Uuid'
        uses.add('uuid::Uuid')
    elif burger_type == 'position':
        field_type_rs = 'BlockPos'
        uses.add('azalea_core::BlockPos')
    elif burger_type == 'nbtcompound':
        field_type_rs = 'azalea_nbt::Tag'
    elif burger_type == 'itemstack':
        field_type_rs = 'Slot'
        uses.add('azalea_core::Slot')
    elif burger_type == 'metadata':
        field_type_rs = 'EntityMetadata'
        uses.add('crate::mc_buf::EntityMetadata')
    elif burger_type == 'enum':
        if not instruction or not mappings or not obfuscated_class_name:
            field_type_rs = 'todo!("enum")'
        else:
            # generate the whole enum :)
            enum_name = mappings.get_field_type(
                obfuscated_class_name, instruction['field'])
            enum_obfuscated_name = mappings.get_class_from_deobfuscated_name(
                enum_name)
            enum_variants = []
            for obfuscated_field_name in mappings.fields[enum_obfuscated_name]:
                field_name = mappings.get_field(
                    enum_obfuscated_name, obfuscated_field_name)

                # get the type just to make sure it's actually a variant and not something else
                field_type = mappings.get_field_type(
                    enum_obfuscated_name, obfuscated_field_name)
                if field_type != enum_name:
                    continue

                enum_variants.append(field_name)

            field_type_rs = to_camel_case(
                enum_name.split('.')[-1].split('$')[-1])
            extra_code.append(f'#[derive(McBuf, Copy, Debug)]')
            extra_code.append(f'enum {field_type_rs} {{')
            for index, variant in enumerate(enum_variants):
                print(field_name)
                extra_code.append(
                    f'    {to_camel_case(variant.lower())}={index},')
            extra_code.append('}')
            print(extra_code)

    elif burger_type.endswith('[]'):
        field_type_rs, is_var, uses = burger_type_to_rust_type(
            burger_type[:-2])
        field_type_rs = f'Vec<{field_type_rs}>'
    else:
        raise Exception(f'Unknown field type: {burger_type}')
    return field_type_rs, is_var, uses, extra_code


def write_packet_file(state, packet_name_snake_case, code):
    with open(get_dir_location(f'../azalea-protocol/src/packets/{state}/{packet_name_snake_case}.rs'), 'w') as f:
        f.write(code)


def fmt():
    os.system(f'cd {get_dir_location("..")} && cargo fmt')
