pub struct Generator {
    pub seed: u64
}

impl Generator {
    pub fn random(&mut self) {
        let lower_limit: u64 = 1;
        let upper_limit: u64 = 100;
        self.seed = self.seed.wrapping_add(1);

        println!("{}", (self.seed % upper_limit) + lower_limit);
    }
}

