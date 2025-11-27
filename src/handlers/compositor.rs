use crate::{HobbitWm, state::ClientState};
use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_compositor,
    desktop::WindowSurfaceType,
    reexports::wayland_server::{Client, protocol::wl_surface::WlSurface},
    utils::{Logical, Point},
    wayland::compositor::{
        CompositorClientState, CompositorHandler, CompositorState, get_parent, is_sync_subsurface,
    },
};

use super::xdg_shell;

impl CompositorHandler for HobbitWm {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);

        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self
                .space
                .elements()
                .find(|w| w.toplevel().unwrap().wl_surface() == &root)
            {
                window.on_commit();
            }
        };

        xdg_shell::handle_commit(&mut self.popup_manager, &self.space, surface);
    }
}
delegate_compositor!(HobbitWm);

impl HobbitWm {
    pub fn surface_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(WlSurface, Point<f64, Logical>)> {
        self.space
            .element_under(pos)
            .and_then(|(window, location)| {
                window
                    .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                    .map(|(s, p)| (s, (p + location).to_f64()))
            })
    }
}
