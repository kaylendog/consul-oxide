use serde::de::DeserializeOwned;

use crate::Result;

/// Utility trait for making HTTP requests.
#[async_trait::async_trait]
pub(crate) trait Http {
    /// Returns a reference to the `reqwest::Client` used to make HTTP requests.
    fn inner(&self) -> (&reqwest::Client, &crate::Config);

    /// Makes a GET request to the given URL and returns the response.
    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let (client, config) = self.inner();
        let url = format!("{}/v1/{}", config.address, url);
        let response = client.get(&url).send().await?;
        let response: T = response.json().await?;
        Ok(response)
    }

    /// Makes a GET request to the given URL and returns the response. If
    /// the response is a 404, returns `None`.
    async fn get_empty<T: DeserializeOwned>(&self, url: &str) -> Result<Option<T>> {
        let (client, config) = self.inner();
        let url = format!("{}/v1/{}", config.address, url);
        let response = client.get(&url).send().await?;
        // check length
        if response.content_length().is_none() || response.content_length().unwrap() == 0 {
            return Ok(None);
        }
        let response: T = response.json().await?;
        Ok(Some(response))
    }
}
