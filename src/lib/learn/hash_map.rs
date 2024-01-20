use std::collections::HashMap;

fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Red"), 5);

    println!("score board: {:#?}", scores);

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) {
        Some(&score) => score,
        None => 0,
    };
    println!("orange team score: {}", score);

    let score = scores.get("Orange").copied().unwrap_or(0);
    println!("blue team score: {}", score);

    // traverse in a random order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // illegal, already borrowed
    // println!("{field_name} {field_value}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(100);
    let inserted_value = scores.entry(String::from("Yellow")).or_insert(10);
    // illegal, already borrowed
    // println!("{:?}", scores);
    println!("yellow: {inserted_value}",);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // *map.entry(word).or_insert(0) += 1;
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("count word appearances in \"{text}\": {:#?}", { map });
}
