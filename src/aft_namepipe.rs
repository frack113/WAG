//
// Name Pipe Artefact
//

use serde::Deserialize;
use std::collections::{HashMap, HashSet};

//Structure for the Json
#[derive(Deserialize)]
struct JsonNamepipeInfo {
    name: String,
    namepipe: Vec<String>,
}

#[derive(Deserialize)]
struct JsonGlobalInfo {
    dictionary: Vec<JsonNamepipeInfo>,
}

//Structure form the name pipe
// Very simple struct
#[derive(Clone)]
pub struct NamePipeArtefact {
    namepipe: HashMap<String, Vec<String>>,
}

impl NamePipeArtefact {
    pub fn new() -> Self {
        Self {
            namepipe: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        let file_path = std::env::current_dir()
            .expect("Failed to get current folder")
            .join(path);
        let json_open = std::fs::File::open(file_path).expect("Unable to open json file");
        let json_data: JsonGlobalInfo =
            serde_json::from_reader(json_open).expect("error while reading or parsing the json");

        for data in json_data.dictionary {
            self.namepipe.insert(data.name, data.namepipe);
        }
    }

    pub fn namepipe_list(&self) -> HashSet<String> {
        self.namepipe.keys().cloned().collect()
    }

    pub fn namepipe_value_list(&self, name: &str) -> Vec<String> {
        self.namepipe.get(name).unwrap().to_vec()
    }

    pub fn namepipe_exist(&self, name: &str) -> bool {
        self.namepipe.contains_key(name)
    }

    pub fn namepipe_get_value_at_index(&self, name: &str, number: usize) -> String {
        let mut index_payload = number;
        let namepipe_value_list = self.namepipe_value_list(name);
        //Don't trust humain
        if index_payload > namepipe_value_list.len() - 1 {
            index_payload = 0;
        }
        let barrow = &namepipe_value_list[index_payload];
        barrow.to_owned()
    }
}
