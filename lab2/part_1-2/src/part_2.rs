use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request {
    pub id: u32,
    pub title: String,
    pub tags: Vec<String>,
}

pub fn convert_json_to_toml(json_str: &str) -> String {
    let request: Request = serde_json::from_str(json_str).expect("Failed to parse JSON");
    toml::to_string(&request).expect("Failed to format TOML")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conversion() {
        let json = r#"{"id": 42, "title": "Lab 2", "tags": ["rust", "types"]}"#;
        let toml_out = convert_json_to_toml(json);
        assert!(toml_out.contains("id = 42"));
        assert!(toml_out.contains(r#"title = "Lab 2""#));
    }
}