use csv::Reader;
use std::{
    collections::HashMap,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    println!("Hello, world!");

    let names = vec![
        "CAvideos", "DEvideos", "FRvideos", "GBvideos", "INvideos", "JPvideos", "KRvideos",
        "MXvideos", "RUvideos", "USvideos",
    ];

    let mut handles = vec![];
    let mut general_data: HashMap<String, i64> = HashMap::new();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    for name in names {
        let handle = thread::spawn(move || process_file(name));
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        for (key, value) in result {
            let general_value = general_data.entry(key).or_insert(0);
            *general_value += value;
        }
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("Time: {:?}", end - start);

    println!("Cantidad: {:?}", general_data.len());
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
