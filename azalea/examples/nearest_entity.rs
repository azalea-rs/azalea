use azalea::{
    ClientBuilder,
    bot::{Bot, LookAtEvent},
    nearest_entity::EntityFinder,
};
use azalea_client::Account;
use azalea_core::tick::GameTick;
use azalea_entity::{
    LocalEntity, Position,
    dimensions::EntityDimensions,
    metadata::{ItemItem, Player},
};
use bevy_app::Plugin;
use bevy_ecs::{
    prelude::{Entity, MessageWriter},
    query::With,
    system::Query,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let account = Account::offline("bot");

    ClientBuilder::new()
        .add_plugins(LookAtStuffPlugin)
        .start(account, "localhost")
        .await
        .unwrap();
}

pub struct LookAtStuffPlugin;
impl Plugin for LookAtStuffPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(GameTick, (look_at_everything, log_nearby_item_drops));
    }
}

fn look_at_everything(
    bots: Query<Entity, (With<LocalEntity>, With<Player>)>,
    entities: EntityFinder,
    entity_positions: Query<(&Position, Option<&EntityDimensions>)>,
    mut look_at_event: MessageWriter<LookAtEvent>,
) {
    for bot_id in bots.iter() {
        let Some(entity) = entities.nearest_to_entity(bot_id, 16.0) else {
            continue;
        };

        let (position, dimensions) = entity_positions.get(entity).unwrap();

        let mut look_target = **position;
        if let Some(dimensions) = dimensions {
            look_target.y += dimensions.eye_height as f64;
        }

        look_at_event.write(LookAtEvent {
            entity: bot_id,
            position: look_target,
        });
    }
}

fn log_nearby_item_drops(
    bots: Query<Entity, With<Bot>>,
    entities: EntityFinder<With<ItemItem>>,
    item_drops: Query<&ItemItem>,
) {
    for bot_id in bots.iter() {
        for (entity, distance) in entities.nearby_entities_to_entity(bot_id, 8.0) {
            let item_drop = item_drops.get(entity).unwrap();
            let kind = item_drop.kind();

            println!("Bot {bot_id:?} can see an {kind:?} {distance:.1} meters away.");
        }
    }
}
