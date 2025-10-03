use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub dest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub languages: Vec<String>,
    pub status: String,
    pub source: String,
    pub links: Vec<Link>,
    pub tags: Vec<String>,
}

impl Project {
    pub fn load() -> io::Result<Vec<Self>> {
        let data: Vec<Self> = json5::from_str(&fs::read_to_string("projects.json5")?)
            .expect("Since this is not a public application, I have no concern about it panicing.");
        Ok(data)
    }
}
