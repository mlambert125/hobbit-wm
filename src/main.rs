#![allow(irrefutable_let_patterns)]

mod handlers;

mod grabs;
mod input;
mod state;
mod winit;

use smithay::reexports::{
    calloop::EventLoop,
    wayland_server::{Display, DisplayHandle},
};
pub use state::HobbitWm;

pub struct CalloopData {
    compositor: HobbitWm,
    display_handle: DisplayHandle,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

    let display: Display<HobbitWm> = Display::new()?;
    let display_handle = display.handle();
    let state = HobbitWm::new(&mut event_loop, display);

    let mut data = CalloopData {
        compositor: state,
        display_handle,
    };

    crate::winit::init_winit(&mut event_loop, &mut data)?;

    std::process::Command::new("alacritty").spawn().ok();

    event_loop.run(None, &mut data, move |_| {})?;
    Ok(())
}
