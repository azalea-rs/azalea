from lib.utils import padded_hex, to_snake_case, to_camel_case, get_dir_location
from lib.code.utils import burger_type_to_rust_type, write_packet_file
from lib.mappings import Mappings
from typing import Optional
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
        extra_code = []

        packet_derive_name = f'{to_camel_case(direction)}{to_camel_case(state)}Packet'

        generated_packet_code.append(
            f'#[derive(Clone, Debug, McBuf, {packet_derive_name})]')
        uses.add(f'azalea_protocol_macros::{packet_derive_name}')
        uses.add(f'azalea_buf::McBuf')

        obfuscated_class_name = packet['class'].split('.')[0]
        class_name = mappings.get_class(
            obfuscated_class_name).split('.')[-1]
        if '$' in class_name:
            class_name, extra_part = class_name.split('$')
            if class_name.endswith('Packet'):
                class_name = class_name[:-
                                        len('Packet')] + extra_part + 'Packet'

        generated_packet_code.append(
            f'pub struct {to_camel_case(class_name)} {{')

        # call burger_instruction_to_code for each instruction
        i = -1
        instructions = packet.get('instructions', [])
        while (i + 1) < len(instructions):
            i += 1

            if instructions[i]['operation'] == 'write':
                skip = burger_instruction_to_code(
                    instructions, i, generated_packet_code, mappings, obfuscated_class_name, uses, extra_code)
                if skip:
                    i += skip
            else:
                generated_packet_code.append(f'// TODO: {instructions[i]}')

        generated_packet_code.append('}')

        if uses:
            # empty line before the `use` statements
            generated_packet_code.insert(0, '')
        for use in uses:
            generated_packet_code.insert(0, f'use {use};')
        for line in extra_code:
            generated_packet_code.append(line)

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

    # ids are repeated
    assert len(packet_ids) == len(set(packet_ids))

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


