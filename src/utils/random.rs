use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// The source of randomness used by this example.
#[derive(Resource, Deref, DerefMut)]
pub struct RandomSource<T>(pub T)
where
    T: SeedableRng;

impl<T: SeedableRng> Default for RandomSource<T> {
    fn default() -> Self {
        ChaCha8Rng::from_os_rng();
        Self(T::from_os_rng())
    }
    // add code here
}
