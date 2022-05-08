// TODO: have an azalea-inventory or azalea-container crate and put this there

#[derive(Debug, Clone)]
pub enum Slot {
    Present(SlotData),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SlotData {
    pub id: i32,
    pub count: u8,
    pub nbt: azalea_nbt::Tag,
}
