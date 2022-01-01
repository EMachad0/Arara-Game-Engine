use glium::Frame;

pub struct TrackedFrame {
    pub frame: Frame,
}

impl TrackedFrame {
    pub fn new(frame: Frame) -> Self {
        Self { frame }
    }
}

