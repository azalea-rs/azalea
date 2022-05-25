from .utils import burger_type_to_rust_type, write_packet_file
from ..utils import padded_hex, to_snake_case, to_camel_case
from ..mappings import Mappings


def make_packet_mod_rs_line(packet_id: int, packet_class_name: str):
    return f'        {padded_hex(packet_id)}: {to_snake_case(packet_class_name)}::{to_camel_case(packet_class_name)},'


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


def set_packet_ids(packet_ids: list, packet_class_names: list, direction: str, state: str):
    assert len(packet_ids) == len(packet_class_names)

    mod_rs_dir = f'../azalea-protocol/src/packets/{state}/mod.rs'
    with open(mod_rs_dir, 'r') as f:
        mod_rs = f.read().splitlines()
    new_mod_rs = []

    ignore_lines = False

    for line in mod_rs:
        if line.strip() == 'Serverbound => {':
            if direction == 'serverbound':
                ignore_lines = True
                for packet_id, packet_class_name in zip(packet_ids, packet_class_names):
                    new_mod_rs.append(
                        make_packet_mod_rs_line(packet_id, packet_class_name)
                    )
            else:
                ignore_lines = False
        elif line.strip() == 'Clientbound => {':
            if direction == 'serverbound':
                ignore_lines = True
                for packet_id, packet_class_name in zip(packet_ids, packet_class_names):
                    new_mod_rs.append(
                        make_packet_mod_rs_line(packet_id, packet_class_name)
                    )
            else:
                ignore_lines = False
        elif line.strip() in ('}', '},'):
            ignore_lines = False

        if not ignore_lines:
            new_mod_rs.append(line)

    with open(mod_rs_dir, 'w') as f:
        f.write('\n'.join(new_mod_rs))
