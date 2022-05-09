use lazy_static::lazy_static;
use std::{collections::HashMap, fs::File};

// use tokio::fs::File;

// pub struct Language {
//     pub storage: HashMap<String, String>,
// }

// impl Language {
//     pub async fn load() -> Self {
//         // TODO: download from mojang's servers and cache somewhere

//         let mut storage = HashMap::new();
//         let mut file = File::open("en_us.json").unwrap();
//         let mut contents = String::new();
//         file.read_to_string(&mut contents).unwrap();
//         let en_us: HashMap<String, String> = serde_json::from_str(&contents).unwrap();
//         Language { storage: en_us }
//     }

//     pub fn get(&self, key: &str) -> Option<&str> {
//         self.storage.get(key)
//     }
// }

// yeah i just decided to do this because otherwise we would have to have a
// Language object that we passed around everywhere which is not convenient
// The code above is kept in case I come up with a better solution

lazy_static! {
    pub static ref STORAGE: HashMap<String, String> = serde_json::from_str({
        let mut file = File::open("en_us.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    })
    .unwrap();
}

pub fn get(key: &str) -> Option<&str> {
    STORAGE.get(key).map(|s| s.as_str())
}
