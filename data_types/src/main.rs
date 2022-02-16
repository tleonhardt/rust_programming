fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {}, y = {}", x, y);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
