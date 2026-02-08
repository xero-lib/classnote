use std::{io::{Write, stdout}, path::PathBuf};

use serde::{Serialize, Deserialize};

use super::class::Class;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    root: PathBuf,
    classes: Vec<Class>,
    editor: String,
}

impl Config {

    pub fn new(root: PathBuf, classes: Vec<Class>, editor: String) -> Config {
        Config { root, classes, editor }
    }
    pub fn get_classes(&self) -> &Vec<Class> {
        &self.classes
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = root;
    }

    pub fn get_root(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn set_editor(&mut self, editor: String) {
        self.editor = editor;
    }

    pub fn get_editor(&self) -> String {
        if !self.editor.is_empty() {
            self.editor.clone()
        } else {
            match std::env::var("EDITOR") {
                Ok(e) => e,
                Err(_) => {
                    eprintln!("No editor set, defaulting to vim.");
                    "vim".into()
                }
            }
        }
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    pub fn serialize(&self) -> String {
        toml::to_string(self).expect("Encountered error during serialization of TOML data.")
    }

    pub fn print_available_classes(&self) {
        print!("Available classes:\n{}", self.get_classes().iter().fold(String::new(), |init, class| 
            init + &class.to_string()
        ));

        stdout().flush().expect("Unable to flush stdout.");
    }
}