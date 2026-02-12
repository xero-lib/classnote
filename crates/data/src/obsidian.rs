use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObsidianPath {
    pub vault: String,
    pub courses_root: PathBuf
}

impl ObsidianPath {
    pub fn to_uri(&self) -> PathBuf {
        PathBuf::from_str(
            &format!(
                "obsidian://open?vault={}&file={}",
                self.vault
                    .replace(' ', "%20"),
                self.courses_root
                    .to_string_lossy()
                    .replace(' ', "%20")
                    .replace('/', "%2F")
                // "obsidian://open?path={}"
            )
        ).expect("Unable to build Obsidian URI.")
    }

    pub fn build_uri(vault: &String, note_path: PathBuf) -> String {
        format!("obsidian://open?vault={}&file={}",
            vault
                .replace(' ', "%20")
                .replace('/', "%20")
            ,
            // ? There has to be a better way
            note_path
                .iter()
                .skip_while(|&d| d != PathBuf::from(vault)) // trim path until the vault name
                .skip(1) // also trim the vault name
                .filter_map(|osstr| osstr.to_str().and_then(|s| Some(s.to_string()))) // better way?
                .collect::<Vec<String>>()
                .join("%2F")
                .replace(' ', "%20")
                .replace('/', "%2F")
        )
    }
}