pub struct Test {
    pub value: f32
}

impl Test {
    pub fn print_log(&self, msg: &str) {
        println!("{} {}", msg, self.value);
    }
}
