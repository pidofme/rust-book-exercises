use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // compiler error: value borrowed here after move
    // println!("field_name: {}", field_name);

    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut m = HashMap::new();

    for word in text.split_whitespace() {
        let count = m.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", m);
}