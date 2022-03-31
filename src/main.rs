/*
tue
*/
#![allow(unused_variables)]
use std::str::ParseBoolError;
// use json::object; 
use serde::Serialize;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    date: String,
    feeling_good: bool
}

struct Settings {
    admin: bool,
    times_to_enter: u8 
}

impl Settings {
    fn new(admin_buffer: bool, times_to_enter_buffer: u8)  -> Settings{
        Settings {
            admin: admin_buffer,
            times_to_enter: times_to_enter_buffer
        }
    }
}

fn init_settings (admin_setting: bool, times: u8) -> Settings{
    Settings::new(admin_setting, times)
}

fn input_system(list: &mut Vec<Day>, settings: &Settings) {
        
    let mut count = 0;

    'main: loop{
        
        if count == settings.times_to_enter {
            break;
        }

        count += 1;

        println!("feeling good?");
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let input_slice: &str = &input[..];
        let t:Result<bool, ParseBoolError>= input_slice.trim().parse();

        match t {
            Ok(input) => {
                println!("processing...");

                let mut buffer = String::new(); 

                println!("date: ");
                
                std::io::stdin().read_line(&mut buffer).unwrap();

                buffer = buffer.trim().to_string(); 

                for day in list.iter() {
                    if day.date == buffer {
                        println!("already done!");
                        break 'main;
                    }
                }

                list.push(Day{date: buffer, feeling_good: input});

            },

            Err(err) => {println!("Something went wrong");}
        }
    }

}

fn write_to_file(list: &Vec<Day>){
    let mut file = File::create("save.json").expect("failed to create file");

    let serialised = serde_json::to_string(list).unwrap();

    file.write_all(&serialised.into_bytes()).expect("failed to write to file"); 

}

fn main() {
    // File reader and creat-er system thingy COPYRIGHT TheAverageAvocado 2022 https://github.com/LittlRayRay 
   
    let settings = init_settings(true, 2);

    let mut exists: bool= false;
    let mut list: Vec<Day> = Vec::new();

    if Path::new("save.json").exists(){
        exists = true;
    }
    
    if exists {
        let reader = BufReader::new(File::open("save.json").expect("failed to read save.json")); 
        list= serde_json::from_reader(reader).expect("file exists, could not read it");
    }

    input_system(&mut list, &settings);

    // Now we can insert our logic for the actual system here
    // TODO Transfer old system, mostly same code. 

    write_to_file(&list);

    /*

    let file = File::create("test.json").expect("failed to open file");
    
    //let file_read = File::open("save.json").expect("failed to open file");
    let reader = BufReader::new(file);

    let mut list: Vec<Day>= serde_json::from_reader(reader).expect("read file");
    
    //let mut list: Vec<Day> = Vec::new();
    let mut i = 0;
    while i < 2{
        'main: loop {
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
                            break 'main;
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

   // file.write_all(&serialised.into_bytes()).expect("failed to write to file");
    
    */

}
