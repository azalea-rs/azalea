pub mod attributes;
mod data;
mod dimensions;
pub mod metadata;

use self::attributes::{AttributeInstance, AttributeModifiers};
pub use self::metadata::EntityMetadata;
use crate::World;
use azalea_block::BlockState;
use azalea_core::{BlockPos, Vec3, AABB};
pub use data::*;
pub use dimensions::*;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use uuid::Uuid;

/// A reference to an entity in a world.
#[derive(Debug)]
pub struct Entity<'d, D = &'d mut World> {
    /// The world this entity is in.
    pub world: D,
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub data: NonNull<EntityData>,
    _marker: PhantomData<&'d ()>,
}

impl<'d, D: Deref<Target = World>> Entity<'d, D> {
    pub fn new(world: D, id: u32, data: NonNull<EntityData>) -> Self {
        // TODO: have this be based on the entity type
        Self {
            world,
            id,
            data,
            _marker: PhantomData,
        }
    }
}

impl<'d, D: DerefMut<Target = World>> Entity<'d, D> {
    /// Sets the position of the entity. This doesn't update the cache in
    /// azalea-world, and should only be used within azalea-world!
    ///
    /// # Safety
    /// Cached position in the world must be updated.
    pub unsafe fn move_unchecked(&mut self, new_pos: Vec3) {
        self.pos = new_pos;
        let bounding_box = self.make_bounding_box();
        self.bounding_box = bounding_box;
    }

    pub fn set_rotation(&mut self, y_rot: f32, x_rot: f32) {
        self.y_rot = y_rot % 360.0;
        self.x_rot = x_rot.clamp(-90.0, 90.0) % 360.0;
        // TODO: minecraft also sets yRotO and xRotO to xRot and yRot ... but idk what they're used for so
    }

    pub fn move_relative(&mut self, speed: f32, acceleration: &Vec3) {
        let input_vector = self.input_vector(speed, acceleration);
        self.delta += input_vector;
    }

    pub fn input_vector(&self, speed: f32, acceleration: &Vec3) -> Vec3 {
        let distance = acceleration.length_squared();
        if distance < 1.0E-7 {
            return Vec3::default();
        }
        let acceleration = if distance > 1.0 {
            acceleration.normalize()
        } else {
            *acceleration
        }
        .scale(speed as f64);
        let y_rot = f32::sin(self.y_rot * 0.017453292f32);
        let x_rot = f32::cos(self.y_rot * 0.017453292f32);
        Vec3 {
            x: acceleration.x * (x_rot as f64) - acceleration.z * (y_rot as f64),
            y: acceleration.y,
            z: acceleration.z * (x_rot as f64) + acceleration.x * (y_rot as f64),
        }
    }

    /// Apply the given metadata items to the entity. Everything that isn't
    /// included in items will be left unchanged. If an error occured, None
    /// will be returned.
    ///
    /// TODO: this should be changed to have a proper error.
    pub fn apply_metadata(&mut self, items: &Vec<EntityDataItem>) -> Option<()> {
        for item in items {
            self.metadata.set_index(item.index, item.value.clone())?;
        }
        Some(())
    }
}

impl<'d, D: Deref<Target = World>> Entity<'d, D> {
    #[inline]
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn make_bounding_box(&self) -> AABB {
        self.dimensions.make_bounding_box(self.pos())
    }

    /// Get the position of the block below the entity, but a little lower.
    pub fn on_pos_legacy(&self) -> BlockPos {
        self.on_pos(0.2)
    }

    // int x = Mth.floor(this.position.x);
    // int y = Mth.floor(this.position.y - (double)var1);
    // int z = Mth.floor(this.position.z);
    // BlockPos var5 = new BlockPos(x, y, z);
    // if (this.level.getBlockState(var5).isAir()) {
    //    BlockPos var6 = var5.below();
    //    BlockState var7 = this.level.getBlockState(var6);
    //    if (var7.is(BlockTags.FENCES) || var7.is(BlockTags.WALLS) || var7.getBlock() instanceof FenceGateBlock) {
    //       return var6;
    //    }
    // }
    // return var5;
    pub fn on_pos(&self, offset: f32) -> BlockPos {
        let x = self.pos().x.floor() as i32;
        let y = (self.pos().y - offset as f64).floor() as i32;
        let z = self.pos().z.floor() as i32;
        let pos = BlockPos { x, y, z };

        // TODO: check if block below is a fence, wall, or fence gate
        let block_pos = pos.down(1);
        let block_state = self.world.get_block_state(&block_pos);
        if block_state == Some(BlockState::Air) {
            let block_pos_below = block_pos.down(1);
            let block_state_below = self.world.get_block_state(&block_pos_below);
            if let Some(_block_state_below) = block_state_below {
                // if block_state_below.is_fence()
                //     || block_state_below.is_wall()
                //     || block_state_below.is_fence_gate()
                // {
                //     return block_pos_below;
                // }
            }
        }

        pos
    }
}

