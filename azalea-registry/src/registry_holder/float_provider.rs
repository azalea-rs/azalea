use azalea_codec::Codec;
use azalea_registry_builtin::builtin::FloatProviderKind;

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum FloatProvider {
    Constant(f32),
    Dispatch(FloatProviderDispatch),
}
#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(tag = "type", tag_as = FloatProviderKind)]
#[codec(default_snake_case_key)]
pub enum FloatProviderDispatch {
    Uniform {
        min_inclusive: f32,
        max_exclusive: f32,
    },
    ClampedNormal {
        mean: f32,
        deviation: f32,
        min: f32,
        max: f32,
    },
    Trapezoid {
        min: f32,
        max: f32,
        plateau: f32,
    },
}

impl Default for FloatProvider {
    fn default() -> Self {
        Self::Constant(0.)
    }
}
azalea_codec::impl_defaultable!(FloatProvider);
