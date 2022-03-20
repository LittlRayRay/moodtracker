/*
tue
*/
#![allow(dead_code, unused_variables, unused_imports)]
use chrono::prelude::*;
use std::str::ParseBoolError;
// use json::object; 
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    date: String,
    feeling_good: bool
}

fn test(arg1: String) -> i64 {
    0
}

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let new_utc = utc.to_string();
    let first_line = &new_utc[..10];
    println!("{}", first_line);
    let mut list: Vec<Day> = Vec::new();
    let mut i = 0;
    while i < 2{
        loop {
            println!("Are you feeling good today?");
            let mut input = String::new();
              
            std::io::stdin().read_line(&mut input).unwrap();
              
            let input_slice: &str = &input[..];
            let t:Result<bool, ParseBoolError>= input_slice.trim().parse();

            match t {
                Ok(input) => {
                    println!("Thanks for the input!");

                    let mut buffer = String::new();

                    println!("Please enter today's date (YYYY-MM-DD)!");

                    std::io::stdin().read_line(&mut buffer).unwrap();

                    buffer = buffer.trim().to_string();
                    
                    for day in &list{
                        if day.date == buffer {
                            println!("You have already done today's moodtracker!");
                            break;
                        }

                    }

                    list.push(Day{date:buffer, feeling_good: input });

                    break;
                    
                },

                Err(_error) => {println!("Something went wrong!"); println!("Please enter 'true' or 'false'!")}
            }
        }
        i += 1;
     }

    println!("{:?}", list);

    let serialised= serde_json::to_string(&list).unwrap();

    println!("serialised = {}", serialised);

}
