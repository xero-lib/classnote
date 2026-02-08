use std::path::PathBuf;

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
        self.editor.clone()
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    pub fn serialize(&self) -> String {
        toml::to_string(self).expect("Encountered error during serialization of TOML data.")
    }
}