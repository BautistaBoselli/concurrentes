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

    let mut general_data: HashMap<String, i64> = HashMap::new();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    for name in names {
        process_file(name, &mut general_data);
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("Time: {:?}", end - start);
}

fn process_file(name: &str, general_data: &mut HashMap<String, i64>) {
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

    for (key, value) in data {
        let general_value = general_data.entry(key).or_insert(0);
        *general_value += value;
    }
}
