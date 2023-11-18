mod generate_value;
use std::io;
use std::io::Write;

fn main(){
    let result:u8 = generate_value::get_value();
    let mut trying:u8 = 1;

    println!("Digite um numero de 1 a 100: ");
    loop {

        let mut input:String = String::new();
        print!("Tentativa #{}: ",trying);
        io::stdout().flush().expect("Erro ao limpar buffer");
        let _ = io::stdin().read_line(&mut input);

        let number_user:u8 = input.trim().parse().expect("Input not an integer");

        match number_user{
            n if n < result => {
                println!("Você digitou um numero menor");
                trying += 1
            },
            n if n > result => {
                println!("Você digitou um numero numero maior");
                trying += 1
            },
            n if n == result => {
                println!("Parabéns, você acertou na tentativa #{}",trying);
                break;
            },
            _ => todo!()
        }

        println!("-------------");
        
    }
}