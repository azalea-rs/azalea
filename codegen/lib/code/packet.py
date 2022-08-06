from typing import Optional
from lib.code.utils import burger_type_to_rust_type, write_packet_file
from lib.utils import padded_hex, to_snake_case, to_camel_case, get_dir_location
from lib.mappings import Mappings
import os
import re


def make_packet_mod_rs_line(packet_id: int, packet_class_name: str):
    return f'        {padded_hex(packet_id)}: {to_snake_case(packet_class_name)}::{to_camel_case(packet_class_name)},'


def fix_state(state: str):
    return {'PLAY': 'game'}.get(state, state.lower())


def generate_packet(burger_packets, mappings: Mappings, target_packet_id, target_packet_direction, target_packet_state):
    for packet in burger_packets.values():
        if packet['id'] != target_packet_id:
            continue

        direction = packet['direction'].lower()  # serverbound or clientbound
        state = fix_state(packet['state'])

        if state != target_packet_state or direction != target_packet_direction:
            continue

        generated_packet_code = []
        uses = set()

        packet_derive_name = f'{to_camel_case(direction)}{to_camel_case(state)}Packet'

        generated_packet_code.append(
            f'#[derive(Clone, Debug, McBuf, {packet_derive_name})]')
        uses.add(f'packet_macros::{packet_derive_name}')
        uses.add(f'azalea_buf::McBuf')

        obfuscated_class_name = packet['class'].split('.')[0]
        class_name = mappings.get_class(
            obfuscated_class_name).split('.')[-1]
        if '$' in class_name:
            class_name = class_name.replace('$', '')

        generated_packet_code.append(
            f'pub struct {to_camel_case(class_name)} {{')

        for instruction in packet.get('instructions', []):
            if instruction['operation'] == 'write':
                burger_instruction_to_code(
                    instruction, generated_packet_code, mappings, obfuscated_class_name, uses)
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

        mod_rs_dir = get_dir_location(
            f'../azalea-protocol/src/packets/{state}/mod.rs')
        with open(mod_rs_dir, 'r') as f:
            mod_rs = f.read().splitlines()

        pub_mod_line = f'pub mod {to_snake_case(class_name)};'
        if pub_mod_line not in mod_rs:
            mod_rs.insert(0, pub_mod_line)
            packet_mod_rs_line = make_packet_mod_rs_line(
                packet['id'], class_name)

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


def set_packets(packet_ids: list[int], packet_class_names: list[str], direction: str, state: str):
    assert len(packet_ids) == len(packet_class_names)

    # sort the packets by id
    packet_ids, packet_class_names = [list(x) for x in zip(
        *sorted(zip(packet_ids, packet_class_names), key=lambda pair: pair[0]))]  # type: ignore

    mod_rs_dir = get_dir_location(
        f'../azalea-protocol/src/packets/{state}/mod.rs')
    with open(mod_rs_dir, 'r') as f:
        mod_rs = f.read().splitlines()
    new_mod_rs = []

    required_modules = []

    ignore_lines = False

    for line in mod_rs:
        if line.strip() == 'Serverbound => {':
            new_mod_rs.append(line)
            if direction == 'serverbound':
                ignore_lines = True
                for packet_id, packet_class_name in zip(packet_ids, packet_class_names):
                    new_mod_rs.append(
                        make_packet_mod_rs_line(packet_id, packet_class_name)
                    )
                    required_modules.append(packet_class_name)
            else:
                ignore_lines = False
            continue
        elif line.strip() == 'Clientbound => {':
            new_mod_rs.append(line)
            if direction == 'clientbound':
                ignore_lines = True
                for packet_id, packet_class_name in zip(packet_ids, packet_class_names):
                    new_mod_rs.append(
                        make_packet_mod_rs_line(packet_id, packet_class_name)
                    )
                    required_modules.append(packet_class_name)
            else:
                ignore_lines = False
            continue
        elif line.strip() in ('}', '},'):
            ignore_lines = False
        elif line.strip().startswith('pub mod '):
            continue

        if not ignore_lines:
            new_mod_rs.append(line)
            # 0x00: clientbound_status_response_packet::ClientboundStatusResponsePacket,
            if line.strip().startswith('0x'):
                required_modules.append(
                    line.strip().split(':')[1].split('::')[0].strip())

    for i, required_module in enumerate(required_modules):
        if required_module not in mod_rs:
            new_mod_rs.insert(i, f'pub mod {required_module};')

    with open(mod_rs_dir, 'w') as f:
        f.write('\n'.join(new_mod_rs))


