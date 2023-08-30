"""Generates packets code for azalea-protocol"""
import logging
import re

from azalea_codegen.utils import Mappings, to_snake_case, to_camel_case, strip_suffix, strip_prefix, \
    exception_to_string, get_root_path
from lib import utils
from lib.utils import padded_hex

_LOGGER = logging.getLogger(__name__)


def _fix_state(state: str) -> str:
    # We call the states login, status and game.
    return {'PLAY': 'game'}.get(state, state.lower())


def generate_packet(mappings: Mappings, packet_data: dict) -> str:
    direction = packet_data['direction'].lower()
    state = _fix_state(packet_data['state'])
    generated_code = []

    # Get a nice class name for the packet.
    obf_class_name = strip_suffix(packet_data['class'], '.class')
    class_name = mappings.get_class_name(obf_class_name).split('.')[-1]

    if '$' in class_name:
        class_name, extra_part = class_name.split('$', 1)

        if class_name.endswith('Packet'):
            class_name = strip_suffix(class_name, 'Packet') + extra_part + 'Packet'

    # Generate struct definition.
    packet_derive_name = f'{to_camel_case(direction)}{to_camel_case(state)}Packet'
    generated_code.append(f'#[derive(Clone, Debug, azalea_buf::McBuf, azalea_protocol_macros::{packet_derive_name})]\n')

    struct_name = utils.to_camel_case(class_name)
    generated_code.append(f'pub struct {struct_name} {{\n')

    generated_extra = []
    instructions = packet_data.get('instructions', [])

    try:
        generated_fields = []
        i = 0

        while i < len(instructions):
            skipped, extra = _generate_struct_field(mappings, instructions, i, obf_class_name, generated_fields)
            i += skipped

            if extra:
                generated_extra.extend(extra)
                generated_extra.append('\n')

            generated_fields.append('\n')

        for line in ''.join(generated_fields).splitlines():
            generated_code.append(f'    {line}\n')

    except Exception as e:
        _LOGGER.warning(f'Exception during generation of packet {class_name} ({packet_data["id"]})', exc_info=True)

        lines = exception_to_string(e).splitlines()
        generated_code.extend(f'// TODO(codegen): Exception during packet generation! {lines.pop(0)}\n')

        for line in lines:
            generated_code.extend(f'// {line}\n')

    generated_code.append('}\n')
    generated_code.extend(generated_extra)

    # Write out packet code to file.
    generated_code = ''.join(generated_code)

    with open(get_root_path('azalea-protocol', 'src', 'packets', state, f'{to_snake_case(class_name)}.rs'), 'w') as f:
        f.write(''.join(generated_code))

    # Update mod.rs
    mod_rs_path = get_root_path('azalea-protocol', 'src', 'packets', state, 'mod.rs')

    with open(mod_rs_path, 'r') as f:
        mod_rs = f.read().splitlines()

    # If packet has not already been generated, this line will be missing.
    pub_mod_line = f'pub mod {to_snake_case(class_name)};'

    if pub_mod_line not in mod_rs:
        mod_rs.insert(0, pub_mod_line)
        packet_mod_rs_line = f'        {padded_hex(packet_data["id"])}: {to_snake_case(class_name)}::{to_camel_case(class_name)},'

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

            if line_packet_id > packet_data['id']:
                mod_rs.insert(i, packet_mod_rs_line)
                break

        with open(mod_rs_path, 'w') as f:
            f.write('\n'.join(mod_rs))


