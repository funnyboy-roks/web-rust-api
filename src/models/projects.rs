use serde::{Deserialize, Serialize};
use std::{fs, io};
use strum::EnumVariantNames;

#[derive(Debug, Clone, Serialize, Deserialize, EnumVariantNames)]
pub enum Language {
    Rust,
    Java,
    JavaScript,
    TypeScript,
    Svelte,
    Python,
    HTML,
    CSS,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    /// Project is complete
    Complete,
    /// Project is WIP
    WIP,
    /// Project will be continuiously updated
    NeverEnding,
    /// Project is no longer supported
    Unsupported,
    /// Project complete but still maintained
    Maintained,
    /// Project archived
    Archived,
}

type URL = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub dest: URL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub languages: Vec<Language>,
    pub status: ProjectStatus,
    pub source: URL,
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
