use std::io;

mod generator;

fn main() {
    let instance_random = generator::Random; // instance_random module
    let random_value = instance_random.get_value(); // get random value with rand lib
    let mut user_attempt = 0;
    let mut user_error = false; // user error control
    let parse_random_value = random_value.try_into().unwrap();

    loop {
        let mut input = String::new(); // get user input value

        if user_attempt == 0 {
            println! ("Pick a number between 1 and 100 and see if you get it right!");
        } else if user_error {
            println! ("You typed a character outside the rules. Please choose another one.");
        } else  {
            println!("You messed up. Please try again.");
        }

        user_error = false; // reset error
        user_attempt += 1;


        io::stdin().read_line(&mut input).expect("Error reading input.");
        
        // check int type
        match input.trim().parse::<i32>() {
            Ok(value) => {
                  
                if value > 100 || value < 1  {
                    user_error = true;
                    continue;
                }

                if value < parse_random_value {
                    println!("Você digitou um número menor.");
                    continue;
                } else if value > parse_random_value {
                    println!("Você digitou um número maior.");
                    continue;
                }

                if value != parse_random_value {
                    continue;
                }

                println!(
                    "Congratulations! You nailed it! The random number was: {}, you got it right on the {}° try",
                    random_value,
                    user_attempt
                    );
                break;
            }
            Err(_) => {
                user_error = true;
            }
        }
    }
}
