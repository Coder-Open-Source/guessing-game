mod generate_value;

fn main() {
    let result:u8 = generate_value::get_value();

    println!("{0}", result);
}