def get_packets(direction: str, state: str):
    mod_rs_dir = get_dir_location(
        f'../azalea-protocol/src/packets/{state}/mod.rs')
    with open(mod_rs_dir, 'r') as f:
        mod_rs = f.read().splitlines()

    in_serverbound = False
    in_clientbound = False

    packet_ids: list[int] = []
    packet_class_names: list[str] = []

    for line in mod_rs:
        if line.strip() == 'Serverbound => {':
            in_serverbound = True
            continue
        elif line.strip() == 'Clientbound => {':
            in_clientbound = True
            continue
        elif line.strip() in ('}', '},'):
            if (in_serverbound and direction == 'serverbound') or (in_clientbound and direction == 'clientbound'):
                break
            in_serverbound = in_clientbound = False
            continue

        if line.strip() == '' or line.strip().startswith('//') or (not in_serverbound and direction == 'serverbound') or (not in_clientbound and direction == 'clientbound'):
            continue

        line_packet_id_hex = line.strip().split(':')[0]
        assert line_packet_id_hex.startswith('0x')
        line_packet_id = int(line_packet_id_hex[2:], 16)
        packet_ids.append(line_packet_id)

        packet_class_name = line.strip().split(':')[1].strip()
        packet_class_names.append(packet_class_name)

    return packet_ids, packet_class_names


def burger_instruction_to_code(instruction: dict, generated_packet_code: list[str], mappings: Mappings, obfuscated_class_name: str, uses: set):
    field_type = instruction['type']
    obfuscated_field_name = instruction['field']
    field_name = mappings.get_field(
        obfuscated_class_name, obfuscated_field_name) or mappings.get_field(
        obfuscated_class_name.split('$')[0], obfuscated_field_name)

    field_type_rs, is_var, instruction_uses = burger_type_to_rust_type(
        field_type, field_name)

    field_comment = None
    if '.' in obfuscated_field_name or ' ' in obfuscated_field_name or '(' in obfuscated_field_name:
        field_type_rs, obfuscated_field_name, field_comment = burger_field_to_type(
            obfuscated_field_name, mappings, obfuscated_class_name)
        if not field_type_rs:
            generated_packet_code.append(f'// TODO: {instruction}')
            return
        # try to get the field name again with the new stuff we know
        field_name = mappings.get_field(
            obfuscated_class_name, obfuscated_field_name) or mappings.get_field(
            obfuscated_class_name.split('$')[0], obfuscated_field_name)

    if not field_name:
        generated_packet_code.append(
            f'// TODO: unknown field {instruction}')
        return

    if is_var:
        generated_packet_code.append('#[var]')
    line = f'pub {to_snake_case(field_name)}: {field_type_rs},'
    if field_comment:
        line += f' // {field_comment}'
    generated_packet_code.append(line)
    uses.update(instruction_uses)


def burger_field_to_type(field, mappings: Mappings, obfuscated_class_name: str) -> tuple[Optional[str], str, Optional[str]]:
    # match `(x) ? 1 : 0`
    match = re.match(r'\((.*)\) \? 1 : 0', field)
    if match:
        return ('bool', match.group(1), None)
    match = re.match(r'^\w+\.\w+\(\)$', field)
    if match:
        obfuscated_first = field.split('.')[0]
        obfuscated_second = field.split('.')[1].split('(')[0]
        first = mappings.get_field(obfuscated_class_name, obfuscated_first)
        first_type = mappings.get_field_type(
            obfuscated_class_name, obfuscated_first)
        second = mappings.get_method(
            mappings.get_class_from_deobfuscated_name(first_type), obfuscated_second, '')
        first_type_short = first_type.split('.')[-1]
        return (first_type_short, obfuscated_first, f'TODO: Does {first_type_short}::{second}, may not be implemented')
    return None, field, None


def change_packet_ids(id_map: dict[int, int], direction: str, state: str):
    existing_packet_ids, existing_packet_class_names = get_packets(
        direction, state)

    new_packet_ids = []

    for packet_id in existing_packet_ids:
        new_packet_id = id_map.get(packet_id, packet_id)
        new_packet_ids.append(new_packet_id)

    set_packets(new_packet_ids, existing_packet_class_names, direction, state)


def remove_packet_ids(removing_packet_ids: list[int], direction: str, state: str):
    existing_packet_ids, existing_packet_class_names = get_packets(
        direction, state)

    new_packet_ids = []
    new_packet_class_names = []

    for packet_id, packet_class_name in zip(existing_packet_ids, existing_packet_class_names):
        if packet_id in removing_packet_ids:
            try:
                os.remove(
                    f'../azalea-protocol/src/packets/{state}/{packet_class_name}.rs')
            except:
                pass
        else:
            new_packet_ids.append(packet_id)
            new_packet_class_names.append(packet_class_name)

    set_packets(new_packet_ids, new_packet_class_names, direction, state)


def are_packet_instructions_identical(old_packet, new_packet):
    old_packet = old_packet or []
    new_packet = new_packet or []

    if len(old_packet) != len(new_packet):
        return False

    for old_field, new_field in zip(old_packet, new_packet):
        if old_field['operation'] != new_field['operation']:
            return False
        if new_field['operation'] == 'write':
            if burger_type_to_rust_type(old_field.get('type')) != burger_type_to_rust_type(new_field.get('type')):
                return False
        else:
            # comparing is too complicated here since it's possible the type has variables
            # so we just don't
            pass

        if 'instructions' in old_field and 'instructions' in new_field:
            if not are_packet_instructions_identical(old_field['instructions'], new_field['instructions']):
                return False

    return True
