use std::fs::File;
//use std::io::BufRead;
//use std::io::BufReader;
use std::path::PathBuf;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rss_gen")
    .version("0.0.1")
    .author("Jo Phillips <joxphillips@gmail.com>")
    .about("Reads in a html file and creates a rss entry for a blog.")
    .arg("-i --input=[file] 'blog post html file'")
    .arg("-o --output=[file] 'sets the file to put blog post xml data if one is not supplied a file will be created")
    .get_matches();
    let file_name = String::new();

    match matches.value_of('i') {
        Some(x) => file_name = x.to_string(),
        None => println!("Please enter a file to parse"),
    }
    let f = File::open(file_name); 
}


fn enclose_tags( content: &str, tag_type: &str) -> String {
    let mut tag= String::from("<");
    let end_tag_start="</";
    let tag_end=">";
    tag.push_str(tag_type);
    tag.push_str(tag_end);
    tag.push_str(content);
    tag.push_str(end_tag_start);
    tag.push_str(tag_type);
    tag.push_str(tag_end);
    tag
}

#[test]
fn title_string() {
    let test_title = enclose_tags("title","Description");
    assert_eq!(test_title, "<Description>title</Description>".to_string());
}
#[test]
fn body_string() {
    let body = enclose_tags("some blog", "title");

    assert_eq!(body, "<title>some blog</title>");
}
