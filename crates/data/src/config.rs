use std::{io::{Write, stdout}, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::obsidian::ObsidianPath;

use super::class::Class;

#[derive(Debug, Serialize, Deserialize)]
pub enum Editor {
    Simple(String),
    Complex {
        program: String,
        uri: ObsidianPath
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::Simple(String::new())
    }
}

impl Editor {
    pub fn is_simple(&self) -> bool {
        match &self {
            Editor::Simple(_) => true,
            _ => false
        }
    }

    pub fn get_program(&self) -> &String {
        match &self {
            Self::Simple(e) => e,
            Self::Complex { program, .. } => program
        }
    }

    pub fn set_program(&mut self, program: String) {
        match self {
            Self::Simple(p) => *p = program,
            Self::Complex { program: p, .. } => *p = program
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    root: PathBuf,
    classes: Vec<Class>,
    editor: Editor,
}

impl Config {

    pub fn new(root: PathBuf, classes: Vec<Class>, editor: String) -> Config {
        Config { root, classes, editor: Editor::Simple(editor) }
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

    pub fn set_editor(&mut self, editor: Editor) {
        self.editor = editor;
    }

    pub fn get_editor(&self) -> &Editor {
        &self.editor
    }

    pub fn get_editor_program(&self) -> String {
        let editor = match &self.editor {
            Editor::Simple(e) => e,
            Editor::Complex { program: e, .. } => e
        };

        if !editor.is_empty() {
            editor.clone()
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