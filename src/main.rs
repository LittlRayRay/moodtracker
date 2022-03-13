use chrono::prelude::*;
use serde_json; 


fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let new_utc = utc.to_string();
    let first_line = &new_utc[..10];
    println!("{}", first_line);
    let mut list: Vec<(String, bool)> = Vec::new();
    if list.contains(&(first_line.to_string(), true)) || list.contains(&(first_line.to_string(), false)) {
        println!("You have already done today's moodtracker!");
    } else {
        println!("Are you feeling good today?");
	let mut input = String::new();
	
	std::io::stdin().read_line(&mut input).unwrap(); 	
	println!("You typed: {}", input);
    }
} 
