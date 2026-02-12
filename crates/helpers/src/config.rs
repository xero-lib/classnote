use std::fs::{File, ReadDir, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;

use data::class::Class;
use data::obsidian::ObsidianPath;
use data::time::{ClassTime, Day, Time, Times};
use data::{Config, Location, Editor};

use super::io::{demand_stdin, prompt};
use super::print_flush;

use super::parse_time;

// change all to post-pub reexport?

fn get_class(name: String) -> Class {
    let professor = demand_stdin(&format!("{} professor", name));        
    let mut times: Vec<ClassTime> = Default::default();
    let mut office_hours: Vec<ClassTime>  = Default::default();

    loop {
        let input = prompt!("Which days does {name} meet? (M/T/W/Th/F/Sat/Sun, or empty for Async): ");
        if input.is_empty() {
            println!("Marking {name} as asynchronous.");
            break;
        }

        let days: Vec<Day> = match input.split('/').map(str::parse).collect() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        println!("Now setting meeting times for {name} on {}...", days.iter().map(Day::to_string).collect::<Vec<_>>().join("/"));

        let mut prev_time = ClassTime::default();
        for day in days {
            if day == Day::Async { continue; }
            if prev_time != ClassTime::default() {
                let input = prompt!("Would you like to set the {day} meeting to the last time ({} - {})? [Y/n]: ", prev_time.start.hms_string(), prev_time.end.hms_string());

                if input.is_empty() || input.to_ascii_lowercase().starts_with('y') {
                    let mut new_time = prev_time.clone();
                    [new_time.start.day, new_time.end.day] = [day, if prev_time.start.day != prev_time.end.day { day.tomorrow() } else { day } ];
                    times.push(new_time);
                    continue;
                }
            }

            let new_start = get_time("start", day, None);
            let new_end = get_time("end", day, Some(new_start.clone()));

            prev_time = (new_start, new_end, get_location(&name, day)).into();
            times.push(prev_time.clone());
        }

        break;
    }

    // print saved times

    let input = prompt!("Would you like to enter office hours of {professor} for {name}? [Y/n]: ");

    if input.is_empty() || input.to_ascii_lowercase().starts_with('y') {
        loop {
            let input = prompt!("Which days are the office hours of {professor}? (M/T/W/Th/F/Sat/Sun): ");
            let days: Vec<Day> = match input.split('/').map(str::parse).collect::<Result<Vec<_>, _>>() {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            print_flush!("Now setting office hours for {professor} on {}...", days.iter().map(Day::to_string).collect::<Vec<_>>().join("/"));

            let prev_time = ClassTime::default();
            for day in days {
                if day == Day::Async { continue; }
                if prev_time != ClassTime::default() {
                    let input = prompt!("Would you like to set {professor}'s {day} office hours to the last submitted time ({}:{}:{} - {}:{}:{})? [Y/n]: ",
                        prev_time.start.hour, prev_time.start.minute, prev_time.start.second,
                        prev_time.end.hour,   prev_time.end.minute,   prev_time.end.second
                    );

                    if input.is_empty() || input.to_ascii_lowercase().starts_with('y') {
                        let mut new_time = prev_time.clone();
                        [new_time.start.day, new_time.end.day] = [day, if prev_time.start.day != prev_time.end.day { day.tomorrow() } else { day } ];
                        office_hours.push(new_time);
                        continue;
                    }
                }

                let new_start = get_time("start", day, None);
                let new_end = get_time("end", day, Some(new_start.clone()));

                office_hours.push((new_start, new_end, get_location(&format!("{professor}'s office hours"), day)).into());
            }

            break;
        }
    }

    return Class::new(name, professor, Times::from(times), Times::from(office_hours));
}

fn build_config_from_dir(dir: ReadDir) -> Config {
    let class_names = dir
        .filter_map(Result::ok)
        .filter_map(|f| 
            f
                .file_type()
                .ok()
                .and_then(|t| 
                    t
                        .is_dir()
                        .then_some(f)
                )
        )
        .map(|f|
            f
                .file_name()
                .to_string_lossy()
                .to_string()
        )
        .collect::<Vec<_>>()
    ;

    let mut config = Config::default();

    for name in class_names {
        config.add_class(get_class(name));
    }

    let mut root = std::env::current_dir().unwrap_or_default();
    
    let should_prompt_path = if !root.as_os_str().is_empty() {
        let input = prompt!("Would you like to set a new path to create new notes in? (N = set current directory as path) [y/N]: ");

        input.to_ascii_lowercase().starts_with('y')
    } else { true };

    if should_prompt_path {
        loop {
            let input = prompt!("Enter new notes path: ");

            if !std::fs::exists(&input).unwrap_or_default() {
                println!("Unable to find \"{input}\". Please try again.");
                continue;
            }

            println!("Setting path to \"{input}\".");

            root = PathBuf::from(input);

            break;
        }
    }

    let mut program = std::env::var("EDITOR").unwrap_or_default();
    let should_prompt = if !program.is_empty() {
        let input = prompt!("Would you like to set a custom editor? [y/N]: ");
        
        input.to_ascii_lowercase().starts_with('y')
    } else { true };
    
    if should_prompt { program = prompt!(required, "Editor program: "); }

    let editor = match program.as_str() {
        "obsidian" => {
            let vault = prompt!(required, "Which vault do you want to use?");
            // todo: Implement Editor::new or from
            Editor::Complex {
                program,
                uri: ObsidianPath {
                    courses_root: root.clone(),
                    vault
                }
            }
            // ! ask obsidian if it exists
        }
        _ => Editor::Simple(program)
    };

    config.set_editor(editor);
    config.set_root(root);

    return config;

    // traverse each folder individually and correlate days into class days
}

fn write_config<'file>(config: &Config, file: &mut File) -> Result<(), &'file str> {
    if let Err(e) = file.write_all(config.serialize().as_bytes()) {
        eprintln!("Failed to write config: {e:#}");
        println!("Here's your config file so you don't have to start over:\n{}", config.serialize());
        return Err("Failed to write config to file.");
    }

    println!("Successfully wrote config to file.");

    Ok(())
}

pub fn read_or_init_config(file: &mut File) -> Config {
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Encountered issue when accessing config file.");
    
    content = content.trim().to_string();

    // if no config
    if content.len() == 0 {
        let response = prompt!("No config found. Would you like to build a config from an existing directory? [y/N]: ");

        if response.to_ascii_lowercase().starts_with('y') {
            loop {
                let path = prompt!("Please enter path, or leave blank to read current directory, or c to cancel: ");
                if path == "c" { break; }
                let dir = if path.len() == 0 { std::env::current_dir().expect("Unable to get current directory.").read_dir().unwrap() } else { 
                    if !std::fs::exists(std::path::absolute(&path).expect("Unable to normalize path.")).expect("Unable to check for dir existence.") {
                        eprintln!("\"{path}\" does not appear to exist. Please try again...");
                        continue;
                    }

                    if !std::fs::metadata(&path).expect("Unable to read path metadata.").is_dir() {
                        eprintln!("\"{path}\" does not appear to be a directory. Please try again...");
                        continue;
                    }

                    std::fs::read_dir(path).expect("Unable to read path, even after confirming its existence.")
                };
                
                let config = build_config_from_dir(dir);
            
                write_config(&config, file).unwrap();
                
                return config;
            }
        }

        // find local classes/weeks 
        let response = prompt!("Would you like to create a new config and add classes now? [Y/n]: ");

        if response.len() == 0 || response.to_ascii_lowercase() == "y" {
            let config = init_classes();

            write_config(&config, file).unwrap();
                
            return config;
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

fn get_time(which: &str, day: Day, previous: Option<Time>) -> Time {
    let mut time = Time::default();
    time.day = day;

    loop {
       let input = prompt!("What time on {day} does this class {which} (13:00, 1:00 PM): ");

        if input.len() == 0 {
            println!("Please enter a valid {which} time.");
            continue;
        }

        (time.hour, time.minute, time.second) = match parse_time(&input) {
            Err(e) => {
                println!("{e}");
                continue;
            },
            Ok(t) => t
        };

        if let Some(start_time) = previous {
            time.day = if time.hour < start_time.hour || (time.hour == start_time.hour && time.minute < start_time.minute) {
                let input = prompt!("Confirm class spans between two days [y/N]: ");
                if input.len() == 0 || input.trim().to_ascii_lowercase().starts_with('n') {
                    continue;
                }
                
                start_time.day.tomorrow()
            } else {
                start_time.day
            };
        }

        println!("Set class to {which} at {time}");
        return time;
    }
}

fn get_location(of: &str, day: Day) -> Location {
    println!("Please fill out location information for {of} on {day}...");
    // what campus
    Location { 
        campus: demand_stdin("Campus"),
        building: demand_stdin("Building name/number"),
        room: demand_stdin("Room name/number")
    }
}

pub fn init_classes() -> Config {
    let mut config = Config::default();
    
    loop {
        let input = prompt!(required, "Where would you like to store notes? (Will be created if it doesn't exist): ");
        let result = match std::fs::exists(&input) {
            Ok(exists) if exists => Ok(config.set_root(PathBuf::from(&input))),
            Ok(_) => create_dir_all(&input),
            Err(e) => {
                eprintln!("Unable to check for file existence. Please try again, or press Ctrl+c to exit the program. ({e:#})");
                continue;
            }
        };

        match result {
            Ok(_) => break,
            Err(e) => eprintln!("Failed to create path \"{input}\", please try again: {e:#}")
        }
    }
    

    loop {
        // create enum for items/
        let name = demand_stdin("Class name");
        config.add_class(get_class(name));

        let response = prompt!("Would you like to add another class? [Y/n]: ");
        match response.as_str() {
            "" | "Y" | "y" => continue,
            _ => break
        }
    }

    return config;
}