use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;
use std::fs::File;
use std::fs;
use std::io::Read;


pub fn read_file(file_name: &str) -> Option<Map<String, Value>> {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Result<Value, _> = serde_json::from_str(&data);
    match json {
        Ok(j) => {
            let state: Map<String, Value> = j.as_object().unwrap().clone();
            Some(state)
        },
        Err(e) => {
            println!("Error parsing: {:?}", e);
            None
        }
    }
}

pub fn write_to_file(file_name: &str, state:&mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to write file");
}