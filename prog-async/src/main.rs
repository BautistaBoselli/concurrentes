use crate::reqwest::Error;
use async_std::task;
use reqwest;

use serde_json::Value;

//Intentar:
// Crear un vector con el codigo de 5 paises y mostrar las 10 aves mas observadas de cada uno
// Ver en la clase bien como trabajar con multiples futures

async fn process() -> Result<(), Error> {
    let client = reqwest::Client::new();
    // Define the URL of the API you want to send a request to.
    let url = "https://api.ebird.org/v2/data/obs/US/recent";

    // Send a GET request to the URL.
    let response = client
        .get(url)
        .header("X-eBirdApiToken", "rvpet2ra8kpc")
        .send()
        .await?;

    // Check if the response was successful (status code 200).
    if response.status().is_success() {
        // Read the response as a string and print it.
        let json_data: Value = response.json().await?;

        let mut i = 0;
        for item in json_data.as_array().unwrap_or(&vec![]) {
            i += 1;
            let com_name = item["comName"].as_str().unwrap_or("N/A");
            println!("Common Name: {}", com_name);
            if i == 10 {
                break;
            }
        }
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

pub fn main() {
    let _ = task::block_on(process());
}
