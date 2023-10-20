use std::time::{SystemTime, UNIX_EPOCH};

use crate::reqwest::Error;
use async_std::task;
use futures::join;
use reqwest;

use serde_json::Value;

//Intentar:
// Crear un vector con el codigo de 5 paises y mostrar las 10 aves mas observadas de cada uno
// Ver en la clase bien como trabajar con multiples futures

async fn process(region_code: String) -> Result<String, Error> {
    let client = reqwest::Client::new();
    // Define the URL of the API you want to send a request to.
    let url = format!("https://api.ebird.org/v2/data/obs/{}/recent", region_code);

    // Send a GET request to the URL.
    let response = client
        .get(url)
        .header("X-eBirdApiToken", "rvpet2ra8kpc")
        .send()
        .await?;

    // Check if the response was successful (status code 200).
    // if response.status().is_success() {
    //     // Read the response as a string and print it.
    //     let json_data: Value = response.json().await?;

    //     let mut i = 0;
    //     for item in json_data.as_array().unwrap_or(&vec![]) {
    //         i += 1;
    //         let com_name = item["comName"].as_str().unwrap_or("N/A");
    //         information.push(com_name);
    //         println!("Common Name: {}", com_name);
    //         if i == 10 {
    //             break;
    //         }
    //     }
    // } else {
    //     println!("Request failed with status code: {}", response.status());
    // }

    // Ok(information)

    if response.status().is_success() {
        // Parse the JSON response as a Value.
        let json_data: Value = response.json().await?;

        // Initialize a vector to store common names.
        let mut common_names: Vec<&str> = Vec::new();

        let mut i = 0;
        // Iterate over the JSON data and extract common names.
        if let Some(json_array) = json_data.as_array() {
            for item in json_array {
                i += 1;
                if let Some(com_name) = item["comName"].as_str() {
                    common_names.push(com_name);
                }
                if i == 5 {
                    break;
                }
            }
        }

        // Return the vector of common names.
        let mut information = common_names.join(" | ");
        information.push_str(" || \n");
        return Ok(information);
    } else {
        println!("Request failed with status code: {}", response.status());
        return Ok(String::from("N/A"));
    }
}

async fn async_main() -> Result<(), Error> {
    let str_result = process("AR".to_string()).await.unwrap()
        + &process("US".to_string()).await.unwrap()
        + &process("BR".to_string()).await.unwrap()
        + &process("UY".to_string()).await.unwrap()
        + &process("CL".to_string()).await.unwrap();
    println!("{}", str_result);
    Ok(())
}

async fn async_aux() -> Result<(), Error> {
    let (r1, r2, r3, r4, r5) = join!(
        process("AR".to_string()),
        process("US".to_string()),
        process("BR".to_string()),
        process("UY".to_string()),
        process("CL".to_string())
    );
    println!(
        "{}",
        r1.unwrap() + &r2.unwrap() + &r3.unwrap() + &r4.unwrap() + &r5.unwrap()
    );

    Ok(())
}

pub fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let _ = task::block_on(async_main());
    // let _ = task::block_on(async_aux());
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?}", end - start);
}
