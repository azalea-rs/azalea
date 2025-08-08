//! Some functions that are useful to have when implementing
//! `Serialize`/`Deserialize`, which Azalea uses to imitate Minecraft codecs.

use serde::{Serialize, Serializer};

/// Intended to be used for skipping serialization if the value is the default.
///
/// ```no_run
/// #[serde(skip_serializing_if = "is_default")]
/// ```
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

/// Intended to be used for skipping serialization if the value is `true`.
///
/// ```no_run
/// #[serde(skip_serializing_if = "is_true")]
/// ```
pub fn is_true(t: &bool) -> bool {
    *t
}

/// If the array has a single item, don't serialize as an array
///
/// ```no_run
/// #[serde(serialize_with = "flatten_array")]
/// ```
pub fn flatten_array<S: Serializer, T: Serialize>(x: &Vec<T>, s: S) -> Result<S::Ok, S::Error> {
    if x.len() == 1 {
        x[0].serialize(s)
    } else {
        x.serialize(s)
    }
}
