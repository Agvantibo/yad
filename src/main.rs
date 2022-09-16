use random_string::generate;
//use std::io;
//use std::fs;
use walkdir::WalkDir;
use std::fs::metadata;

fn main() {
    let rcharset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut filename: String = String::from("yad");
    let randstr = generate(16 - filename.chars().count(), rcharset);
    let excludes = ["proc", "run", "sys", "var", "cache", "tmp", "Windows"];
	let mut all_suitable_paths: Vec<std::path::PathBuf> = vec![];
    let mut skip_flag;
    filename.push_str(&randstr);
    for entry in WalkDir::new("/") {
    	skip_flag = false;
    	let probably_writable = match entry {
    		Ok(path) => path,
    		Err(_) => continue,
    	};
    	for badname in excludes {
    		if probably_writable.path().to_str().expect("Something has gone terribly wrong").to_string().contains(&badname){
    			skip_flag = true;
    		}
    	}
    	if skip_flag {continue}
        if  match metadata(probably_writable.path()) {
    		Ok(path) => path,
    		Err(_) => continue,
    	}.is_dir() {
        	all_suitable_paths.push(std::path::PathBuf::from(probably_writable.path()));
        }
    }
    
}
