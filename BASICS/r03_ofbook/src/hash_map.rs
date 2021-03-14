// https://doc.rust-lang.org/book/ch08-03-hash-maps.html



use std::collections::HashMap;


fn hash_map_basics() {
    
    // 1. Basic usage
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("John"), 10);
    scores.insert(String::from("Grace"), 50);
    scores.insert(String::from("Yuri"), 15);
    println!("  * Yuri's score: {}", scores["Yuri"]);

    // 2. Create hash map from two vectors where values correspond to each other
    let names = vec!("John".to_string(), "Grace".to_string(), "Yuri".to_string());
    let numbers = vec!(10, 50, 15);
    let mut score_map: HashMap<_, _> = names.into_iter().zip(numbers.into_iter()).collect();
    println!("  * Yuri's score: {}", score_map["Yuri"]);
    println!("  * All scores: {:?}", score_map);

    // 3. Safely get value from hash map
    let print_score = |_name: &str| {
        let _score = score_map.get(_name);
        if let Some(val) = _score {
            println!("  * {}'s score: {}", _name, val);
        } else { println!("  * {}'s score not found", _name); }
    };
    print_score("Andy");
    print_score("John");

    // 4. Insert new value or overwrite existing
    score_map.insert("Yuri".to_string(), 100);
    println!("  * Yuri's score: {}", score_map["Yuri"]);

    // 5. Insert only if not exists
    score_map.entry("Yuri".to_string()).or_insert(200);
    println!("  * Yuri's score: {}", score_map["Yuri"]);
    score_map.entry("Mona".to_string()).or_insert(200);
    println!("  * Mona's score: {}", score_map["Mona"]);

    // 6. Modify existing values
    for (_, val) in &mut score_map {
        *val += 10;  // We have direct reference to memory location
    }
    println!("  * Increased scores: {:?}", score_map);
}


pub fn demo() {

    println!("== hash_map demo begin ==");

    hash_map_basics();

    println!("== hash_map demo end ==");
    println!();
}
