use std::fs::File;
use std::io::{Read, Write, stdout};
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use data::class::Class;
use data::time::{Day, Time};
use super::io::{stdin_readline, print};
use super::parse_time;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    classes: Vec<Class>,
}

pub fn read_or_init_config(file: &mut File) -> Config {
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Encountered issue when accessing config file.");
    
    content = content.trim().to_string();

    // if no config
    if content.len() == 0 {
        // find local classes/weeks 
        print("No existing classes found in config. Would you like to add classes now? [Y/n]: ");
        let raw = stdin_readline().expect("Encountered error when reading from stdin.");
        let response = raw.trim();

        if response.len() == 0 || response.to_ascii_lowercase() == "y" {
            let config = toml::to_string(&init_classes()).unwrap();
            if let Err(e) = file.write_all(config.as_bytes()) {
                eprintln!("Failed to write config: {e:#}");
                println!("Here's your config file so you don't have to start over:\n{}", config)
            }
        }

        return Config::default();
    }

    return toml::from_str(&content).expect("Failed to parse TOML.");
}

fn get_config_path() -> PathBuf {
    return std::env::home_dir()
        .unwrap_or(std::env::current_dir().unwrap_or(".".into()))
        .join(".config/classnote")
    ;
}

pub fn get_config_file(path: Option<&PathBuf>) -> File {
    let file_path = match path {
        Some(p) => p,
        None => {
            let config_path = get_config_path();
            &config_path.join("config.toml")
        }
    };

    std::fs::create_dir_all(file_path.parent().unwrap()).expect("Unable to create config directory.");
    std::fs::File::options().append(true).read(true).create(true).open(file_path).expect("Unable to create config file. Please check home directory .config permissions")
}


pub fn init_classes() -> Config {
    let mut config = Config::default();

    loop {
        let mut class = Class::default();

        print("Class name: ");
        let raw = stdin_readline().expect("Unable to read class name.");
        let input = raw.trim();

        if input.len() == 0 {
            println!("Please enter a class name.");
            continue;
        }

        class.name = input.to_string();
        
        loop {
            let mut start_time = Time::default();
            print("What day is this class on (one at a time, M, T, W, Th, F, Sat, Sun, or Ctrl+c to exit): ");
            let raw = stdin_readline().expect("Unable to read date");
            let input = raw.trim();
            let day = Day::from(input);
            if day == Day::Async {
                config.classes.push(class.clone());
                println!("Added {} as an asynchronous class.", class.name);
                break;
            }

            if day == Day::Unset {
                println!("Invalid input: \"{input}\". Please use specified format.");
                continue;
            };

            start_time.day = day;

            loop {
                print!("What time on {} does this class start (13:00, 1:00 PM): ", start_time.day); stdout().flush().expect("Unable to flush stdout buffer.");
                let raw = stdin_readline().expect("Unable to read input time.");
                let input = raw.trim();

                if input.len() == 0 {
                    println!("Please enter a valid start time.");
                    continue;
                }

                (start_time.hour, start_time.minute, start_time.second) = match parse_time(input) {
                    Err(e) => {
                        println!("{e}");
                        continue;
                    },
                    Ok(t) => t
                };

                println!("Set class to begin at {:0>2}:{:0>2}:{:0>2} on {}", start_time.hour, start_time.minute, start_time.second, start_time.day);

                break;
            }
        
            let mut end_time = Time::default();
            loop {
                print("What time does this class end (13:00, 1:00 PM): ");
                let raw = stdin_readline().expect("Unable to read input time.");
                let input = raw.trim();

                // if end_time is earlier than start_time, increment day
                if input.len() == 0 {
                    println!("Pleae enter a valid end time.");
                    continue;
                }

                (end_time.hour, end_time.minute, end_time.second) = match parse_time(input) {
                    Err(e) => {
                        println!("{e}");
                        continue;
                    }

                    Ok(t) => t
                };

                end_time.day = if end_time.hour < start_time.hour || (end_time.hour == start_time.hour && end_time.minute < start_time.minute) {
                    print("Confirm class spans between two days [y/N]: ");
                    let input = stdin_readline().expect("Unable to read confirmation.");
                    if input.trim().len() == 0 || input.trim().to_ascii_lowercase() != "y" {
                        continue;
                    }
                    
                    start_time.day.tomorrow()
                } else {
                    start_time.day
                };

                println!("Set class to end at {:0>2}:{:0>2}:{:0>2} on {}", end_time.hour, end_time.minute, end_time.second, end_time.day);

                break;
            }

            class.times.push((start_time, end_time).into());

            // set defaults based on previous entry
            print("Would you like to add another time for this class? [Y/n]: ");
            match stdin_readline().expect("Failed to read input.").as_str().trim() {
                "" | "Y" | "y" => continue,
                _ => break
            }
        }
        
        config.classes.push(class);

        print("Would you like to add another class? [Y/n]: ");
        match stdin_readline().expect("Failed to read input.").as_str().trim() {
            "" | "Y" | "y" => continue,
            _ => break
        }
    }

    return config;
}