def _burger_type_to_rust_type(burger_type: str, field_name: str | None, instruction, mappings: Mappings,
                              obfuscated_class_name: str) -> (str, bool):
    """Converts a Burger type (`insn["type"]`) to a Rust one."""
    _BASIC_TYPE_SIZES = {
        'byte': ('>8', False),
        'short': ('>16', False),
        'int': ('>32', False),
        'long': ('>64', False),
        'float': ('f32', False),
        'double': ('f64', False),
        'varint': ('>32', True),
        'varlong': ('>64', True),
        'boolean': ('bool', False),
        'string': ('String', False),
        'chatcomponent': ('azalea_chat::FormattedText', False),
        'identifier': ('azalea_core::ResourceLocation', False),
        'uuid': ('uuid::Uuid', False),
        'position': ('azalea_core::BlockPos', False),
        'nbtcompound': ('azalea_nbt::Nbt', False),
        'itemstack': ('azalea_core::Slot', False),
        'metadata': ('azalea_entity::EntityMetadata', False),
        'bitset': ('todo', False),
        'abstract': ('todo', False),
        'enum': ('todo', False),
    }

    is_var = False
    extra_code = []  # extra code such as generated enums

    # Coordinates are always signed
    should_be_signed = field_name is not None and any(map(lambda w: w in {'x', 'y', 'z', 'xa', 'ya', 'za'},
                                                          to_snake_case(field_name).split('_')))

    if burger_type in _BASIC_TYPE_SIZES:
        rust_type, is_var = _BASIC_TYPE_SIZES[burger_type]
        rust_type = rust_type.replace('>', 'i' if should_be_signed else 'u')

    elif burger_type == 'bitset':
        if instruction:
            length = instruction['length']
            rust_type = f'todo!("burger_type_to_rust_type: fixed bitset of length {length}")'

        else:
            rust_type = 'todo!("burger_type_to_rust_type: fixed bitset")'

    elif burger_type == 'abstract':
        rust_type = 'todo!("burger_type_to_rust_type: abstract")'

    elif burger_type == 'enum':
        # Generate an enum type that matches the burger definition.
        enum_field = instruction['field']

        # enums with a.b() as the field
        if '.' in enum_field:
            enum_first_part_name = mappings.get_field_type(obfuscated_class_name, enum_field.split('.')[0])
            enum_first_part_obfuscated_name = mappings.get_obfuscated_class_name(enum_first_part_name)

            enum_name = mappings.get_method_return_type(enum_first_part_obfuscated_name,
                                                        enum_field.split('.')[1].split('(')[0], '')

            if enum_name is None:
                # Sometimes enums are fields instead of methods
                enum_name = mappings.get_field_type(enum_first_part_obfuscated_name,
                                                    enum_field.split('.')[1].split('(')[0])

            print('hm', enum_name)
            # TODO: ^

        else:
            enum_name = mappings.get_field_type(obfuscated_class_name, enum_field)

            if enum_name is None:
                enum_name = mappings.get_class_name(obfuscated_class_name)
                _LOGGER.warning(f'Failed to get enum field {obfuscated_class_name}.{enum_field} but continuing with '
                                f'{enum_name} anyways')

        enum_obfuscated_name = mappings.get_obfuscated_class_name(enum_name)

        enum_variants = []
        for obfuscated_field_name in mappings.field_names[enum_obfuscated_name]:
            field_name = mappings.get_field_name(enum_obfuscated_name, obfuscated_field_name)

            # get the type just to make sure it's actually a variant and not something else
            field_type = mappings.get_field_type(enum_obfuscated_name, obfuscated_field_name)

            if field_type != enum_name:
                continue

            enum_variants.append(field_name)

        rust_type = to_camel_case(enum_name.split('.')[-1].split('$')[-1])
        extra_code.append('')
        extra_code.append('#[derive(McBuf, Clone, Copy, Debug)]')
        extra_code.append(f'pub enum {rust_type} {{')

        for index, variant in enumerate(enum_variants):
            extra_code.append(
                f'    {to_camel_case(variant.lower())}={index},')

        extra_code.append('}')

    elif burger_type.endswith('[]'):
        # Handle lists.
        rust_type, is_var, extra_code = _burger_type_to_rust_type(strip_suffix(burger_type, '[]'), field_name,
                                                                  instruction,
                                                                  mappings, obfuscated_class_name)
        rust_type = f'Vec<{rust_type}>'

        # sometimes burger gives us a slightly incorrect type
        if mappings and instruction:
            if rust_type == 'Vec<u8>':
                field = strip_suffix(instruction['field'], '.copy()')
                array_type = mappings.get_field_type(obfuscated_class_name, field)

                if array_type is None:
                    return rust_type, is_var, extra_code

                if array_type == 'net.minecraft.network.FriendlyByteBuf':
                    rust_type = 'azalea_buf::UnsizedByteArray'

    else:
        rust_type = f'todo!("_burger_type_to_rust_type: Unsupported burger type: {burger_type}")'

    return rust_type, is_var, extra_code


