pub mod collision;

use azalea_block::Block;
use azalea_core::{BlockPos, Vec3};
use azalea_world::entity::{EntityData, EntityMut};
use collision::{MovableEntity, MoverType};

pub trait HasPhysics {
    fn travel(&mut self, acceleration: &Vec3);
    fn ai_step(&mut self);
}

impl HasPhysics for EntityMut<'_> {
    /// Move the entity with the given acceleration while handling friction,
    /// gravity, collisions, and some other stuff.
    fn travel(&mut self, acceleration: &Vec3) {
        // if !self.is_effective_ai() && !self.is_controlled_by_local_instance() {
        //     // this.calculateEntityAnimation(this, this instanceof FlyingAnimal);
        //     return;
        // }

        let gravity: f64 = 0.08;

        // TODO: slow falling effect
        // let is_falling = self.delta.y <= 0.;

        // TODO: fluids

        // TODO: elytra

        let block_pos_below = get_block_pos_below_that_affects_movement(self);
        let block_friction =
            if let Some(block_state_below) = self.dimension.get_block_state(&block_pos_below) {
                let block_below: Box<dyn Block> = block_state_below.into();
                block_below.behavior().friction
            } else {
                unreachable!("Block below should be a real block.")
            };

        let inertia = if self.on_ground {
            block_friction * 0.91
        } else {
            0.91
        };
        let mut movement =
            handle_relative_friction_and_calculate_movement(self, acceleration, block_friction);

        movement.y -= gravity;

        // if (this.shouldDiscardFriction()) {
        //     this.setDeltaMovement(movement.x, yMovement, movement.z);
        // } else {
        //     this.setDeltaMovement(movement.x * (double)inertia, yMovement * 0.9800000190734863D, movement.z * (double)inertia);
        // }

        // if should_discard_friction(self) {
        if false {
            self.delta = movement;
        } else {
            self.delta = Vec3 {
                x: movement.x * inertia as f64,
                y: movement.y * 0.98f64,
                z: movement.z * inertia as f64,
            };
        }
    }

    /// applies air resistance, calls self.travel(), and some other random
    /// stuff.
    fn ai_step(&mut self) {
        // vanilla does movement interpolation here, doesn't really matter much for a bot though

        self.xxa *= 0.98;
        self.zza *= 0.98;

        self.travel(&Vec3 {
            x: self.xxa as f64,
            y: self.yya as f64,
            z: self.zza as f64,
        });
        // freezing
        // pushEntities
        // drowning damage
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

fn handle_relative_friction_and_calculate_movement(
    entity: &mut EntityMut,
    acceleration: &Vec3,
    block_friction: f32,
) -> Vec3 {
    entity.move_relative(get_speed(&*entity, block_friction), acceleration);
    // entity.delta = entity.handle_on_climbable(entity.delta);
    entity
        .move_colliding(&MoverType::Own, &entity.delta.clone())
        .expect("Entity should exist.");
    // let delta_movement = entity.delta;
    //   if ((entity.horizontalCollision || entity.jumping) && (entity.onClimbable() || entity.getFeetBlockState().is(Blocks.POWDER_SNOW) && PowderSnowBlock.canEntityWalkOnPowderSnow(entity))) {
    //      var3 = new Vec3(var3.x, 0.2D, var3.z);
    //   }
    // TODO: powdered snow

    entity.delta
}

// private float getFrictionInfluencedSpeed(float friction) {
//     return this.onGround ? this.getSpeed() * (0.21600002F / (friction * friction * friction)) : this.flyingSpeed;
// }
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
