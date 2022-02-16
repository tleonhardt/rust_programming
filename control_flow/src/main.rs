fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }

    let even = number % 2 == 0;
    let parity = if even { "even" } else { "odd" };

    println!("{} is {}", number, parity);
}