def burger_instruction_to_code(instructions: list[dict], index: int, generated_packet_code: list[str], mappings: Mappings, obfuscated_class_name: str, uses: set, extra_code: list[str]) -> Optional[int]:
    '''
    Generate a field for an instruction, returns the number of instructions to skip (if any).
    '''
    instruction = instructions[index]
    next_instruction = instructions[index +
                                    1] if index + 1 < len(instructions) else None
    next_next_instruction = instructions[index +
                                         2] if index + 2 < len(instructions) else None

    is_var = False
    skip = 0
    field_type_rs = None
    field_comment = None

    # iterators
    if instruction['operation'] == 'write' and instruction['field'].endswith('.size()') and next_instruction and next_instruction['type'] == 'Iterator' and next_next_instruction and next_next_instruction['operation'] == 'loop':
        field_obfuscated_name = instruction['field'].split('.')[
            0]
        field_name = mappings.get_field(
            obfuscated_class_name, field_obfuscated_name)

        # figure out what kind of iterator it is
        loop_instructions = next_next_instruction['instructions']
        if len(loop_instructions) == 2:
            entry_type_rs, is_var, uses, extra_code = burger_type_to_rust_type(
                loop_instructions[1]['type'], None, loop_instructions[1], mappings, obfuscated_class_name)
            field_type_rs = f'Vec<{entry_type_rs}>'
        elif len(loop_instructions) == 3:
            is_map = loop_instructions[0]['type'].startswith(
                'Map.Entry<')
            if is_map:
                assert loop_instructions[1]['field'].endswith(
                    '.getKey()')
                assert loop_instructions[2]['field'].endswith(
                    '.getValue()')

                # generate the type for the key
                key_type_rs, is_key_var, key_uses, key_extra_code = burger_type_to_rust_type(
                    loop_instructions[1]['type'], None, loop_instructions[1], mappings, obfuscated_class_name)
                uses.update(key_uses)
                extra_code.extend(key_extra_code)

                # generate the type for the value
                value_type_rs, is_value_var, value_uses, value_extra_code = burger_type_to_rust_type(
                    loop_instructions[2]['type'], None, loop_instructions[2], mappings, obfuscated_class_name)
                uses.update(value_uses)
                extra_code.extend(value_extra_code)

                field_type_rs = f'HashMap<{key_type_rs}, {value_type_rs}>'
                uses.add('std::collections::HashMap')

                # only the key is var since the value can be made var in other ways
                is_var = is_key_var

        skip = 2  # skip the next 2 instructions

    # Option<T>
    elif instruction['operation'] == 'write' and (instruction['field'].endswith('.isPresent()') or instruction['field'].endswith(' != null')) and next_instruction and (next_instruction.get('condition', '').endswith('.isPresent()') or next_instruction.get('condition', '').endswith(' != null')):
        field_obfuscated_name = instruction['field'].split('.')[
            0].split(' ')[0]
        field_name = mappings.get_field(
            obfuscated_class_name, field_obfuscated_name)
        condition_instructions = next_instruction['instructions']

        condition_types_rs = []
        for condition_instruction in condition_instructions:
            condition_type_rs, is_var, this_uses, this_extra_code = burger_type_to_rust_type(
                condition_instruction['type'], None, condition_instruction, mappings, obfuscated_class_name)
            condition_types_rs.append(condition_type_rs)
            uses.update(this_uses)
            extra_code.extend(this_extra_code)
        field_type_rs = f'Option<({", ".join(condition_types_rs)})>' if len(
            condition_types_rs) != 1 else f'Option<{condition_types_rs[0]}>'
        skip = 1
    else:
        field_type = instruction['type']
        obfuscated_field_name = instruction['field']

        if obfuscated_field_name.startswith('(float)'):
            obfuscated_field_name = obfuscated_field_name[len('(float)'):]

        field_name = mappings.get_field(
            obfuscated_class_name, obfuscated_field_name) or mappings.get_field(
            obfuscated_class_name.split('$')[0], obfuscated_field_name)

        field_type_rs, is_var, instruction_uses, instruction_extra_code = burger_type_to_rust_type(
            field_type, field_name, instruction, mappings, obfuscated_class_name)

        if '.' in obfuscated_field_name or ' ' in obfuscated_field_name or '(' in obfuscated_field_name:
            field_type_rs2, obfuscated_field_name, field_comment = burger_field_to_type(
                obfuscated_field_name, mappings, obfuscated_class_name)
            if not field_type_rs2:
                generated_packet_code.append(f'// TODO: {instruction}')
                return
            # try to get the field name again with the new stuff we know
            field_name = mappings.get_field(
                obfuscated_class_name, obfuscated_field_name) or mappings.get_field(
                obfuscated_class_name.split('$')[0], obfuscated_field_name)
        uses.update(instruction_uses)
        extra_code.extend(instruction_extra_code)

    if not field_name:
        generated_packet_code.append(
            f'// TODO: unknown field {instruction}')
        return

    if is_var:
        generated_packet_code.append('#[var]')
    line = f'pub {to_snake_case(field_name)}: {field_type_rs or "todo!()"},'
    if field_comment:
        line += f' // {field_comment}'
    generated_packet_code.append(line)

    return skip


def burger_field_to_type(field, mappings: Mappings, obfuscated_class_name: str) -> tuple[Optional[str], str, Optional[str]]:
    '''
    Returns field_type_rs, obfuscated_field_name, field_comment
    '''
    # match `(x) ? 1 : 0`
    match = re.match(r'\((.*)\) \? 1 : 0', field)
    if match:
        return ('bool', match.group(1), None)
    match = re.match(r'^\w+\.\w+\(\)$', field)
    if match:
        print('field', field)
        obfuscated_first = field.split('.')[0]
        obfuscated_second = field.split('.')[1].split('(')[0]
        first = mappings.get_field(obfuscated_class_name, obfuscated_first)
        first_type = mappings.get_field_type(
            obfuscated_class_name, obfuscated_first)
        first_obfuscated_class_name: Optional[str] = mappings.get_class_from_deobfuscated_name(
            first_type)
        if first_obfuscated_class_name:
            try:
                second = mappings.get_method(
                    first_obfuscated_class_name, obfuscated_second, '')
            except:
                # if this happens then the field is probably from a super class
                second = obfuscated_second
        else:
            second = obfuscated_second
        first_type_short = first_type.split('.')[-1]
        return (first_type_short, obfuscated_first, f'TODO: Does {first_type_short}::{second}, may not be implemented')
    return None, field, None


def change_packet_ids(id_map: dict[int, int], direction: str, state: str):
    existing_packet_ids, existing_packet_class_names = get_packets(
        direction, state)

    new_packet_ids = []

    for packet_id in existing_packet_ids:
        new_packet_id = id_map.get(packet_id, packet_id)
        if new_packet_id in new_packet_ids:
            raise Exception('Two packets have the same id')
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
