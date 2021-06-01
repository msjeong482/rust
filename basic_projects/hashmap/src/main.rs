use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    //let teams = vec![String::from("blue"), String::from("yellow")];
    //let initial_scores = vec![10, 50];
    //let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("blue"), 25);
    scores.entry(String::from("yellow")).or_insert(13);
    scores.entry(String::from("blue")).or_insert(13);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
