use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub default: Section,
}
#[derive(Debug, Deserialize)]
pub struct Section {
    pub prompts: Prompt,
}

#[derive(Debug, Deserialize)]
pub struct Prompt {
    pub summarisation: String,
}

pub fn get_settings(settings_file_path: Option<&str>) -> Settings {
    let yaml_file = fs::read_to_string(settings_file_path.unwrap_or("settings.yaml")).unwrap();
    let settings: Settings = serde_yaml::from_str(&yaml_file).unwrap();

    settings
}
