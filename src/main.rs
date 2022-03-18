/*
true
*/
use chrono::prelude::*;
use std::str::FromStr;
use std::str::ParseBoolError;

// use serde_json; 


fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let new_utc = utc.to_string();
    let first_line = &new_utc[..10];
    println!("{}", first_line);
    let mut list: Vec<(String, bool)> = Vec::new();

    if list.contains(&(first_line.to_string(), true)) || list.contains(&(first_line.to_string(), false)) {
        println!("You have already done today's moodtracker!");
    } else {
        loop {

            println!("Are you feeling good today?");
        	let mut input = String::new();
        	
        	std::io::stdin().read_line(&mut input).unwrap();
        	
            let input_slice: &str = &input[..];
            let t:Result<bool, ParseBoolError>= input_slice.trim().parse();

            match t {
                Ok(_input) => {
                    println!("Thanks for the input!");
                    break
                },

                Err(_error) => {println!("Something went wrong!"); println!("Please enter 'true' or 'false'")}
            }
        }
    }

    // This is a test! How are you doing there? 
} 
