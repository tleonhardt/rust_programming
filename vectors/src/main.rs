enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Create a new, empty vector to hold values of type i32
    let v1: Vec<i32> = Vec::new();
    println!("Vector v1 has length {}", v1.len());

    // Create a new Vec<i32> that holds the values 1, 2, and
    let v2 = vec![1, 2, 3];
    println!("Vector v2 has length {}", v2.len());

    // To create a vector and then add elements to it, we can use the push method
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("Vector v3 has length {}", v3.len());

    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // let does_not_exist1 = &v[100]; // This will panic at runtime
    let does_not_exist2 = v.get(100); // This will simply return None for the Option<T>

    // use a for loop to get immutable references to each element in a vector of i32
    for i in &v {
        println!("{}", i);
    }

    // We can also iterate over mutable references to each element in a mutable v
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{}", &v[0]);

    // Using an Enum to Store Multiple Types in a Vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
