use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct NearByResponse {
    pub id: u64,
    pub address: String,
    pub name: String,
    pub distance_meters: f64,
    pub stations: Vec<Station>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Station {
    #[serde(deserialize_with = "plugshare_type")]
    pub available: String,
}

fn plugshare_type<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match u16::deserialize(deserializer)? {
        1 => Ok("Available".to_string()),
        2 => Ok("In Use".to_string()),
        3 => Ok("Offline".to_string()),
        4 => Ok("Under Repair".to_string()),
        _ => Ok("Unknown".to_string()),
    }
}