// impl<
//         'd,
//         D: DerefMut<Target = World> + Deref<Target = World>,
//         D2: Deref<Target = World>,
//     > From<Entity<'d, D>> for Entity<'d, D2>
// {
//     fn from(entity: Entity<'d, D>) -> Entity<'d, D> {
//         Entity {
//             world: entity.world,
//             id: entity.id,
//             data: entity.data,
//             _marker: PhantomData,
//         }
//     }
// }

impl<D: DerefMut<Target = World>> DerefMut for Entity<'_, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}

impl<D: Deref<Target = World>> Deref for Entity<'_, D> {
    type Target = EntityData;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

#[derive(Debug)]
pub struct EntityData {
    pub uuid: Uuid,
    /// The position of the entity right now.
    /// This can be changde with unsafe_move, but the correct way is with world.move_entity
    pos: Vec3,
    /// The position of the entity last tick.
    pub last_pos: Vec3,
    pub delta: Vec3,

    /// X acceleration.
    pub xxa: f32,
    /// Y acceleration.
    pub yya: f32,
    /// Z acceleration.
    pub zza: f32,

    pub x_rot: f32,
    pub y_rot: f32,

    pub x_rot_last: f32,
    pub y_rot_last: f32,

    pub on_ground: bool,
    pub last_on_ground: bool,

    /// The width and height of the entity.
    pub dimensions: EntityDimensions,
    /// The bounding box of the entity. This is more than just width and height, unlike dimensions.
    pub bounding_box: AABB,

    /// Whether the entity will try to jump every tick
    /// (equivalent to the space key being held down in vanilla).
    pub jumping: bool,

    pub has_impulse: bool,

    /// Stores some extra data about the entity, including the entity type.
    pub metadata: EntityMetadata,

    /// The attributes and modifiers that the entity has (for example, speed).
    pub attributes: AttributeModifiers,
}

impl EntityData {
    pub fn new(uuid: Uuid, pos: Vec3, metadata: EntityMetadata) -> Self {
        let dimensions = EntityDimensions {
            width: 0.6,
            height: 1.8,
        };

        Self {
            uuid,
            pos,
            last_pos: pos,
            delta: Vec3::default(),

            xxa: 0.,
            yya: 0.,
            zza: 0.,

            x_rot: 0.,
            y_rot: 0.,

            y_rot_last: 0.,
            x_rot_last: 0.,

            on_ground: false,
            last_on_ground: false,

            // TODO: have this be based on the entity type
            bounding_box: dimensions.make_bounding_box(&pos),
            dimensions,

            has_impulse: false,

            jumping: false,

            metadata,

            attributes: AttributeModifiers {
                // TODO: do the correct defaults for everything, some entities have different defaults
                speed: AttributeInstance::new(0.1),
            },
        }
    }

    /// Get the position of the entity in the world.
    #[inline]
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    /// Convert this &self into a (mutable) pointer.
    ///
    /// # Safety
    /// The entity MUST exist for at least as long as this pointer exists.
    pub unsafe fn as_ptr(&self) -> NonNull<EntityData> {
        // this is cursed
        NonNull::new_unchecked(self as *const EntityData as *mut EntityData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_mut_entity_to_ref_entity() {
        let mut world = World::default();
        let uuid = Uuid::from_u128(100);
        world.add_entity(
            0,
            EntityData::new(
                uuid,
                Vec3::default(),
                EntityMetadata::Player(metadata::Player::default()),
            ),
        );
        let entity: Entity = world.entity_mut(0).unwrap();
        let entity_ref = Entity::from(entity);
        assert_eq!(entity_ref.uuid, uuid);
    }
}
