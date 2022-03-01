use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}", map);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{} team has score {:?}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwrite value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only inserting a value if key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
