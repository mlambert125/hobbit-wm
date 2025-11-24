use smithay::{reexports::wayland_server::protocol::wl_buffer, wayland::buffer::BufferHandler};

use crate::HobbitWm;

impl BufferHandler for HobbitWm {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}
