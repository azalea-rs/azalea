use azalea_codec::{Codec, types::Drain};

#[derive(Clone, Debug, Default, PartialEq, Codec)]
pub struct BlockPosArgument(pub String);

#[derive(Clone, Debug, Default, PartialEq, Codec)]
pub struct EntitySelector(pub Drain);
