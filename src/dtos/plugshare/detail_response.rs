use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DetailResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub address: String,
    pub formatted_phone_number: String,
    pub hours: String,

    pub stations: Vec<Station>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Station {
    pub id: u64,
    pub name: Option<String>,
    pub outlets: Vec<Outlet>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Outlet {
    pub id: u64,

    #[serde(deserialize_with = "plugshare_connector_type")]
    pub connector: String,

    pub status: Option<String>,
}

fn plugshare_connector_type<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match u16::deserialize(deserializer)? {
        1 => Ok("US Wall Outlet".to_string()),
        2 => Ok("J-1772".to_string()),
        3 => Ok("CHAdeMO".to_string()),
        4 => Ok("Tesla Roadster".to_string()),
        5 => Ok("NEMA 14-50".to_string()),
        6 => Ok("Tesla Supercharger".to_string()),
        7 => Ok("Type 2 (Mennekes)".to_string()),
        8 => Ok("Type 3".to_string()),
        9 => Ok("BS1363".to_string()),
        10 => Ok("Europlug".to_string()),
        11 => Ok("UK Commando".to_string()),
        12 => Ok("AS3112".to_string()),
        13 => Ok("SAE Combo DC CCS".to_string()),
        14 => Ok("Three Phase (AU - EU)".to_string()),
        15 => Ok("Caravan Mains Socket".to_string()),
        16 => Ok("GB/T".to_string()),
        17 => Ok("GB/T 2".to_string()),
        _ => Ok("Unknown".to_string()),
    }
}
