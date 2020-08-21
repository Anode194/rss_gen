mod config;
mod input;
use clap::App;
use config::ConfigData;
use std::fs;
use input::*;
extern crate nom;

fn main() {
    let matches = App::new("rss_gen")
    .version("0.0.1")
    .author("Jo Phillips <joxphillips@gmail.com>")
    .about("Reads in a html file and creates a rss entry for a blog.")
    .arg("-i --input=[file] 'blog post html file'")
    .arg("-o --output=[file] 'sets the file to put blog post xml data if one is not supplied a file will be created")
    .arg("-c --config=[file] 'sets a config file to use for the url and other information that can't be gleemed from the input file.'")
    .arg("-n --new_conf 'prints default config in your systems config directory. default values will not work must change before program can be run.'")
    .get_matches();

    let mut file_name = String::new();
    //println!("{:?}",matches.value_of("input"));
    match matches.value_of("input") {
        Some(x) => file_name = x.to_string(),
        None => println!("Please enter a file to parse"),
    }
    let mut contents =
        fs::read_to_string(file_name).expect("Something went wrong opening the file.");
    let header_garbage = match strip_till_title(contents.as_mut_str()) {
        // should return tuple of .0=header_garbagef.1 rest of the contents of the file.
        Ok(i) => i,

        Err(e) => panic!("failed to parse header garbage \n{:?}", e),
    };

    let header = match parse_title(header_garbage.0) {
        Ok(i) => i,
        Err(e) => panic!("parse blogpost title failed \n{:?}", e),
    };
    let mut _title = header.1; // saved for when we need to put it into the file at the end.
    let mut body = match strip_suffix(header.0) {
        Ok(i) => i.1,
        Err(e) => panic!("failed to strip prefix from body. \n{:?}", e),
    };
    let body = &body.replace("\n", " ").to_string();
    println!("\n\n\n");
    println!("{:?}", body);
    let conf = config::ConfigData::new(
        "dreamrot",
        "https://dreamrot.com/blog",
        "a blog about coding art and life",
        "/posts",
        "en-us",
    );
}

