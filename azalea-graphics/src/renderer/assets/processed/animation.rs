#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Animation {
    width: Option<u32>,
    height: Option<u32>,
    frametime: Option<u32>,
    frames: Option<Vec<Frame>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(untagged)]
pub enum Frame {
    Simple(u32),
    WithTime { index: u32, time: Option<u32> },
}

impl Animation {
    pub fn size(&self, image_size: (u32, u32)) -> (u32, u32) {
        let min_dim = image_size.0.min(image_size.1);

        let width = self.width.or(self.height).unwrap_or(min_dim);
        let height = self.height.or(self.width).unwrap_or(min_dim);

        (width, height)
    }

    pub fn get_frame(&self, frame: usize, image_size: (u32, u32)) -> (u32, u32) {
        let (fw, fh) = self.size(image_size);
        let cols = image_size.0 / fw;

        let index = if let Some(frames) = &self.frames {
            match &frames[frame] {
                Frame::Simple(i) => *i,
                Frame::WithTime { index, .. } => *index,
            }
        } else {
            frame as u32
        };

        let col = index % cols;
        let row = index / cols;

        (col * fw, row * fh)
    }

    pub fn frame_time(&self, frame: usize) -> u32 {
        self.frames
            .as_ref()
            .and_then(|frames| frames.get(frame))
            .and_then(|f| match f {
                Frame::Simple(_) => None,
                Frame::WithTime { time, .. } => *time,
            })
            .or(self.frametime)
            .unwrap_or(1)
    }
}
