use glium::glutin::event_loop::EventLoop as WinitEventLoop;

pub struct EventLoop {
    event_loop: WinitEventLoop<()>,
}

impl Default for EventLoop {
    fn default() -> Self {
        Self { event_loop: WinitEventLoop::new() }
    }
}

impl EventLoop {
    pub fn borrow(&self) -> &WinitEventLoop<()> {
        &self.event_loop
    }

    pub fn take(&mut self) -> WinitEventLoop<()> {
        std::mem::replace(&mut self.event_loop, WinitEventLoop::new())
    }
}