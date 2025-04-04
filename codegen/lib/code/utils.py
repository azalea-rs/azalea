
from lib.utils import to_camel_case, to_snake_case, get_dir_location
from lib.mappings import Mappings
from typing import Optional
import os

# utilities specifically for codegen


def burger_type_to_rust_type(burger_type, field_name: Optional[str] = None, instruction=None, mappings: Optional[Mappings] = None, obfuscated_class_name: Optional[str] = None):
    is_var = False
    uses = set()
    # extra code, like enum definitions
    extra_code = []

    should_be_signed = False
    if field_name and any(map(lambda w: w in {'x', 'y', 'z', 'xa', 'ya', 'za'}, to_snake_case(field_name).split('_'))):
        # coordinates are signed
        should_be_signed = True

    if burger_type == 'byte':
        field_type_rs = 'i8' if should_be_signed else 'u8'
    elif burger_type == 'short':
        field_type_rs = 'i16' if should_be_signed else 'u16'
    elif burger_type == 'int':
        field_type_rs = 'i32' if should_be_signed else 'u32'
    elif burger_type == 'long':
        field_type_rs = 'i64' if should_be_signed else 'u64'
    elif burger_type == 'float':
        field_type_rs = 'f32'
    elif burger_type == 'double':
        field_type_rs = 'f64'

    elif burger_type == 'varint':
        is_var = True
        field_type_rs = 'i32' if should_be_signed else 'u32'
    elif burger_type == 'varlong':
        is_var = True
        field_type_rs = 'i64' if should_be_signed else 'u64'

    elif burger_type == 'boolean':
        field_type_rs = 'bool'
    elif burger_type == 'string':
        field_type_rs = 'String'

    elif burger_type == 'chatcomponent':
        field_type_rs = 'FormattedText'
        uses.add('azalea_chat::FormattedText')
    elif burger_type == 'identifier':
        field_type_rs = 'ResourceLocation'
        uses.add('azalea_core::resource_location::ResourceLocation')
    elif burger_type == 'uuid':
        field_type_rs = 'Uuid'
        uses.add('uuid::Uuid')
    elif burger_type == 'position':
        field_type_rs = 'BlockPos'
        uses.add('azalea_core::position::BlockPos')
    elif burger_type == 'nbtcompound':
        field_type_rs = 'simdnbt::owned::NbtCompound'
    elif burger_type == 'itemstack':
        field_type_rs = 'Slot'
        uses.add('azalea_core::slot::Slot')
    elif burger_type == 'metadata':
        field_type_rs = 'EntityMetadata'
        uses.add('azalea_entity::EntityMetadata')
    elif burger_type == 'bitset':
        if instruction:
            length = instruction['length']
            field_type_rs = f'todo!("fixed bitset of length {length}")'
        else:
            field_type_rs = 'todo!("fixed bitset")'
    elif burger_type == 'abstract':
        field_type_rs = 'todo!()'
    elif burger_type == 'interface':
        # depends on context
        field_type_rs = 'todo!()'
    elif burger_type == 'Iterator':
        field_type_rs = 'todo!()'
    elif burger_type == 'Object':
        # depends on context
        field_type_rs = 'todo!()'
    elif burger_type == 'enum':
        if not instruction or not mappings or not obfuscated_class_name:
            field_type_rs = 'todo!("enum")'
        else:
            # generate the whole enum :)
            print(instruction)
            enum_field = instruction['field']
            # enums with a.b() as the field
            if '.' in enum_field:
                enum_first_part_name = mappings.get_field_type(
                    obfuscated_class_name, enum_field.split('.')[0])
                enum_first_part_obfuscated_name = mappings.get_class_from_deobfuscated_name(
                    enum_first_part_name)
                print('enum_first_part_obfuscated_name',
                      enum_first_part_obfuscated_name)
                print('enum field', enum_field.split('.')[1].split('(')[0])
                try:
                    enum_name = mappings.get_method_type(
                        enum_first_part_obfuscated_name, enum_field.split('.')[1].split('(')[0], '')
                except KeyError:
                    # sometimes enums are fields instead of methods
                    enum_name = mappings.get_field_type(
                        enum_first_part_obfuscated_name, enum_field.split('.')[1].split('(')[0])

                print('hm', enum_name)
            else:
                try:
                    enum_name = mappings.get_field_type(
                        obfuscated_class_name, enum_field)
                except:
                    enum_name = mappings.get_class(obfuscated_class_name)
                    print(f'failed getting {obfuscated_class_name}.{enum_field} but continuing with {enum_name} anyways')
            print('enum_name', enum_name)
            enum_obfuscated_name = mappings.get_class_from_deobfuscated_name(
                enum_name)
            print('enum_obfuscated_name', enum_obfuscated_name)
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
            extra_code.append('')
            extra_code.append(f'#[derive(AzBuf, Clone, Copy, Debug)]')
            extra_code.append(f'pub enum {field_type_rs} {{')
            for index, variant in enumerate(enum_variants):
                extra_code.append(
                    f'    {to_camel_case(variant.lower())}={index},')
            extra_code.append('}')

    elif burger_type.endswith('[]'):
        field_type_rs, is_var, uses, extra_code = burger_type_to_rust_type(
            burger_type[:-2])
        field_type_rs = f'Vec<{field_type_rs}>'

        # sometimes burger gives us a slightly incorrect type
        if mappings and instruction:
            if field_type_rs == 'Vec<u8>':
                field = instruction['field']
                if field.endswith('.copy()'):
                    field = field[:-7]
                try:
                    array_type = mappings.get_field_type(
                        obfuscated_class_name, field)
                except KeyError:
                    print('Error getting array type', field)
                    return field_type_rs, is_var, uses, extra_code
                if array_type == 'net.minecraft.network.FriendlyByteBuf':
                    field_type_rs = 'UnsizedByteArray'
                    uses.add('azalea_buf::UnsizedByteArray')

    else:
        print('instruction that we errored on:', instruction)
        deobfuscated_class_name = mappings.get_class(obfuscated_class_name) if obfuscated_class_name else None
        raise Exception(f'Unknown field type: {burger_type} ({deobfuscated_class_name or obfuscated_class_name})')
    return field_type_rs, is_var, uses, extra_code


def write_packet_file(state, packet_module_name, code):
    with open(get_dir_location(f'../azalea-protocol/src/packets/{state}/{packet_module_name}.rs'), 'w') as f:
        f.write(code)


def fmt():
    os.system(f'cd {get_dir_location("..")} && cargo fmt')
