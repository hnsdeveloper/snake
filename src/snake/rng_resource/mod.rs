use bevy::prelude::*;
use rand::prelude::*;
use std::ops::Range;
use std::sync::Mutex;

/// Consideration:
/// Wraping the StdRng introduces falsity regarding systems that use the rng being able to run in parallel.
/// At least completely in parallel as once two or more systems reach the point of using the rng, they will
/// have to wait on the other. I still think it is a good idea to wrap it in a Mutex and use it as Res instead
/// of ResMut given that:
/// A) The use of the rng is (usually) a short operation.
/// B) Systems are able to run in parallel up to the point where they require the rng, thus if complex/slow
/// operations are ran, one system doesn't have to wait completely on the other if both depend on the rng.
#[derive(Resource)]
pub struct RngResource(Mutex<StdRng>);

impl RngResource {
    pub fn random_in_range(self: &Self, range: Range<u64>) -> u64 {
        self.0.lock().unwrap().gen_range(range)
    }

    pub fn random(self: &Self) -> u64 {
        self.0.lock().unwrap().r#gen()
    }
}

impl FromWorld for RngResource {
    fn from_world(mut _world: &mut World) -> Self {
        RngResource(Mutex::new(rand::SeedableRng::from_entropy()))
    }
}
