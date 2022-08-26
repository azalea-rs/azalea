pub mod collision;

use azalea_block::Block;
use azalea_core::{BlockPos, Vec3};
use azalea_world::{
    entity::{EntityData, EntityMut},
    Dimension,
};
use collision::{MovableEntity, MoverType};

trait HasPhysics {
    fn travel(&self, acceleration: &Vec3, dimension: &Dimension) -> Result<(), ()>;
}

impl HasPhysics for EntityData {
    fn travel(&self, _acceleration: &Vec3, dimension: &Dimension) -> Result<(), ()> {
        // if !self.is_effective_ai() && !self.is_controlled_by_local_instance() {
        //     // this.calculateEntityAnimation(this, this instanceof FlyingAnimal);
        //     return;
        // }

        let _gravity: f64 = 0.08;

        let _is_falling = self.delta.ya <= 0.;

        // TODO: slow falling effect

        // TODO: fluids

        // TODO: elytra

        let block_pos_below = get_block_pos_below_that_affects_movement(self);
        let block_friction =
            if let Some(block_state_below) = dimension.get_block_state(&block_pos_below) {
                let block_below: Box<dyn Block> = block_state_below.into();
                block_below.behavior().friction
            } else {
                panic!();
            };

        let inertia = if self.on_ground {
            block_friction * 0.91
        } else {
            0.91
        };
        let movement =
            handle_relative_friction_and_calculate_movement(entity, acceleration, block_friction);

        Ok(())
    }
}

fn get_block_pos_below_that_affects_movement(entity: &EntityData) -> BlockPos {
    BlockPos::new(
        entity.pos().x as i32,
        // TODO: this uses bounding_box.min_y instead of position.y
        (entity.pos().y - 0.5f64) as i32,
        entity.pos().z as i32,
    )
}

// TODO: finish this
#[allow(dead_code)]
fn handle_relative_friction_and_calculate_movement(
    entity: &EntityMut,
    acceleration: &Vec3,
    block_friction: f64,
) -> Vec3 {
    entity.add_delta(get_speed(*entity, block_friction), acceleration);
    // entity.delta = entity.handle_on_climbable(entity.delta);
    entity.move_colliding(&MoverType::Own, &entity.delta.into());
    let delta_movement = entity.delta;
    //   if ((entity.horizontalCollision || entity.jumping) && (entity.onClimbable() || entity.getFeetBlockState().is(Blocks.POWDER_SNOW) && PowderSnowBlock.canEntityWalkOnPowderSnow(entity))) {
    //      var3 = new Vec3(var3.x, 0.2D, var3.z);
    //   }
    // TODO: powdered snow

    delta_movement
}

// private float getFrictionInfluencedSpeed(float friction) {
//     return this.onGround ? this.getSpeed() * (0.21600002F / (friction * friction * friction)) : this.flyingSpeed;
// }
// TODO: use this
#[allow(dead_code)]
fn get_speed(entity: &EntityData, friction: f32) -> f32 {
    // TODO: have speed & flying_speed fields in entity
    if entity.on_ground {
        let speed: f32 = 0.7;
        speed * (0.216f32 / (friction * friction * friction))
    } else {
        // entity.flying_speed
        0.02
    }
}
