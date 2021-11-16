mod state;

use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};
/*
mod to_do;

use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    let done: Done = Done::new("shopping");
    println!("Title is:  {}", done.super_struct.title);
    println!("Status is: {}", done.super_struct.status);

    let pending = Pending::new("laundry");
    println!("Title is:  {}", pending.super_struct.title);
    println!("Status is: {}", pending.super_struct.status);

}
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file(&String::from("./state.json"));
    println!("{:?}", state);
    state.insert(title.to_string(), json!(status));
    write_to_file("./state.json", &mut state);

}