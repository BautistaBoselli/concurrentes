use csv::Reader;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    println!("Hello, world!");

    let names = vec![
        "CAvideos", "DEvideos", "FRvideos", "GBvideos", "INvideos", "JPvideos", "KRvideos",
        "MXvideos", "RUvideos", "USvideos",
    ];

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let result = names.iter().map(|name| process_file(name)).fold(
        HashMap::new(),
        |mut acc: HashMap<String, i64>, h2: HashMap<String, i64>| {
            h2.iter().for_each(|(key, value)| {
                let general_value = acc.entry(key.to_string()).or_insert(0);
                *general_value += value;
            });
            acc
        },
    );

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("Time: {:?}", end - start);

    println!("Cantidad: {:?}", result.len());
}

fn process_file(name: &str) -> HashMap<String, i64> {
    let path = format!("./archive/{}.csv", name);
    let mut reader = Reader::from_path(path).unwrap();
    let mut data: HashMap<String, i64> = HashMap::new();

    for result in reader.records() {
        if result.is_err() {
            // ignore
            continue;
        }
        let record = result.unwrap();

        let channel = record.get(3).unwrap().to_string();
        let views = record.get(7).unwrap().parse::<i64>().unwrap();

        let value = data.entry(channel).or_insert(0);
        *value += views;
    }

    data
}
