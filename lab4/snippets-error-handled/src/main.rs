use std::{env, fs, io::{self, Read}};
use rusqlite::{params, Connection};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use anyhow::{Context, Result, anyhow}; 

#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    name: String,
    content: String,
    created_at: String,
}

trait SnippetBackend {
    fn save(&mut self, name: String, content: String) -> Result<()>;
    fn get(&self, name: &str) -> Result<Option<String>>;
}

struct SqliteBackend { conn: Connection }
impl SnippetBackend for SqliteBackend {
    fn save(&mut self, name: String, content: String) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (name TEXT PRIMARY KEY, content TEXT, created_at TEXT)",
            [],
        ).context("Failed to create snippets table in SQLite")?;

        self.conn.execute(
            "INSERT OR REPLACE INTO snippets (name, content, created_at) VALUES (?1, ?2, ?3)",
            params![name, content, Utc::now().to_rfc3339()],
        ).with_context(|| format!("Failed to insert snippet '{}' into database", name))?;
        
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Option<String>> {
        let res = self.conn.query_row(
            "SELECT content FROM snippets WHERE name = ?1",
            [name],
            |r| r.get(0)
        );
        
        match res {
            Ok(content) => Ok(Some(content)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e).context(format!("Error querying snippet '{}'", name))),
        }
    }
}

struct JsonBackend { path: String }
impl SnippetBackend for JsonBackend {
    fn save(&mut self, name: String, content: String) -> Result<()> {
        let content_file = fs::read_to_string(&self.path).unwrap_or_else(|_| "[]".to_string());
        let mut snippets: Vec<Snippet> = serde_json::from_str(&content_file)
            .context("Failed to parse JSON storage file")?;

        snippets.push(Snippet { name: name.clone(), content, created_at: Utc::now().to_rfc3339() });
        
        let json = serde_json::to_string(&snippets).context("Failed to serialize snippets to JSON")?;
        fs::write(&self.path, json).with_context(|| format!("Failed to write data to file: {}", self.path))?;
        
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Option<String>> {
        if !std::path::Path::new(&self.path).exists() {
            return Ok(None);
        }
        let content_file = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read JSON file: {}", self.path))?;
        let snippets: Vec<Snippet> = serde_json::from_str(&content_file)?;
        
        Ok(snippets.into_iter().find(|s| s.name == name).map(|s| s.content))
    }
}

fn main() -> Result<()> {
    let env_val = env::var("SNIPPETS_APP_STORAGE")
        .context("Environment variable SNIPPETS_APP_STORAGE is not set. Expected format 'PROVIDER:PATH'")?;

    let (provider, path) = env_val.split_once(':')
        .ok_or_else(|| anyhow!("Invalid storage format. Expected 'PROVIDER:PATH', got '{}'", env_val))?;

    let mut backend: Box<dyn SnippetBackend> = match provider {
        "JSON" => Box::new(JsonBackend { path: path.to_string() }),
        "SQLITE" => {
            let conn = Connection::open(path)
                .with_context(|| format!("Failed to open SQLite database at {}", path))?;
            Box::new(SqliteBackend { conn })
        },
        _ => return Err(anyhow!("Unsupported storage provider: {}", provider)),
    };

    let args: Vec<String> = env::args().collect();

    if let Some(pos) = args.iter().position(|x| x == "--name") {
        let name = args.get(pos + 1).ok_or_else(|| anyhow!("Missing value for --name"))?;
        let mut content = String::new();
        io::stdin().read_to_string(&mut content).context("Failed to read from stdin")?;
        
        backend.save(name.clone(), content)?;
        println!("Snippet '{}' successfully saved using {} backend.", name, provider);

    } else if let Some(pos) = args.iter().position(|x| x == "--read") {
        let name = args.get(pos + 1).ok_or_else(|| anyhow!("Missing value for --read"))?;
        match backend.get(name)? {
            Some(content) => println!("{}", content),
            None => return Err(anyhow!("Snippet '{}' not found", name)),
        }
    } else {
        println!("Usage: set SNIPPETS_APP_STORAGE and use --name <name> or --read <name>");
    }

    Ok(())
}