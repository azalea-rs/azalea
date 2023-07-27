use bevy_ecs::component::Component;
use derive_more::{Deref, DerefMut};

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Food(pub u32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Saturation(pub f32);
