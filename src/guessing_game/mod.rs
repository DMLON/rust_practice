use std::io;
use std::cmp::Ordering;
use rand::Rng;


pub fn guessing_game() {
    println!("Guessing game");

    let secret_number = rand::thread_rng().gen_range(1..101);

    

    println!("Guess the number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim().parse::<u32>().expect("Please type a number!");

    println!("You guessed: {}", guess);

    println!("The secret number is {}", secret_number);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}