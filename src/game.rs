use crate::helper::*;
use serde::{Deserialize, Serialize};

//================================================================

#[derive(Clone)]
pub struct Game {
    pub info: Info,
    pub data: Option<Data>,
    pub path: String,
}

impl Game {
    pub const RESOURCE_PATH: &'static str = "resource";

    pub fn new(info: Info, data: Option<Data>, path: String) -> Self {
        Self { info, data, path }
    }

    pub fn new_list() -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        let path = std::fs::read_dir(Self::RESOURCE_PATH).unwrap();

        for file in path {
            let path = file.unwrap().path().display().to_string();

            result.push(Game::new(
                Info::new_from_file(&path),
                Data::new_from_file(&path),
                path,
            ));
        }

        result
    }
}

//================================================================

#[derive(Clone, Deserialize)]
pub struct Info {
    pub name: String,
}

impl Info {
    pub const FILE_NAME: &'static str = "info.json";

    pub fn new_from_file(path: &str) -> Self {
        let path = format!("{path}/{}", Self::FILE_NAME);

        let data = std::fs::read_to_string(path)
            .map_err(|e| panic(&format!("Info::new_from_file(): {e}")))
            .unwrap();
        serde_json::from_str(&data)
            .map_err(|e| panic(&format!("Info::new_from_file(): {e}")))
            .unwrap()
    }
}

//================================================================

#[derive(Clone, Deserialize, Serialize)]
pub struct Data {
    pub path: String,
}

impl Data {
    pub const FILE_NAME: &'static str = "data.json";

    pub fn new_from_file(path: &str) -> Option<Self> {
        let path = format!("{path}/{}", Self::FILE_NAME);

        if std::path::Path::new(&path).is_file() {
            let data = std::fs::read_to_string(path)
                .map_err(|e| panic(&format!("Data::new_from_file(): {e}")))
                .unwrap();
            serde_json::from_str(&data)
                .map_err(|e| panic(&format!("Data::new_from_file(): {e}")))
                .unwrap()
        } else {
            None
        }
    }
}