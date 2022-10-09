#![feature(let_chains)]

mod dstarlite;
mod mtdstarlite;

use async_trait::async_trait;
use azalea::{Client, Event};
pub use dstarlite::DStarLite;
pub use mtdstarlite::MTDStarLite;
use std::sync::{Arc, Mutex};

// #[derive(Default)]
// pub struct Plugin {
//     pub state: Arc<Mutex<State>>,
// }

// #[derive(Default)]
// pub struct State {
//     // pathfinder: Option<DStarLite< Node, FloatOrd<f32>>>,
// }

// #[async_trait]
// impl azalea::Plugin for Plugin {
//     async fn handle(self: Arc<Self>, bot: Client, event: Arc<Event>) {
//         // match *
//     }
// }

// pub trait Trait {
//     fn goto(&self, goal: impl Goal);
// }

// impl Trait for azalea_client::Client {
//     fn goto(&self, goal: impl Goal) {
//         let start = BlockPos::from(self.position());

//         let pf = DStarLite::new();
//     }
// }

// // fn heuristt

// pub trait Goal {
//     fn heuristic(&self, x: i32, y: i32) -> f32;
// }
