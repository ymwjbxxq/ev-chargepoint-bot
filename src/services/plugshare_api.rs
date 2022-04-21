use std::cmp::Ordering;

use crate::{dtos::plugshare::{nearby_response::NearByResponse, detail_response::DetailResponse}, error::ApplicationError};
use async_trait::async_trait;

#[async_trait]
pub trait EvApi {
    fn new() -> Self;
    async fn get_locations(&self, latitude: f64, longitude: f64) -> Result<Vec<NearByResponse>, ApplicationError>;
    async fn get_location(&self, location_id: i64) -> Result<DetailResponse, ApplicationError>;
}

#[derive(Debug)]
pub struct PlugShareApi {
    base_url: String,
    api_key: String,
}

#[async_trait]
impl EvApi for PlugShareApi {
    fn new() -> Self {
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
        let base_url = "https://api.plugshare.com/".to_string();

        Self { base_url, api_key }
    }

    async fn get_locations(&self, latitude: f64, longitude: f64) -> Result<Vec<NearByResponse>, ApplicationError> {
        let url = format!(
            "{}locations/nearby?latitude={}&longitude={}&radius=2000&count=15",
            self.base_url, latitude, longitude
        );

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header(
                "Authorization",
                format!("{} {}", "Basic".to_owned(), self.api_key),
            )
            .send()
            .await?;

        if res.status() != 200 {
            return Err(ApplicationError::InternalError(format!(
                "cannot get stations {}",
                res.status()
            )));
        }

        let mut stations = res.json::<Vec<NearByResponse>>().await?;
        stations.sort_by(|a, b| {
            if a.distance_meters < b.distance_meters {
                Ordering::Less
            } else if a.distance_meters == b.distance_meters {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });

        Ok(stations)
    }

    async fn get_location(&self, location_id: i64) -> Result<DetailResponse, ApplicationError> {
        let url = format!("{}v3/locations/{}", self.base_url, location_id);
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header(
                "Authorization",
                format!("{} {}", "Basic".to_owned(), self.api_key),
            )
            .send()
            .await?;

        if res.status() != 200 {
            return Err(ApplicationError::InternalError(format!(
                "cannot get station details {}",
                res.status()
            )));
        }

        let station_details = res.json::<DetailResponse>().await?;

        Ok(station_details)
    }
}
