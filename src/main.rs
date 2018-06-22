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

fn get(url: &String) -> String {
    reqwest::get(url).unwrap().text().unwrap()
}

fn filename(url: &String) -> String {
    let idx = url.rfind('/').unwrap() + 1;
    let slice: String = url.chars().skip(idx).collect();
    format!("./data/{}", slice)
}

fn main() {
    let files = get_img();

    files.par_iter().for_each(|url: &String| {
        let file = filename(url);
        if !std::path::Path::new(file).exists() {
            println!("Writing {}", file);
            let body = get(url);
            std::fs::write(file, body);
        }
    });
}
