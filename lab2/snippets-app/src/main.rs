use std::env;
use std::fs;
use std::io::{self, Read};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
struct SnippetStorage {
    data: HashMap<String, String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let db_path = "snippets.json";

    let mut storage: SnippetStorage = fs::read_to_string(db_path)
        .map(|s| serde_json::from_str(&s).unwrap_or_default())
        .unwrap_or_default();

    if let Some(pos) = args.iter().position(|x| x == "--name") {
        let name = args.get(pos + 1).expect("Provide a name").clone();
        let mut content = String::new();
        io::stdin().read_to_string(&mut content).unwrap();
        storage.data.insert(name, content);
        println!("Snippet saved.");
    } 
    else if let Some(pos) = args.iter().position(|x| x == "--read") {
        let name = args.get(pos + 1).expect("Provide a name");
        match storage.data.get(name) {
            Some(c) => print!("{}", c),
            None => println!("Not found."),
        }
    } 
    else if let Some(pos) = args.iter().position(|x| x == "--delete") {
        let name = args.get(pos + 1).expect("Provide a name");
        storage.data.remove(name);
        println!("Deleted.");
    }

    let json = serde_json::to_string_pretty(&storage).unwrap();
    fs::write(db_path, json).expect("Failed to write file");
}