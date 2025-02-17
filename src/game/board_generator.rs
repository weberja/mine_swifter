use bevy::prelude::*;
use bevy::{math::UVec2, utils::HashSet};
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;
use rand_core::RngCore;

pub struct BoardGenertor {
    pub bombs: HashSet<UVec2>,
}

impl BoardGenertor {
    pub fn random(
        ignore_field: UVec2,
        size: UVec2,
        bomb_count: u32,
        rng: &mut Entropy<WyRand>,
    ) -> Self {
        let mut bombs = HashSet::new();

        for _ in 0..bomb_count {
            Self::set_bomb(&mut bombs, ignore_field, size, rng);
        }

        Self { bombs }
    }

    /*pub fn solvable(
        ignore_field: UVec2,
        size: UVec2,
        bomb_count: u32,
        rng: Entropy<WyRand>,
    ) -> Self {
        let bombs = HashSet::new();
        Self { bombs }
    }*/

    fn set_bomb(
        bombs: &mut HashSet<UVec2>,
        ignore_field: UVec2,
        size: UVec2,
        rng: &mut Entropy<WyRand>,
    ) {
        let x_rand = rng.next_u32() & (size.y * size.x);
        let y_rand = rng.next_u32() & (size.y * size.x);

        let x = x_rand % size.x;
        let y = y_rand % size.y;

        let pos = UVec2 { x, y };

        if !bombs.contains(&pos) {
            info!("Bomb at {}", pos);
            bombs.insert(pos);
        } else {
            Self::set_bomb(bombs, ignore_field, size, rng);
        }
    }
}
