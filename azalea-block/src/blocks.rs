use crate::BlockBehavior;
use block_macros::make_block_states;

pub trait Block {
    fn behavior(&self) -> BlockBehavior;
    fn id(&self) -> &'static str;
}

make_block_states! {
    PROPERTIES => {
        Face {
            Floor,
            Wall,
            Ceiling
        };
        Facing {
            North,
            South,
            West,
            East
        };
        Powered {
            True,
            False
        };
        Half {
            Upper,
            Lower
        };
        Hinge {
            Left,
            Right
        };
        Open {
            True,
            False
        };
    }
    BLOCKS => {
        acacia_button => BlockBehavior::default().no_collision(), {
            Face,
            Facing,
            Powered
        };
        acacia_door => BlockBehavior::default(), {
            Facing,
            Half,
            Hinge,
            Open,
            Powered
        };
    }
}

// #[derive(Debug, Clone, Copy)]
// pub enum Face {
//     Floor,
//     Wall,
//     Ceiling,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Facing {
//     North,
//     South,
//     West,
//     East,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Powered {
//     True,
//     False,
// }

// // the underscore makes it more readable, so i think it's fine to allow it
// #[allow(non_camel_case_types)]
// pub enum BlockState {
//     AcaciaButton_FloorNorthTrue,
//     AcaciaButton_WallNorthTrue,
//     AcaciaButton_CeilingNorthTrue,
// }

// pub trait Block {
//     fn behavior(&self) -> BlockBehavior;
// }

// #[derive(Debug)]
// pub struct AcaciaButtonBlock {
//     pub face: properties::Face,
//     pub facing: properties::Facing,
//     pub powered: properties::Powered,
// }

// impl Block for AcaciaButtonBlock {
//     fn behavior(&self) -> BlockBehavior {
//         BlockBehavior {
//             has_collision: false,
//         }
//     }
// }

// pub struct AcaciaDoorBlock {
//     pub facing: properties::Facing,
//     // pub half: properties::Half,
//     // pub hinge: properties::Hinge,
//     // pub open: properties::Open,
//     pub powered: properties::Powered,
// }

// impl From<BlockState> for &dyn Block {
//     fn from(b: BlockState) -> Self {
//         match b {
//             BlockState::AcaciaButton_FloorNorthTrue => &AcaciaButtonBlock {
//                 face: properties::Face::Floor,
//                 facing: properties::Facing::North,
//                 powered: properties::Powered::True,
//             },
//             // BlockState::AcaciaButton_WallNorthTrue => todo!(),
//             // BlockState::AcaciaButton_CeilingNorthTrue => todo!(),
//             _ => todo!(),
//         }
//     }
// }
// impl From<AcaciaButtonBlock> for BlockState {
//     fn from(b: AcaciaButtonBlock) -> Self {
//         match b {
//             AcaciaButtonBlock {
//                 face: properties::Face::Floor,
//                 facing: properties::Facing::North,
//                 powered: properties::Powered::True,
//             } => BlockState::AcaciaButton_FloorNorthTrue,
//             // AcaciaButtonBlock {
//             //     face: properties::Face::Wall,
//             //     facing: properties::Facing::North,
//             //     powered: properties::Powered::True,
//             // } => todo!(),
//             // AcaciaButtonBlock {
//             //     face: properties::Face::Ceiling,
//             //     facing: properties::Facing::North,
//             //     powered: properties::Powered::True,
//             // } => todo!(),
//             _ => todo!(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn test_from_state_to_block() {
//         let state = BlockState::AcaciaButton_CeilingSouthFalse;
//         let block_state = BlockState::from(state);
//         let block: Box<dyn Block> = block_state.into();
//         assert_eq!(block.id(), "acacia_button");
//         // downcast block to AcaciaButtonBlock
//         // let acacia_button_block = block.try_into::<AcaciaButtonBlock>().unwrap();
//         // assert_eq!(acacia_button_block.face, Face::Ceiling);
//         // assert_eq!(acacia_button_block.facing, Facing::South);
//         // assert_eq!(acacia_button_block.powered, Powered::False);
//     }

//     fn test_from_state_to_block_bottom_edge() {
//         let state = BlockState::AcaciaButton_FloorNorthTrue;
//         let block_state = BlockState::from(state);
//         let block: Box<dyn Block> = block_state.into();
//         assert_eq!(block.id(), "acacia_button");
//     }
// }
