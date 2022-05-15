from utils import to_snake_case, to_camel_case
from mappings import Mappings
import os


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


def generate(burger_packets, mappings: Mappings, target_packet_id, target_packet_direction, target_packet_state):
    for packet in burger_packets.values():
        if packet['id'] != target_packet_id:
            continue

        direction = packet['direction'].lower()  # serverbound or clientbound
        state = {'PLAY': 'game'}.get(packet['state'], packet['state'].lower())

        if state != target_packet_state or direction != target_packet_direction:
            continue

        generated_packet_code = []
        uses = set()
        generated_packet_code.append(
            f'#[derive(Clone, Debug, McBuf, {to_camel_case(state)}Packet)]')
        uses.add(f'packet_macros::{{{to_camel_case(state)}Packet, McBuf}}')

        obfuscated_class_name = packet['class'].split('.')[0].split('$')[0]
        class_name = mappings.get_class(
            obfuscated_class_name).split('.')[-1].split('$')[0]

        generated_packet_code.append(
            f'pub struct {to_camel_case(class_name)} {{')

        for instruction in packet.get('instructions', []):
            if instruction['operation'] == 'write':
                obfuscated_field_name = instruction['field']
                if '.' in obfuscated_field_name or ' ' in obfuscated_field_name or '(' in obfuscated_field_name:
                    generated_packet_code.append(f'// TODO: {instruction}')
                    continue
                field_name = mappings.get_field(
                    obfuscated_class_name, obfuscated_field_name)
                if not field_name:
                    generated_packet_code.append(
                        f'// TODO: unknown field {instruction}')
                    continue

                field_type = instruction['type']
                field_type_rs, is_var, instruction_uses = burger_type_to_rust_type(
                    field_type)
                if is_var:
                    generated_packet_code.append('#[var]')
                generated_packet_code.append(
                    f'pub {to_snake_case(field_name)}: {field_type_rs},')
                uses.update(instruction_uses)
            else:
                generated_packet_code.append(f'// TODO: {instruction}')
                continue

        generated_packet_code.append('}')

        if uses:
            # empty line before the `use` statements
            generated_packet_code.insert(0, '')
        for use in uses:
            generated_packet_code.insert(0, f'use {use};')

        print(generated_packet_code)
        write_packet_file(state, to_snake_case(class_name),
                          '\n'.join(generated_packet_code))
        print()

        mod_rs_dir = f'../azalea-protocol/src/packets/{state}/mod.rs'
        with open(mod_rs_dir, 'r') as f:
            mod_rs = f.read().splitlines()

        pub_mod_line = f'pub mod {to_snake_case(class_name)};'
        if pub_mod_line not in mod_rs:
            mod_rs.insert(0, pub_mod_line)
            packet_mod_rs_line = f'        {hex(packet["id"])}: {to_snake_case(class_name)}::{to_camel_case(class_name)},'

            in_serverbound = False
            in_clientbound = False
            for i, line in enumerate(mod_rs):
                if line.strip() == 'Serverbound => {':
                    in_serverbound = True
                    continue
                elif line.strip() == 'Clientbound => {':
                    in_clientbound = True
                    continue
                elif line.strip() in ('}', '},'):
                    if (in_serverbound and direction == 'serverbound') or (in_clientbound and direction == 'clientbound'):
                        mod_rs.insert(i, packet_mod_rs_line)
                        break
                    in_serverbound = in_clientbound = False
                    continue

                if line.strip() == '' or line.strip().startswith('//') or (not in_serverbound and direction == 'serverbound') or (not in_clientbound and direction == 'clientbound'):
                    continue

                line_packet_id_hex = line.strip().split(':')[0]
                assert line_packet_id_hex.startswith('0x')
                line_packet_id = int(line_packet_id_hex[2:], 16)
                if line_packet_id > packet['id']:
                    mod_rs.insert(i, packet_mod_rs_line)
                    break

            with open(mod_rs_dir, 'w') as f:
                f.write('\n'.join(mod_rs))
