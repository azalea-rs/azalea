mod data;
mod dimensions;

use crate::Dimension;
use azalea_core::{BlockPos, Vec3, AABB};
pub use data::*;
pub use dimensions::*;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use uuid::Uuid;

#[derive(Debug)]
pub struct EntityRef<'d> {
    /// The dimension this entity is in.
    pub dimension: &'d Dimension,
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub data: &'d EntityData,
}

impl<'d> EntityRef<'d> {
    pub fn new(dimension: &'d Dimension, id: u32, data: &'d EntityData) -> Self {
        // TODO: have this be based on the entity type
        Self {
            dimension,
            id,
            data,
        }
    }
}

impl<'d> EntityRef<'d> {
    #[inline]
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn make_bounding_box(&self) -> AABB {
        self.dimensions.make_bounding_box(&self.pos())
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
        // let block_pos = pos.below();
        // let block_state = dimension.get_block_state(&block_pos);
        // if block_state == Some(BlockState::Air) {
        //     let block_pos_below = block_pos.below();
        //     let block_state_below = dimension.get_block_state(&block_pos_below);
        //     if let Some(block_state_below) = block_state_below {
        //         if block_state_below.is_fence()
        //             || block_state_below.is_wall()
        //             || block_state_below.is_fence_gate()
        //         {
        //             return block_pos_below;
        //         }
        //     }
        // }

        pos
    }
}

#[derive(Debug)]
pub struct EntityMut<'d> {
    /// The dimension this entity is in.
    pub dimension: &'d mut Dimension,
    /// The incrementing numerical id of the entity.
    pub id: u32,
    pub data: NonNull<EntityData>,
}

impl<'d> EntityMut<'d> {
    pub fn new(dimension: &'d mut Dimension, id: u32, data: NonNull<EntityData>) -> Self {
        Self {
            dimension,
            id,
            data,
        }
    }

    /// Sets the position of the entity. This doesn't update the cache in
    /// azalea-world, and should only be used within azalea-world!
    pub unsafe fn move_unchecked(&mut self, new_pos: Vec3) {
        self.pos = new_pos;
        let bounding_box = self.make_bounding_box();
        self.bounding_box = bounding_box;
    }

    pub fn set_rotation(&mut self, y_rot: f32, x_rot: f32) {
        self.y_rot = y_rot.clamp(-90.0, 90.0) % 360.0;
        self.x_rot = x_rot % 360.0;
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
}

impl<'d> EntityMut<'d> {
    #[inline]
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn make_bounding_box(&self) -> AABB {
        self.dimensions.make_bounding_box(&self.pos())
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
        // let block_pos = pos.below();
        // let block_state = dimension.get_block_state(&block_pos);
        // if block_state == Some(BlockState::Air) {
        //     let block_pos_below = block_pos.below();
        //     let block_state_below = dimension.get_block_state(&block_pos_below);
        //     if let Some(block_state_below) = block_state_below {
        //         if block_state_below.is_fence()
        //             || block_state_below.is_wall()
        //             || block_state_below.is_fence_gate()
        //         {
        //             return block_pos_below;
        //         }
        //     }
        // }

        pos
    }
}

impl<'d> From<EntityMut<'d>> for EntityRef<'d> {
    fn from(entity: EntityMut<'d>) -> EntityRef<'d> {
        let data = unsafe { entity.data.as_ref() };
        EntityRef {
            dimension: entity.dimension,
            id: entity.id,
            data,
        }
    }
}

impl Deref for EntityMut<'_> {
    type Target = EntityData;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl DerefMut for EntityMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}

impl Deref for EntityRef<'_> {
    type Target = EntityData;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

#[derive(Debug)]
pub struct EntityData {
    pub uuid: Uuid,
    /// The position of the entity right now.
    /// This can be changde with unsafe_move, but the correct way is with dimension.move_entity
    pos: Vec3,
    /// The position of the entity last tick.
    pub last_pos: Vec3,
    pub delta: Vec3,

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
}

impl EntityData {
    pub fn new(uuid: Uuid, pos: Vec3) -> Self {
        let dimensions = EntityDimensions {
            width: 0.8,
            height: 1.8,
        };

        Self {
            uuid,
            pos,
            last_pos: pos,
            delta: Vec3::default(),

            x_rot: 0.0,
            y_rot: 0.0,

            y_rot_last: 0.0,
            x_rot_last: 0.0,

            on_ground: false,
            last_on_ground: false,

            // TODO: have this be based on the entity type
            bounding_box: dimensions.make_bounding_box(&pos),
            dimensions,
        }
    }

    #[inline]
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub(crate) unsafe fn as_ptr(&mut self) -> NonNull<EntityData> {
        NonNull::new_unchecked(self as *mut EntityData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_mut_entity_to_ref_entity() {
        let mut dim = Dimension::default();
        let uuid = Uuid::from_u128(100);
        dim.add_entity(0, EntityData::new(uuid, Vec3::default()));
        let entity: EntityMut = dim.entity_mut(0).unwrap();
        let entity_ref: EntityRef = entity.into();
        assert_eq!(entity_ref.uuid, uuid);
    }
}
