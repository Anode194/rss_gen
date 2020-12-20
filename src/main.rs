mod config;
mod input;
mod output;
use clap::App;
use config::ConfigData;
use input::*;
use std::fs;
use std::path::Path;
extern crate nom;

fn main() {
    let matches = App::new("rss_gen")
    .version("0.0.1")
    .author("Jo Phillips <joxphillips@gmail.com>")
    .about("Reads in a html file and creates a rss entry for a blog.")
    .arg("-i --input=[file] 'blog post html file'")
    .arg("-o --output=[file] 'sets the file to put blog post xml data if one is not supplied a file will be created")
    .arg("-c --config=[file] 'sets a config file to use for the url and other information that can't be gleemed from the input file.'")
    .arg("-d --description 'gives the description of the blogpost that is being read in by -i'")
    .arg("-n --new_conf 'prints default config in your systems config directory. default values will not work must change before program can be run.'")
    .get_matches();

    let mut input_file = String::new();
    let mut output_file = String::from("default");
    //println!("{:?}",matches.value_of("input"));
    match matches.value_of("input") {
        Some(x) => input_file = x.to_string(),
        None => {
            println!("make sure you include an html or php file with the --i or --input option")
        }
    }
    match matches.value_of("output") {
        Some(x) => output_file = x.to_string(),
        None => {
            println!("couldn't find output file creating new file in current directory...");
        }
    }
    let path = Path::new(&input_file);
    if path.is_dir() {
        let mut conf = config::read_config();
        let posts = post_dir(&path);
        output::write_to_out_file(posts, output_file.as_mut_str(), conf);

    } else {
        let mut contents =
        fs::read_to_string(&input_file).expect("Something went wrong opening the file.");
        let header_garbage = match strip_till_title(contents.as_mut_str()) {
            // should return tuple of .0=header_garbagef.1 rest of the contents of the file.
            Ok(i) => i,

            Err(e) => panic!("failed to parse header garbage \n{:?}", e),
        };

        let header = match parse_title(header_garbage.0) {
            Ok(i) => i,
            Err(e) => panic!("parse blogpost title failed \n{:?}", e),
        };
        let title = header.1; // saved for when we need to put it into the file at the end.
        let body = match strip_suffix(header.0) {
            Ok(i) => i.1,
            Err(e) => panic!("failed to strip prefix from body. \n{:?}", e),
        };

        let body = &body.replace("\n", " ").to_string();
        let mut conf = config::read_config();

        //println!("\n\n\n");
        //println!("{:?}", body);
        
        let post = config::Post::new(title, conf.language.as_mut_str(), body, "", &input_file);
        let posts = vec!(post);
        output::write_to_out_file(posts, output_file.as_mut_str(), conf);
    }
}
