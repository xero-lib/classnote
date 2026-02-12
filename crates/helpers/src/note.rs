use std::{fs::{DirEntry, File}, path::PathBuf, process::Command, time};

use chrono::{DateTime, Local};
use data::{Config, Editor, class::Class, obsidian::ObsidianPath, time::{Time, Times}};


pub fn create_note(name: &str) -> File{
    File::options().append(true).create(true).open(name).expect("Unable to open note at requested path.")
}

// using u16 in case the class is realllly long (1200 years)
pub fn get_latest_week_num(config: &Config, class: &Class) -> u16 {
    let root = config.get_root();
    if !std::fs::exists(&root).expect("Unable to check root's existence.") { return 0; }

    let class_path = root.join(class.get_name());
    if !std::fs::exists(&class_path).expect("Unable to check class's path existence.") { return 1; }
    
    std::fs::read_dir(class_path).expect("The class path is either inaccessible or not a directory.")
        .filter_map(Result::ok)
        // get directories
        .filter_map(|e| 
            e.file_type().ok().and_then(|f| 
                f.is_dir().then_some(e)
            )
        )
        // if it's a Week folder, get the week number
        .filter_map(|f| {
            f
                .file_name()
                .to_string_lossy()
                .strip_prefix("Week_")?
                .parse::<u16>()
                .ok()
        })
        .max()
        .unwrap_or(1)
}

pub fn get_current_classnote(config: &Config, class: &Class) -> (PathBuf, File) {
    let latest_week = get_latest_week_num(config, class);
    let course_path = config.get_root().join(class.get_name());
    let week_entries = std::fs::read_dir(course_path.join(format!("Week_{latest_week}"))).unwrap().filter_map(Result::ok).collect::<Vec<DirEntry>>();

    let last_class_num = week_entries
        .iter()
        .filter_map(|e|
            e.file_type().ok().and_then(|f|
                f.is_dir().then_some(e)
            )
        )
        .filter_map(|f| {
            f.file_name().to_string_lossy()
                .trim_end_matches('/')
                .strip_prefix("Class")?
                .split('-')
                .next()?
                .parse::<u16>()
                .ok()
        })
        .max()
        .unwrap_or(0)
    ;
    
    let current_time: DateTime<Local> = time::SystemTime::now().into();
    let current_iso_8601 = current_time.format("%Y-%m-%d");
    
    let potential_class_name = format!("Class{}-{}", last_class_num, current_iso_8601);
    let potential_class_path = course_path.join(format!("Week_{}", latest_week)).join(&potential_class_name);
    if std::fs::exists(&potential_class_path).expect("Unable to check for note folder path existence.") {
        let file_path = potential_class_path.join(potential_class_name + ".md");
        return (file_path.clone(), std::fs::OpenOptions::new().append(true).create(true).read(true).write(true).open(file_path).unwrap());
    }

    let class_instance = format!("Class{}-{}", last_class_num + 1, current_iso_8601);
    let class_path =
        if week_entries.len() >= class.get_times().len() {
            let new_path = course_path.join(format!("Week_{}", latest_week + 1));
            if !std::fs::exists(&new_path).expect("Unable to check for note folder path existence.") {
                std::fs::create_dir(&new_path).expect("Unable to create new week directory.");
            }

            new_path
        } else {
            course_path.join(format!("Week_{latest_week}"))
        }
        .join(&class_instance)
    ;

    std::fs::create_dir_all(&class_path).expect("Failed to create class directory.");

    let file_path = class_path.join(class_instance + ".md");
    // file_path.add_extension("md");

    return (file_path.clone(), std::fs::OpenOptions::new().append(true).create(true).read(true).write(true).open(file_path).unwrap());
}

pub fn open_note(config: Config) {
    // determine class based on times from config.
    let curr_time = Time::now();
    let curr_class: Option<Class> = config.get_classes().iter().filter_map(|class|
        match class.get_times() {
            Times::Async => None,
            Times::At(times) => times.iter().any(|time| time.includes(curr_time)).then_some(class.clone())
        }
    ).next();

    // if there's no class at this time, say so (perhaps determine other functionality in the future)
    let Some(class) = curr_class else {
        eprintln!("No class found for {curr_time}");
        return;
    };

    let (path, _note) = get_current_classnote(&config, &class);
    
    // perhaps later implement your own editor?
    let status = match config.get_editor() {
        Editor::Simple(program) => 
            Command::new(program)
                .arg(path)
                .status()
        ,

        Editor::Complex { program, uri } => 
            Command::new(program)
                .arg(ObsidianPath::build_uri(&uri.vault, path))
                .status()
            
    }.expect("Failed to start editor.");

    println!("{} exited {}.",
        config.get_editor_program(),
        if status.success() {
            "successfully"
        } else {
            "with an error."
        }
    );
}
