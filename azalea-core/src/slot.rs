// TODO: have an azalea-inventory crate and put this there

#[derive(Debug, Clone)]
pub enum Slot {
    Present(SlotData),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SlotData {
    pub id: i32,
    // TODO: is this really a u8? is it a i8? is it a varint?
    // wiki.vg says it's a "byte"
    pub count: u8,
    pub nbt: azalea_nbt::Tag,
}
