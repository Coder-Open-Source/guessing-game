use rand::prelude::*;

pub struct Random;

impl Random {
    pub fn get_value(&self) -> u32 {
        let mut rng = rand::thread_rng();

        rng.gen_range(1..=100)
    }
}

