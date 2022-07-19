use std::str::ParseBoolError;
use clap::Command;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env;

extern crate clap;
use clap::command;

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

// Massive block of code BEGIN

/// Asks the user for how their day went and a date for the day. 
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

            Err(_) => {
                println!("Something went wrong");
            }
        }
    }
}

// Massive block of code END
/// Returns a list of all the previous days at location "./save.json" if none exists, returns an
/// error.
fn reader() -> Result<Vec<Day>, String>{ 
    if Path::new("save.json").exists(){
        let reader = BufReader::new(File::open("save.json").expect("failed to read save.json")); 
        Ok(serde_json::from_reader(reader).expect("file exists, could not read it"))
    } else {
        let _file = File::create("save.json").expect("failed to create file");
        Ok(Vec::<Day>::new())
    }
}

fn append_to_file(list: &Vec<Day>) {
    todo!()
}

// dani is a neek

fn write_to_file(list: &Vec<Day>) {
    // error here maybe?
    let mut file = File{};
    if Path::new("save.json").exists(){
        file = File::open("save.json").expect("couldn't open file");
    } else {
        file = File::create("save.json").expect("failed to create file");
    }

    
    let serialised = serde_json::to_string_pretty(list).unwrap();

    file.write_all(&serialised.into_bytes())
        .expect("failed to write to file");
}

fn moodtracker_input_sys(settings: &Settings) {
    // File reader and creat-er system thingy COPYRIGHT ©️ TheAverageAvocado 2022 https://github.com/LittlRayRay 
    let mut list: Vec<Day> = reader().unwrap();

    input_system(&mut list, settings);
    write_to_file(&list);
}

fn clear_file() {

    let blank = Vec::new();
    println!("successfully cleared file");
    if Path::new("save.json").exists(){
        write_to_file(&blank);
    }
}


fn main(){
    
    let matches = command!()
        .subcommand(
            Command::new("new")
                .about("makes a new entry in the moodtracker file!")
            )
        .subcommand(
            Command::new("clear")
                .about("clears the contents of the file")
            ).get_matches();
    
    let global_settings = init_settings(true, 1);
        
    if let Some(_matches) = matches.subcommand_matches("new") {
        moodtracker_input_sys(&global_settings);
    }

    if let Some(_) = matches.subcommand_matches("clear") {
        clear_file();
    }
}
