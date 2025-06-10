use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AtomicCounter {
    counter: std::sync::atomic::AtomicUsize,
}

impl AtomicCounter {
    pub fn get_id(self: &Self) -> usize {
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
