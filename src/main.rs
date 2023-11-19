use std::io;
mod generator;

enum UserComparison {
    Less,
    Greater,
    Equal,
    OutOfBounds
}

// get user input value
fn get_user_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input.");
    input.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input. Please enter a valid number");
        get_user_input()
    })
}
// compare values with enum type
fn compare_values(user_value: i32, target_value: i32) -> UserComparison {
    match user_value {
        1..=100 => {
            if user_value < target_value {
                UserComparison::Less
            } else if user_value > target_value {
                UserComparison::Greater
            } else {
                UserComparison::Equal
            }
        }
        _ => UserComparison::OutOfBounds
    }
}

fn main() {
    let instance_random = generator::Random; // instance_random module
    let random_value = instance_random.get_value(); // get random value with rand lib
    let parse_random_value = random_value.try_into().unwrap();

    let mut user_attempt = 0; // control attempt

    loop {
        if user_attempt == 0 {
            println!("Pick a number between 1 and 100 and see if you get it right");
        } else {
            match compare_values(get_user_input(), parse_random_value) {
                UserComparison::Less => println!("You typed in a lower number."),
                UserComparison::Greater => println!("You typed in a higher number."),
                UserComparison::Equal => {
                    println!(
                        "Congratulations! You nailed it! The random number was: {}, you got it right on the {}° try",
                        random_value,
                        user_attempt
                    );
                    break;
                }
                UserComparison::OutOfBounds => {
                    println!("You typed a number outside the rules. Please choose another one.");
                    continue;
                }
            }
        }
        
        user_attempt += 1;
    }
}
