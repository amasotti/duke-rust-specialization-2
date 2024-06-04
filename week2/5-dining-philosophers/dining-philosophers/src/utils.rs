use std::sync::{Arc, Mutex};
use crate::philosopher::Fork;

pub fn init_forks() -> Vec<Arc<Fork>> {
    let forks_vec = 0..  4;

    forks_vec
        .map(|id| { init_fork_mutex(id) })
        .collect::<Vec<_>>()
}

/// Initialize a fork with a given id.
///
/// The fork is protected by a mutex to ensure that
/// only one philosopher can pick it up at a time.
///
/// # Arguments
///
/// * `id` - The id of the fork.
///
/// # Returns
///
/// * A fork with the given id.
fn init_fork_mutex(id: u32) -> Arc<Fork> {
    Arc::new(Fork {
        id,
        mutex: Mutex::new(()),
    })
}
