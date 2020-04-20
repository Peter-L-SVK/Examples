use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::process;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    const ROUNDS: usize = 7;
    let mut number_of_guess = ROUNDS;
    let mut logic: bool = true;
    let mut v: Vec<u32> = Vec::with_capacity(ROUNDS);
    
    loop {
        println!("You have {} tryes.", number_of_guess);
        println!("Please input your guess. (To quit enter: quit)");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            println!("Goodbye.");
            break;
        }
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input\n");
                continue;
            },
        };
        v.push(guess);
        if number_of_guess < ROUNDS {
            for index in 0..ROUNDS - number_of_guess {
                if guess == v[index] {
                    println!("You already tried this number!\n");
                    logic = false;
                    break;
                }  
                else {
                    logic = true;
                }
            }
        }
        
        if logic {
            println!("You guessed: {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!\n");
                    number_of_guess = control(number_of_guess);
                },
                Ordering::Greater => {
                    println!("Too big!\n");
                    number_of_guess = control(number_of_guess);
                },
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }        
        else {
            number_of_guess = control(number_of_guess);
        }
    }
}

fn control(mut num_of_guess: usize) -> usize {
    num_of_guess -= 1;
    if num_of_guess <= 0 {
        println!("Out of tryes.\nMaybe next time.");
        process::exit(0x0100);
    }
    return num_of_guess;
}
