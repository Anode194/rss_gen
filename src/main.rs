//use std::io::BufRead;
//use std::io::BufReader;
//use std::path::PathBuf;
//
use clap:: App;
use std::fs;

extern crate nom;
use nom::{
    sequence::delimited,
    IResult,
    bytes::complete::tag,
    bytes::complete::is_not,

};

fn main() {
    let matches = App::new("rss_gen")
    .version("0.0.1")
    .author("Jo Phillips <joxphillips@gmail.com>")
    .about("Reads in a html file and creates a rss entry for a blog.")
    .arg("-i --input=[file] 'blog post html file'")
    .arg("-o --output=[file] 'sets the file to put blog post xml data if one is not supplied a file will be created")
    .get_matches();

    let mut file_name = String::new();
    //println!("{:?}",matches.value_of("input"));
    match matches.value_of("input") {
        Some(x) => file_name = x.to_string(),
        None => println!("Please enter a file to parse"),
    }
    let mut contents = fs::read_to_string(file_name)
        .expect("Something went wrong opening the file.");
    let title = match parse_title(contents.as_mut_str()) {
        Ok(i) => i,
            
        Err(e) => 
            panic!("parse blogpost title failed {:?}",e),
    };
    let _body = title.0;
    let mut header = title.1.to_string();
    let header = enclose(header.as_mut_str(),"title");
    
    println!("{:?}", header );
}

fn parse_title(input: &str) -> IResult<&str, &str> {
    delimited(tag("<h1>"), is_not("<"), tag("</h"))(input)
}

fn enclose(content: &str, tag_type: &str) -> String {
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
#[cfg(test)]
mod tests {
use super::*;
#[test]
fn title_string() {
    let test_title = enclose("title","Description");
    assert_eq!(test_title, "<Description>title</Description>".to_string());
    }
#[test]
fn body_string() {
    let body = enclose("some blog", "title");

    assert_eq!(body, "<title>some blog</title>");
    }
#[test]
fn title_parse() {
    let input_str = "<h1>header</h1>";
    let parse_title = parse_title(input_str);
    assert_eq!(parse_title, Ok(("1>","header")));
}
}
