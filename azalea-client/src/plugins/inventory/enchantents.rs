use azalea_core::{registry_holder::RegistryHolder, resource_location::ResourceLocation};
use azalea_entity::Attributes;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_ecs::system::{Query, Res};

pub fn update_attributes_for_enchantments(
    mut query: Query<(&InstanceName, &mut Attributes)>,
    instance_container: Res<InstanceContainer>,
) {
    for (instance_name, mut attributes) in query.iter_mut() {
        let Some(instance) = instance_container.get(instance_name) else {
            continue;
        };
        let instance = instance.read();
        let registries = &instance.registries;

        println!("enchantments {:?}", registries.enchantment);
    }
}
