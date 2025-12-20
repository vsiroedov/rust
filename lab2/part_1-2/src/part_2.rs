use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicTariff {
    pub id: u32,
    pub price: u32,
    pub duration: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateTariff {
    pub client_price: u32,
    pub duration: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Stream {
    pub user_id: String,
    pub is_private: bool,
    pub settings: u32,
    pub shard_url: String,
    pub public_tariff: PublicTariff,
    pub private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Gift {
    pub id: u32,
    pub price: u32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DebugInfo {
    pub duration: String,
    pub at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Request {
    #[serde(rename = "type")] 
    pub request_type: String,
    pub stream: Stream,
    pub gifts: Vec<Gift>,
    pub debug: DebugInfo,
}

pub fn convert_json_to_toml(json_str: &str) -> String {
    let request: Request = serde_json::from_str(json_str).expect("Failed to parse JSON");
    toml::to_string(&request).expect("Failed to serialize to TOML")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_json() {
        let json_data = r#"{
            "type": "success",
            "stream": {
                "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
                "is_private": false,
                "settings": 45345,
                "shard_url": "https://n3.example.com/sapi",
                "public_tariff": {
                    "id": 1,
                    "price": 100,
                    "duration": "1h",
                    "description": "test public tariff"
                },
                "private_tariff": {
                    "client_price": 250,
                    "duration": "1m",
                    "description": "test private tariff"
                }
            },
            "gifts": [{
                "id": 1,
                "price": 2,
                "description": "Gift 1"
            }],
            "debug": {
                "duration": "234ms",
                "at": "2019-06-28T08:35:46+00:00"
            }
        }"#;

        let toml_output = convert_json_to_toml(json_data);
        println!("{}", toml_output);

        assert!(toml_output.contains("type = \"success\""));
        assert!(toml_output.contains("user_id = \"8d234120-0bda-49b2-b7e0-fbd3912f6cbf\""));
        assert!(toml_output.contains("client_price = 250"));
    }
}