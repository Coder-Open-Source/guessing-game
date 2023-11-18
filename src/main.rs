mod number;

fn main() {
    let mut generator = number::Generator {
        seed: 42
    };

    generator.random();
}
