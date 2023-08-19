use azalea::nearest_entity::EntityFinder;
use azalea::ClientBuilder;
use azalea::{Bot, LookAtEvent};
use azalea_client::Account;
use azalea_entity::metadata::{Item, ItemItem};
use azalea_entity::{EyeHeight, Position};
use bevy_app::{FixedUpdate, Plugin};
use bevy_ecs::{
    prelude::{Entity, EventWriter},
    query::With,
    system::Query,
};

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");

    ClientBuilder::new()
        .add_plugins(LookAtStuffPlugin)
        .start(account, "localhost:34071")
        .await
        .unwrap();
}

pub struct LookAtStuffPlugin;
impl Plugin for LookAtStuffPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(FixedUpdate, (look_at_everything, log_nearby_item_drops));
    }
}

fn look_at_everything(
    bots: Query<Entity, With<Bot>>,
    entities: EntityFinder,
    entity_positions: Query<(&Position, Option<&EyeHeight>)>,
    mut look_at_event: EventWriter<LookAtEvent>,
) {
    for bot_id in bots.iter() {
        let Some(entity) = entities.nearest_to_entity(bot_id, 16.0) else {
            continue;
        };

        let (position, eye_height) = entity_positions.get(entity).unwrap();

        let mut look_target = **position;
        if let Some(eye_height) = eye_height {
            look_target.y += **eye_height as f64;
        }

        look_at_event.send(LookAtEvent {
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

            println!(
                "Bot {:?} can see an {:?} {:.1} meters away.",
                bot_id, kind, distance
            );
        }
    }
}
