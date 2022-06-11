use crate::BlockBehavior;
use block_macros::make_block_states;

pub trait Block {
    fn behavior(&self) -> BlockBehavior;
    fn id(&self) -> &'static str;
}

make_block_states! {
    Properties => {
        "facing" => Facing {
            North,
            South,
            West,
            East,
        },
        "has_bottle" => HasBottle {
            True,
            False,
        },
    },
    Blocks => {
        brain_coral_wall_fan => BlockBehavior::default(), {
            Facing=North,
        },
        brewing_stand => BlockBehavior::default(), {
            HasBottle=False,
            HasBottle=False,
            HasBottle=False,
        },
    }
}
