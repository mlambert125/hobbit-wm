use smithay::{
    delegate_shm,
    wayland::shm::{ShmHandler, ShmState},
};

use crate::HobbitWm;

impl ShmHandler for HobbitWm {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}
delegate_shm!(HobbitWm);
