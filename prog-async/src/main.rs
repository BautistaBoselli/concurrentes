use crate::reqwest::Error;
use async_std::task;
use reqwest;

async fn process() -> Result<(), Error> {
    let client = reqwest::Client::new();
    // Define the URL of the API you want to send a request to.
    let url = "https://api.ebird.org/v2/data/obs/CA/recent";

    // Send a GET request to the URL.
    let response = client
        .get(url)
        .header("X-eBirdApiToken", "rvpet2ra8kpc")
        .send()
        .await?;

    // Check if the response was successful (status code 200).
    if response.status().is_success() {
        // Read the response as a string and print it.
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

pub fn main() {
    let _ = task::block_on(process());
}
