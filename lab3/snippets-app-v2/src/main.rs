use std::{env, fs, io::{self, Read}};
use rusqlite::{params, Connection};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    name: String,
    content: String,
    created_at: String,
}

trait SnippetBackend {
    fn save(&mut self, name: String, content: String);
    fn get(&self, name: &str) -> Option<String>;
    fn delete(&mut self, name: &str);
}

struct SqliteBackend { conn: Connection }
impl SnippetBackend for SqliteBackend {
    fn save(&mut self, name: String, content: String) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (name TEXT PRIMARY KEY, content TEXT, created_at TEXT)",
            [],
        ).unwrap();
        self.conn.execute(
            "INSERT OR REPLACE INTO snippets (name, content, created_at) VALUES (?1, ?2, ?3)",
            params![name, content, Utc::now().to_rfc3339()],
        ).unwrap();
    }
    fn get(&self, name: &str) -> Option<String> {
        self.conn.query_row("SELECT content FROM snippets WHERE name = ?1", [name], |r| r.get(0)).ok()
    }
    fn delete(&mut self, name: &str) {
        let _ = self.conn.execute("DELETE FROM snippets WHERE name = ?1", [name]);
    }
}

struct JsonBackend { path: String }
impl SnippetBackend for JsonBackend {
    fn save(&mut self, name: String, content: String) {
        let mut snippets: Vec<Snippet> = fs::read_to_string(&self.path)
            .ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
        snippets.push(Snippet { name, content, created_at: Utc::now().to_rfc3339() });
        fs::write(&self.path, serde_json::to_string(&snippets).unwrap()).unwrap();
    }
    fn get(&self, name: &str) -> Option<String> {
        let snippets: Vec<Snippet> = fs::read_to_string(&self.path)
            .ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
        snippets.into_iter().find(|s| s.name == name).map(|s| s.content)
    }
    fn delete(&mut self, name: &str) {  }
}

fn main() {
    let env_val = env::var("SNIPPETS_APP_STORAGE").expect("Set SNIPPETS_APP_STORAGE env var");
    let (provider, path) = env_val.split_once(':').expect("Format must be PROVIDER:PATH");

    let mut backend: Box<dyn SnippetBackend> = match provider {
        "JSON" => Box::new(JsonBackend { path: path.to_string() }),
        "SQLITE" => Box::new(SqliteBackend { conn: Connection::open(path).unwrap() }),
        _ => panic!("Unknown provider"),
    };

    let args: Vec<String> = env::args().collect();
    if let Some(pos) = args.iter().position(|x| x == "--name") {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content).unwrap();
        backend.save(args[pos + 1].clone(), content);
        println!("Saved to {}", provider);
    } else if let Some(pos) = args.iter().position(|x| x == "--read") {
        if let Some(c) = backend.get(&args[pos + 1]) { println!("{}", c); }
    }
}