from lib.utils import to_camel_case, to_snake_case, get_dir_location, upper_first_letter
from lib.mappings import Mappings
from typing import Optional
import re

METADATA_RS_DIR = get_dir_location(
    '../azalea-world/src/entity/metadata.rs')


def generate_entity_metadata(burger_entity_data: dict, mappings: Mappings):
    # TODO: auto generate this and use it for generating the EntityDataValue enum
    metadata_types = [
        { 'name': 'Byte', 'type': 'u8' },
        { 'name': 'Int', 'type': 'i32' },
        { 'name': 'Float', 'type': 'f32' },
        { 'name': 'String', 'type': 'String' },
        { 'name': 'Component', 'type': 'Component' },
        { 'name': 'OptionalComponent', 'type': 'Option<Component>' },
        { 'name': 'ItemStack', 'type': 'Slot' },
        { 'name': 'Boolean', 'type': 'bool' },
        { 'name': 'Rotations', 'type': 'Rotations' },
        { 'name': 'BlockPos', 'type': 'BlockPos' },
        { 'name': 'OptionalBlockPos', 'type': 'Option<BlockPos>' },
        { 'name': 'Direction', 'type': 'Direction' },
        { 'name': 'OptionalUuid', 'type': 'Option<Uuid>' },
        { 'name': 'OptionalBlockState', 'type': 'Option<i32>' },
        { 'name': 'CompoundTag', 'type': 'azalea_nbt::Tag' },
        { 'name': 'Particle', 'type': 'Particle' },
        { 'name': 'VillagerData', 'type': 'VillagerData' },
        { 'name': 'OptionalUnsignedInt', 'type': 'Option<u32>' },
        { 'name': 'Pose', 'type': 'Pose' },
        { 'name': 'CatVariant', 'type': 'azalea_registry::CatVariant' },
        { 'name': 'FrogVariant', 'type': 'azalea_registry::FrogVariant' },
        { 'name': 'GlobalPos', 'type': 'GlobalPos' },
        { 'name': 'PaintingVariant', 'type': 'azalea_registry::PaintingVariant' }

    ]

    code = []
    code.append('// This file is generated from codegen/lib/code/entity.py')
    code.append('')
    code.append('use super::{EntityDataValue, Rotations, VillagerData, Pose};')
    code.append('use azalea_chat::Component;')
    code.append('use azalea_core::{BlockPos, Direction, Particle, Slot};')
    code.append('use std::collections::VecDeque;')
    code.append('use uuid::Uuid;')
    code.append('')

    for entity_id, entity_data in burger_entity_data.items():
        entity_parents = get_entity_parents(entity_id, burger_entity_data)
        entity_metadata = get_entity_metadata(entity_id, burger_entity_data)
        entity_metadata_names = get_entity_metadata_names(entity_id, burger_entity_data, mappings)
        
        struct_name: str = upper_first_letter(to_camel_case(entity_parents[0].replace('~', '')))
        parent_struct_name: Optional[str] = upper_first_letter(to_camel_case(entity_parents[1].replace('~', ''))) if (len(entity_parents) >= 2) else None
        if parent_struct_name:
            parent_field_name = to_snake_case(parent_struct_name)
        
        print()
        print(entity_parents, entity_metadata, entity_metadata_names)

        reader_code = []
        writer_code = []
        field_names = []

        code.append(f'pub struct {struct_name} {{')

        if parent_struct_name:
            code.append(f'pub {parent_field_name}: {parent_struct_name},')
        for index, name_or_bitfield in entity_metadata_names.items():
            if isinstance(name_or_bitfield, str):
                name = name_or_bitfield
                if name == 'type':
                    name = 'kind'
                field_names.append(name)
                type_id = next(filter(lambda i: i['index'] == index, entity_metadata))['type_id']
                metadata_type_data = metadata_types[type_id]
                rust_type = metadata_type_data['type']
                type_name = metadata_type_data['name']
                code.append(f'pub {name}: {rust_type},')

                type_name_field = to_snake_case(type_name)
                reader_code.append(f'let {name} = metadata.pop_front()?.as_{type_name_field}()?.clone();')
                writer_code.append(f'metadata.push(EntityDataValue::{type_name}(self.{name}.clone()));')
            else:
                reader_code.append('let bitfield = *metadata.pop_front()?.as_byte()?;')
                writer_code.append('let mut bitfield = 0u8;')
                for mask, name in name_or_bitfield.items():
                    if name == 'type':
                        name = 'kind'

                    field_names.append(name)
                    code.append(f'pub {name}: bool,')
                    reader_code.append(f'let {name} = bitfield & {mask} != 1;')
                    writer_code.append(f'if self.{name} {{ bitfield &= {mask}; }}')
                writer_code.append('metadata.push(EntityDataValue::Byte(bitfield));')
        
        code.append('}')
        code.append('')

        code.append(f'impl {struct_name} {{')

        code.append('pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {')
        code.extend(reader_code)
        
        self_args = []
        if parent_struct_name:
            self_args.append(f'{parent_field_name}: {parent_struct_name}::read(metadata)?')
        self_args.extend(field_names)
        code.append(f'Some(Self {{ {",".join(self_args)} }})')
        code.append('}')
        code.append('')

        code.append('pub fn write(&self) -> Vec<EntityDataValue> {')
        code.append('let mut metadata = Vec::new();')
        code.extend(writer_code)
        code.append('metadata')
        code.append('}')

        code.append('}')
    
    with open(METADATA_RS_DIR, 'w') as f:
        f.write('\n'.join(code))

