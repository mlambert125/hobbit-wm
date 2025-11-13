use crate::compositor::HobbitCompositor;
use smithay::reexports::{
    calloop::EventLoop,
    wayland_server::{Display, DisplayHandle},
};
use tracing::info;

mod compositor;
mod winit;

struct CallLoopData {
    pub display_handle: DisplayHandle,
    pub compositor: HobbitCompositor,
}

fn main() -> anyhow::Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut event_loop = EventLoop::try_new()?;
    let display: Display<HobbitCompositor> = Display::new()?;
    let display_handle = display.handle();
    let compositor = HobbitCompositor::new(&mut event_loop, display);

    let mut call_loop_data = CallLoopData {
        display_handle,
        compositor,
    };

    unsafe {
        crate::winit::init_winit(&mut event_loop, &mut call_loop_data).unwrap();
    }
    info!("Starting the event loop");

    event_loop.run(None, &mut call_loop_data, |_| {})?;

    Ok(())
}
