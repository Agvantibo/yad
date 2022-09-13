use random_string::generate;
use std::io;
use std::fs;
use walkdir::WalkDir;
use std::fs::metadata;

fn main() {
    let rcharset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut filename: String = String::from("yad");
    let randstr = generate(16 - filename.chars().count(), rcharset);
    filename.push_str(&randstr);
    for entry in WalkDir::new(".") {
    	let probably_writable = entry.unwrap_or_else(|error| {continue}).path();
        if metadata(probably_writable).unwrap().is_dir() {
        	
        }
    }
    
}
