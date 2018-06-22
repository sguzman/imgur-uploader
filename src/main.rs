extern crate rayon;
extern crate reqwest;
extern crate serde_json;

use serde_json::{Value, Error};
use rayon::prelude::*;

fn get_img() -> Vec<String> {
    let files = std::fs::read_dir("./rusty-books/data/").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for f in files {
        let f = f.unwrap();
        let f = std::fs::read_to_string(f.path());
        let f = f.unwrap();
        let f: Result<Value, Error> = serde_json::from_str(&f);
        let f = f.unwrap();
        let f = f.get("img");
        let f = f.unwrap();
        let f = f.as_str();
        let f = f.unwrap();
        vec.push(f.clone().to_string());
    }

    vec
}

fn main() {
    let files = get_img();

    for f in files {
        println!("{}", f);
    }
}
