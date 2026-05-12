fn main() {
    // println!("Hello, world!");
    // println!("This is a simple Rust program, by Aderemi Abiodun!");

    //**======= GUESS THE NUMBER GAME ======= */
    use std::io;

    println!("Welcome to the Guess the Number Game!");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    let mut guessed_number = String::new();
    io::stdin().read_line(&mut guessed_number).expect("Something Went wrong!");
    println!("You guessed: {}", guessed_number.trim());
}
