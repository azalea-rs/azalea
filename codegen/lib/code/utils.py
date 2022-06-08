
import os

# utilities specifically for codegen


def burger_type_to_rust_type(burger_type):
    is_var = False
    uses = set()

    if burger_type == 'byte':
        field_type_rs = 'i8'
    elif burger_type == 'short':
        field_type_rs = 'i16'
    elif burger_type == 'int':
        field_type_rs = 'i32'
    elif burger_type == 'long':
        field_type_rs = 'i64'
    elif burger_type == 'float':
        field_type_rs = 'f32'
    elif burger_type == 'double':
        field_type_rs = 'f64'

    elif burger_type == 'varint':
        is_var = True
        field_type_rs = 'i32'
    elif burger_type == 'varlong':
        is_var = True
        field_type_rs = 'i64'

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
        # enums are too complicated, leave those to the user
        field_type_rs = 'todo!()'
    elif burger_type.endswith('[]'):
        field_type_rs, is_var, uses = burger_type_to_rust_type(
            burger_type[:-2])
        field_type_rs = f'Vec<{field_type_rs}>'
    else:
        print('Unknown field type:', burger_type)
        exit()
    return field_type_rs, is_var, uses


def write_packet_file(state, packet_name_snake_case, code):
    with open(f'../azalea-protocol/src/packets/{state}/{packet_name_snake_case}.rs', 'w') as f:
        f.write(code)


def fmt():
    os.system('cd .. && cargo fmt')
