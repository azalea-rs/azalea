use azalea_entity::Entity;
use azalea_world::Dimension;

use crate::AABB;

trait CollisionGetter {
    fn get_block_collisions<'a>(
        &'a self,
        entity: Option<&Entity>,
        aabb: AABB,
    ) -> BlockCollisions<'a>;
}

impl CollisionGetter for Dimension {
    fn get_block_collisions<'a>(
        &'a self,
        entity: Option<&Entity>,
        aabb: AABB,
    ) -> BlockCollisions<'a> {
        BlockCollisions::new(self, entity, aabb)
    }
}

pub struct BlockCollisions<'a> {
    dimension: &'a Dimension,
    // context: CollisionContext,
    aabb: AABB,
}

impl<'a> BlockCollisions<'a> {
    pub fn new(dimension: &'a Dimension, entity: Option<&Entity>, aabb: AABB) -> Self {
        Self { dimension, aabb }
    }
}

impl Iterator for BlockCollisions<'a> {
    type Item = VoxelShape;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.cursor.advance() {
                return None;
            }
        }
    }
}

pub struct VoxelShape {}
