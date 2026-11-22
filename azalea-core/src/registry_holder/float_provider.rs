use azalea_codec::{Codec, types::StringLiteral};

#[derive(Clone, Debug, Codec)]
pub enum FloatProvider {
    Constant(f32),
    #[codec(virt(
        #[codec(rename = "type")]
        kind: StringLiteral<"uniform">
    ))]
    Uniform {
        min_inclusive: f32,
        max_exclusive: f32,
    },
    #[codec(virt(
        #[codec(rename = "type")]
        kind: StringLiteral<"clamped_normal">
    ))]
    ClampedNormal {
        mean: f32,
        deviation: f32,
        min: f32,
        max: f32,
    },
    #[codec(virt(
        #[codec(rename = "type")]
        kind: StringLiteral<"trapezoid">
    ))]
    Trapezoid {
        min: f32,
        max: f32,
        plateau: f32,
    },
}