def get_entity_parents(entity_id: str, burger_entity_data: dict):
    parents = []
    while entity_id:
        parents.append(entity_id)
        entity_id = get_entity_parent(entity_id, burger_entity_data)
    return parents
def get_entity_parent(entity_id: str, burger_entity_data: dict):
    entity_metadata = burger_entity_data[entity_id]['metadata']
    first_metadata = entity_metadata[0]
    return first_metadata.get('entity')

def get_entity_metadata(entity_id: str, burger_entity_data: dict):
    entity_metadata = burger_entity_data[entity_id]['metadata']
    entity_useful_metadata = []
    for metadata_item in entity_metadata:
        if 'data' in metadata_item:
            for metadata_attribute in metadata_item['data']:
                entity_useful_metadata.append({
                    'index': metadata_attribute['index'],
                    'type_id': metadata_attribute['serializer_id'],
                })
    return entity_useful_metadata

def get_entity_metadata_names(entity_id: str, burger_entity_data: dict, mappings: Mappings):
    entity_metadata = burger_entity_data[entity_id]['metadata']
    mapped_metadata_names = {}

    for metadata_item in entity_metadata:
        if 'data' in metadata_item:
            obfuscated_class = metadata_item['class']
            mojang_class = mappings.get_class(obfuscated_class)

            first_byte_index = None

            for metadata_attribute in metadata_item['data']:
                obfuscated_field = metadata_attribute['field']
                mojang_field = mappings.get_field(obfuscated_class, obfuscated_field)
                pretty_mojang_name = prettify_mojang_field(mojang_field)
                mapped_metadata_names[metadata_attribute['index']] = pretty_mojang_name

                if metadata_attribute['serializer'] == 'Byte' and first_byte_index is None:
                    first_byte_index = metadata_attribute['index']
            
            if metadata_item['bitfields'] and first_byte_index is not None:
                clean_bitfield = {}
                for bitfield_item in metadata_item['bitfields']:
                    bitfield_item_obfuscated_class = bitfield_item.get('class', obfuscated_class)
                    mojang_bitfield_item_name = mappings.get_method(bitfield_item_obfuscated_class, bitfield_item['method'], '')
                    bitfield_item_name = prettify_mojang_method(mojang_bitfield_item_name)
                    bitfield_hex_mask = hex(bitfield_item['mask'])
                    clean_bitfield[bitfield_hex_mask] = bitfield_item_name
                mapped_metadata_names[first_byte_index] = clean_bitfield
    return mapped_metadata_names
            

def prettify_mojang_field(mojang_field: str):
    # mojang names are like "DATA_AIR_SUPPLY" and that's ugly
    better_name = mojang_field
    if better_name.startswith('DATA_'):
        better_name = better_name[5:]
    
    # remove the weird "Id" from the end of names
    if better_name.endswith('_ID'):
        better_name = better_name[:-3]
    # remove the weird "id" from the front of names
    if better_name.startswith('ID_'):
        better_name = better_name[3:]
    
    return better_name.lower()

def prettify_mojang_method(mojang_method: str):
    better_name = mojang_method
    if better_name.endswith('()'):
        better_name = better_name[:-2]
    if re.match(r'is[A-Z]', better_name):
        better_name = better_name[2:]
    return to_snake_case(better_name)

