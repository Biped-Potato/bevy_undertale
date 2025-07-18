use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::process::exit;
use toml;

pub fn load_contents(filename: String) -> String {
    let contents = match fs::read_to_string(filename.clone()) {
        Ok(c) => c,
        Err(_) => {
            log::info!("Could not read file `{}`", filename);
            "".to_string()
        }
    };
    return contents;
}

pub fn read_toml<T>(contents: String) -> Option<T>
where
    T: DeserializeOwned,
{
    let data: Option<T> = match toml::from_str(&contents) {
        Ok(d) => Some(d),
        Err(_) => {
            log::info!("Unable to load data");
            None
        }
    };
    return data;
}