def _handle_burger_field_expressions(mappings: Mappings, obf_class_name: str, obf_field_name: str) -> tuple[
    str | None, str, list[str]]:
    # match `(x) ? 1 : 0`
    match = re.match(r'\((.*)\) \? 1 : 0', obf_field_name)

    if match:
        return 'bool', match.group(1), []

    # Handle `.getId()` (mostly registries).
    match = re.search(r'\.getId\((.+)\)', obf_field_name)

    if match:
        obf_target_fields = obf_field_name.split('.')
        obf_target_fields.pop()  # Remove `.getId()`

        # Class.FIELD.OTHER_FIELD.getId()
        obf_target_class = obf_target_fields.pop(0)

        # Remap class name and fields.
        target_class = mappings.get_class_name(obf_target_class)
        target_fields = [mappings.get_field_name(obf_target_class, f) for f in obf_target_fields]

        # Handle specific registries.
        if target_class == 'net.minecraft.core.registries.BuiltInRegistries' and len(target_fields) == 1:
            registry_type = to_camel_case(target_fields[0].lower())

            return f'azalea_registry::{registry_type}', match.group(1), []

        elif target_class == 'net.minecraft.world.level.block.Block' and len(target_fields) == 1 and target_fields[
            0] == 'BLOCK_STATE_REGISTRY':
            return f'azalea_block::Block', match.group(1), []

    # Handle Class.method()
    match = re.match(r'^\w+\.+\w+\(\)$', obf_field_name)

    if match:
        obf_first = obf_field_name.split('.')[0]
        obf_second = obf_field_name.split('.')[1].split('(')[0]

        first_type = mappings.get_field_type(obf_class_name, obf_first)
        obf_first_class_name = mappings.get_obfuscated_class_name(first_type)

        if obf_first_class_name is None:
            second = obf_second

        else:
            second = mappings.get_method_name(obf_first_class_name, obf_second, '')

            if second is None:
                second = obf_second

        first_type_short = first_type.split('.')[-1]
        if second == 'byteValue':
            return first_type_short, obf_first, []

        return first_type_short, obf_first, [
            f'TODO(codegen): Calls {first_type_short}::{second}, may not be implemented']

    return None, obf_field_name, []


