fn main() {
    let s = String::new();
    println!("Empty string has length {}", s.len());

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {} and s is {}", s2, s);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}, s1 = {}", s, s1);

    let hello = String::from("Hola");
    println!("{:?}.len() = {}", hello, hello.len());

    let hello = String::from("Здравствуйте");
    println!("{:?}.len() = {}", hello, hello.len());

    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
