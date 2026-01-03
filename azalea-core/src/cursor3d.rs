use crate::position::BlockPos;

pub struct Cursor3d {
    index: usize,

    origin: BlockPos,

    width: usize,
    height: usize,
    depth: usize,

    end: usize,
}

impl Cursor3d {
    pub fn origin(&self) -> BlockPos {
        self.origin
    }
}

impl Iterator for Cursor3d {
    type Item = CursorIteration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end {
            return None;
        }
        let x = self.index % self.width;
        let r = self.index / self.width;
        let y = r % self.height;
        let z = r / self.height;
        self.index += 1;

        let mut iteration_type = 0;
        if x == 0 || x == self.width - 1 {
            iteration_type += 1;
        }
        if y == 0 || y == self.height - 1 {
            iteration_type += 1;
        }
        if z == 0 || z == self.depth - 1 {
            iteration_type += 1;
        }

        Some(CursorIteration {
            pos: BlockPos {
                x: self.origin.x + x as i32,
                y: self.origin.y + y as i32,
                z: self.origin.z + z as i32,
            },
            iteration_type: iteration_type.into(),
        })
    }
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum CursorIterationType {
    Inside = 0,
    Face = 1,
    Edge = 2,
    Corner = 3,
}

pub struct CursorIteration {
    pub pos: BlockPos,
    pub iteration_type: CursorIterationType,
}

impl Cursor3d {
    pub fn new(origin: BlockPos, end: BlockPos) -> Self {
        let width = (end.x - origin.x + 1)
            .try_into()
            .unwrap_or_else(|_| panic!("Impossible width, origin: {origin:?}, end: {end:?}"));
        let height = (end.y - origin.y + 1)
            .try_into()
            .unwrap_or_else(|_| panic!("Impossible height, origin: {origin:?}, end: {end:?}"));
        let depth = (end.z - origin.z + 1)
            .try_into()
            .unwrap_or_else(|_| panic!("Impossible depth, origin: {origin:?}, end: {end:?}"));

        Self {
            index: 0,

            origin,

            width,
            height,
            depth,

            end: width * height * depth,
        }
    }
}

impl From<u8> for CursorIterationType {
    fn from(value: u8) -> Self {
        match value {
            0 => CursorIterationType::Inside,
            1 => CursorIterationType::Face,
            2 => CursorIterationType::Edge,
            3 => CursorIterationType::Corner,
            _ => panic!("Invalid iteration type"),
        }
    }
}