def _generate_struct_field(mappings: Mappings, instructions: list[dict], i: int, obf_class_name: str,
                           generated_code: list[str]) -> (int, list[str]):
    insn = instructions[i]
    insn_next = instructions[i + 1] if (i + 1) < len(instructions) else None
    insn_next_next = instructions[i + 2] if (i + 2) < len(instructions) else None

    extra_code = []
    is_var = False
    field_name = None
    field_type_rs = None
    field_comment = []
    skipped_instructions = 1

    if insn['operation'] == 'write' and insn['field'].endswith('.size()') \
            and insn_next and insn_next['type'] == 'Iterator' \
            and insn_next_next and insn_next_next['operation'] == 'loop':
        # Handle iterators.
        obf_field_name = insn['field'].split('.')[0]
        field_name = mappings.get_field_name(obf_class_name, obf_field_name)

        # figure out what kind of iterator it is
        loop_instructions = insn_next_next['instructions']

        if len(loop_instructions) == 2:
            entry_type_rs, is_var, extra_code = _burger_type_to_rust_type(loop_instructions[1]['type'], None,
                                                                          loop_instructions[1], mappings,
                                                                          obf_class_name)
            field_type_rs = f'Vec<{entry_type_rs}>'

        elif len(loop_instructions) == 3 and loop_instructions[0]['type'].startswith('Map.Entry<'):
            assert loop_instructions[1]['field'].endswith('.getKey()')
            assert loop_instructions[2]['field'].endswith('.getValue()')

            # Generate type for key.
            key_type_rs, is_key_var, key_extra_code = _burger_type_to_rust_type(loop_instructions[1]['type'], None,
                                                                                loop_instructions[1], mappings,
                                                                                obf_class_name)
            extra_code.extend(key_extra_code)

            # Generate type for value.
            value_type_rs, _, value_extra_code = _burger_type_to_rust_type(loop_instructions[2]['type'], None,
                                                                           loop_instructions[2], mappings,
                                                                           obf_class_name)
            extra_code.extend(value_extra_code)

            field_type_rs = f'std::collections::HashMap<{key_type_rs}, {value_type_rs}>'

            # We only care if the key is a var because the value is made var in other ways.
            is_var = is_key_var

        else:
            field_comment.append('DEBUG: Field appears to be an iterator, but could not infer type.')

        skipped_instructions = 3

    elif insn['operation'] == 'write' \
            and (insn['field'].endswith('.isPresent()') or insn['field'].endswith(' != null')) \
            and insn_next and (insn_next.get('condition', '').endswith('.isPresent()')
                               or insn_next.get('condition', '').endswith(' != null')):
        # An optional field, which we will convert to an `Option<T>`.
        obf_field_name = insn['field'].split('.')[0].split(' ')[0]
        field_name = mappings.get_field_name(obf_class_name, obf_field_name)

        # TODO: I don't know why this exists, mat wrote it
        if field_name is None:
            field_name = obf_field_name.split('/')[-1]

        if '<' in field_name:
            field_name = 'value'

        condition_instructions = insn_next['instructions']
        condition_types_rs = []

        for condition_instruction in condition_instructions:
            condition_type_rs, is_var, this_extra_code = _burger_type_to_rust_type(condition_instruction['type'], None,
                                                                                   condition_instruction, mappings,
                                                                                   obf_class_name)
            condition_types_rs.append(condition_type_rs)
            extra_code.extend(this_extra_code)

        if len(condition_types_rs) == 1:
            field_type_rs = f'Option<{condition_types_rs[0]}>'

        else:
            field_type_rs = f'Option<({", ".join(condition_types_rs)})>'

        skipped_instructions = 2

    else:
        # Just a normal field.
        burger_field_type = insn['type']
        obf_field_name = strip_prefix(insn['field'], '(float)')

        # If the field type is more complex than just an assignment, handle it specially.
        if '.' in obf_field_name or ' ' in obf_field_name or '(' in obf_field_name:
            field_type_rs, obf_field_name, field_to_type_comment = _handle_burger_field_expressions(mappings,
                                                                                                    obf_class_name,
                                                                                                    obf_field_name)

            field_comment.extend(field_to_type_comment)

            if field_type_rs is not None:
                # Get the deobfuscated field name.
                field_name = mappings.get_field_name(obf_class_name, obf_field_name) or \
                             mappings.get_field_name(obf_class_name.split('$')[0], obf_field_name)

                # TODO: I don't know why this exists, mat wrote it
                if field_name is None:
                    field_name = obf_field_name.split('/')[-1]

        else:
            # Just a regular field, remap it and then convert the type from a burger type to a rust one.
            field_name = mappings.get_field_name(obf_class_name, obf_field_name) or \
                         mappings.get_field_name(obf_class_name.split('$')[0], obf_field_name)

            field_type_rs, is_var, instruction_extra_code = _burger_type_to_rust_type(burger_field_type, field_name,
                                                                                      insn, mappings, obf_class_name)

            extra_code.extend(instruction_extra_code)

    # We failed to identify the field name.
    if field_name is None:
        field_comment.append(f'TODO(codegen): couldn\'t get name for field')
        field_comment.append('')
        field_comment.append('DEBUG: Remaining instructions are:')

        for remaining in instructions[i:]:
            field_comment.append(f'  {remaining}')

        generated_code.append('\n'.join(f'// {line}' for line in field_comment) + '\n')

    else:
        if is_var:
            # varint / varlong fields have an attribute added.
            generated_code.append('#[var]\n')

        # If we failed to generate a field type, we replace it with a `to_do!()`.
        if field_type_rs is None:
            field_comment.append('DEBUG: Remaining instructions are:')

            for remaining in instructions[i:]:
                field_comment.append(f'  {remaining}')

            field_type_rs = 'todo!("_generate_struct_field could not infer the type for this field")'

        # Generate the actual field definition and add it to the result.
        field_code = ''

        if field_comment:
            field_code += '\n'.join(f'// {line}' for line in field_comment)
            field_code += '\n'

        field_code += f'pub {to_snake_case(field_name)}: {field_type_rs},\n'
        generated_code.append(field_code)

    return skipped_instructions, extra_code
