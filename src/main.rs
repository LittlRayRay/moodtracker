#![allow(unused_variables, dead_code)]
use std::io;
use std::str::ParseBoolError;
// use json::object;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env;

// Data structure for each day, storing how the day went and the date.
#[derive(Serialize, Deserialize, Debug)]
struct Day {
    date: String,
    feeling_good: bool,
}

// Settings for the user, changeable at any time (hopefully)
struct Settings {
    admin: bool,
    times_to_enter: u8,
}

impl Settings {
    // Default settings for the user to use
    fn new(admin_buffer: bool, times_to_enter_buffer: u8) -> Settings {
        Settings {
            admin: admin_buffer,
            times_to_enter: times_to_enter_buffer,
        }
    }
}

fn init_settings (admin_setting: bool, times: u8) -> Settings{
    Settings::new(admin_setting, times)
}

/// Asks the user for how their day went and a date. 
/// ```
/// fn input_sytem() {};
/// ```
fn input_system(list: &mut Vec<Day>, settings: &Settings) {
    let mut count = 0;

    'main: loop {
        if count == settings.times_to_enter {
            break;
        }

        count += 1;

        println!("feeling good?");
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let input_slice: &str = &input[..];
        let t: Result<bool, ParseBoolError> = input_slice.trim().parse();

        match t {
            Ok(input) => {
                println!("processing...\n - Saving Input \n \n Get ready to enter date of admission!");

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

                list.push(Day {
                    date: buffer,
                    feeling_good: input,
                });
            }

            Err(err) => {
                println!("Something went wrong");
            }
        }
    }
}

fn write_to_file(list: &Vec<Day>) {
    let mut file = File::create("save.json").expect("failed to create file");

    let serialised = serde_json::to_string_pretty(list).unwrap();

    

    file.write_all(&serialised.into_bytes())
        .expect("failed to write to file");
}

fn moodtracker_input_sys(settings: &Settings) {
    // File reader and creat-er system thingy COPYRIGHT ©️ TheAverageAvocado 2022 https://github.com/LittlRayRay 
    let mut list: Vec<Day> = Vec::new();

    if Path::new("save.json").exists(){
        let reader = BufReader::new(File::open("save.json").expect("failed to read save.json")); 
        list= serde_json::from_reader(reader).expect("file exists, could not read it");
    }

    input_system(&mut list, settings);

    write_to_file(&list);

}

fn clear_file() {

    let blank = Vec::new();

    if Path::new("save.json").exists(){
        write_to_file(&blank);
    }
}

fn main(){

    let global_settings = init_settings(true, 2);

    'cli: loop {

        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        
        std::io::stdin().read_line(&mut input).unwrap();
            
        // All commands in the cli api (?) 
        //
        // Very VERY dodgy way to handle commands
        //
        // TODO make this less dodgy
        // - Maybe use a file to read commands, along with what they do?
        
        

        input = String::from(input.trim()); 
        match input.as_str() {
            "help" => {println!("following commands can be used: 'new entry', 'exit', 'clear file'");}
            "new entry" => {moodtracker_input_sys(&global_settings)}, 
            "exit" => {break 'cli;},
            "clear file" => {
                clear_file();
            },
            "" => {println!("please enter a command.");},
            _ => {println!("\"{}\" \x1b[91mError - Command not found!\x1b[0m", input.as_str());}
        }
    }

     
}
