use azalea_codec::Codec;

/// TODO: unimplemented
#[derive(Clone, Debug, PartialEq, Default, Codec)]
#[codec(allow_unknown_keys)]
pub struct BlockPredicate {}
azalea_codec::impl_defaultable!(BlockPredicate);
