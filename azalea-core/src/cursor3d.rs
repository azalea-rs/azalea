use crate::BlockPos;

pub struct Cursor3d {
    index: usize,

    origin_x: i32,
    origin_y: i32,
    origin_z: i32,

    width: usize,
    height: usize,
    depth: usize,

    end: usize,
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
                x: self.origin_x + x as i32,
                y: self.origin_y + y as i32,
                z: self.origin_z + z as i32,
            },
            iteration_type: iteration_type.into(),
        })
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, Debug)]
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
    pub fn new(
        origin_x: i32,
        origin_y: i32,
        origin_z: i32,
        end_x: i32,
        end_y: i32,
        end_z: i32,
    ) -> Self {
        let width = (end_x - origin_x + 1)
            .try_into()
            .expect("Impossible width.");
        let height = (end_y - origin_y + 1)
            .try_into()
            .expect("Impossible height.");
        let depth = (end_z - origin_z + 1)
            .try_into()
            .expect("Impossible depth.");

        Self {
            index: 0,

            origin_x,
            origin_y,
            origin_z,

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
