#![allow(irrefutable_let_patterns)]

mod handlers;

mod input;
mod state;
mod winit;

use std::sync::Arc;

use smithay::{
    reexports::{
        calloop::{EventLoop, Interest, Mode, PostAction, generic::Generic},
        wayland_server::{Display, DisplayHandle},
    },
    wayland::socket::ListeningSocketSource,
};
pub use state::HobbitWm;

use crate::state::ClientState;

pub struct CalloopData {
    compositor: HobbitWm,
    display_handle: DisplayHandle,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

    let display: Display<HobbitWm> = Display::new()?;
    let display_handle = display.handle();

    let dh = display.handle();
    let listening_socket = ListeningSocketSource::new_auto().unwrap();
    let socket_name = listening_socket.socket_name().to_os_string();

    let loop_handle = event_loop.handle();

    loop_handle
        .insert_source(listening_socket, move |client_stream, _, state| {
            state
                .display_handle
                .insert_client(client_stream, Arc::new(ClientState::default()))
                .unwrap();
        })
        .expect("Failed to init the wayland event source.");

    loop_handle
        .insert_source(
            Generic::new(display, Interest::READ, Mode::Level),
            |_, display, state| {
                unsafe {
                    display
                        .get_mut()
                        .dispatch_clients(&mut state.compositor)
                        .unwrap();
                }
                Ok(PostAction::Continue)
            },
        )
        .unwrap();

    let state = HobbitWm::new(&mut event_loop, dh, socket_name);

    let mut data = CalloopData {
        compositor: state,
        display_handle,
    };

    crate::winit::init_winit(&mut event_loop, &mut data)?;

    std::process::Command::new("alacritty").spawn().ok();

    event_loop.run(None, &mut data, move |_| {})?;
    Ok(())
}
