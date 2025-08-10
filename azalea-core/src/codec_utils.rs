//! Some functions that are useful to have when implementing
//! `Serialize`/`Deserialize`, which Azalea uses to imitate Minecraft codecs.

use azalea_buf::SerializableUuid;
use serde::{Serialize, Serializer, ser::SerializeTupleStruct};
use uuid::Uuid;

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

/// Minecraft writes UUIDs as an IntArray<4>
pub fn uuid<'a, S: Serializer>(
    uuid: impl Into<&'a Option<Uuid>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(uuid) = uuid.into() {
        let arr: [u32; 4] = uuid.to_int_array();
        let arr: [i32; 4] = [arr[0] as i32, arr[1] as i32, arr[2] as i32, arr[3] as i32];
        IntArray(arr).serialize(serializer)
    } else {
        serializer.serialize_unit()
    }
}

/// An internal type that makes the i32 array be serialized differently.
///
/// Azalea currently only uses this when writing checksums, but Minecraft also
/// uses this internally when converting types to NBT.
pub struct IntArray<const N: usize>(pub [i32; N]);
/// An internal type that makes the i64 array be serialized differently.
///
/// Azalea currently only uses this when writing checksums, but Minecraft also
/// uses this internally when converting types to NBT.
pub struct LongArray<const N: usize>(pub [i64; N]);

impl<const N: usize> Serialize for IntArray<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // see checksum::serialize_tuple_struct
        let mut seq = serializer.serialize_tuple_struct("azalea:int_array", N)?;
        for &item in &self.0 {
            seq.serialize_field(&item)?;
        }
        seq.end()
    }
}
impl<const N: usize> Serialize for LongArray<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // see checksum::serialize_tuple_struct
        let mut seq = serializer.serialize_tuple_struct("azalea:long_array", N)?;
        for &item in &self.0 {
            seq.serialize_field(&item)?;
        }
        seq.end()
    }
}
