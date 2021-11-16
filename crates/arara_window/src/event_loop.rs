use glium::glutin::event_loop::EventLoop as WinitEventLoop;

pub struct EventLoop {
    event_loop: Option<WinitEventLoop<()>>,
}

impl Default for EventLoop {
    fn default() -> Self {
        Self {
            event_loop: Some(WinitEventLoop::new()),
        }
    }
}

impl EventLoop {
    pub fn borrow(&self) -> &Option<WinitEventLoop<()>> {
        &self.event_loop
    }

    pub fn take(&mut self) -> Option<WinitEventLoop<()>> {
        std::mem::take(&mut self.event_loop)
    }
}
