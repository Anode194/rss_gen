mod config;
mod input;
use clap::App;
use config::ConfigData;
use std::fs;
extern crate nom;
use nom::{
    bytes::complete::is_not,
    bytes::complete::tag,
    bytes::complete::take_until,
    //    branch::Alt,              needed for parsing multiple h tags for finding the heading. later
    sequence::delimited,
    IResult,
};

fn main() {
    let matches = App::new("rss_gen")
    .version("0.0.1")
    .author("Jo Phillips <joxphillips@gmail.com>")
    .about("Reads in a html file and creates a rss entry for a blog.")
    .arg("-i --input=[file] 'blog post html file'")
    .arg("-o --output=[file] 'sets the file to put blog post xml data if one is not supplied a file will be created")
    .arg("-c --config=[file] 'sets a config file to use for the url and other information that can't be gleemed from the input file.'")
    .arg("-n --new_conf'sets a config file to use for the url and other information that can't be gleemed from the input file.'")
    .arg("-u --url 'sets the url should be used in conjunction with -n'")
    .arg("-t --title 'sets the rss feed title should be used in conjunction with -n'")
    .arg("-d --description 'sets the description of the rss feed'")
    .arg("-b --blogdir 'sets the blog directory of your website. ie where your posts are found ie (foobar.com)/blogs <- blogs'")
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

    let mut header = match parse_title(header_garbage.0) {
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

fn parse_title(input: &str) -> IResult<&str, &str> {
    delimited(tag("<h2>"), is_not("<"), tag("</h2>"))(input)
}
// TODO: need to implement a combinator to search for any h tag not just h2. priority: hi;
fn strip_till_title(input: &str) -> IResult<&str, &str> {
    take_until("<h2>")(input)
}
fn strip_prefix(input: &str) -> IResult<&str, &str> {
    take_until("<p>")(input)
}
fn strip_suffix(input: &str) -> IResult<&str, &str> {
    take_until("HTML;")(input)
}
fn parse_body(input: &str) -> IResult<&str, &str> {
    delimited(tag("<<<HTML"), is_not("1>"), tag("</h"))(input)
}

fn enclose(content: &str, tag_type: &str) -> String {
    let mut tag = String::from("<");
    let end_tag_start = "</";
    let tag_end = ">";
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
        let test_title = enclose("title", "Description");
        assert_eq!(test_title, "<Description>title</Description>".to_string());
    }
    #[test]
    fn body_string() {
        let body = enclose("some blog", "title");

        assert_eq!(body, "<title>some blog</title>");
    }
    #[test]
    fn title_parse() {
        let input_str = "<h2>header</h2>";
        let parse_title = parse_title(input_str);
        assert_eq!(parse_title, Ok(("", "header")));
    }
    #[test]
    fn strip_pre() {
        let input_str = "asdfasdf<p>f";
        let parse_pre = strip_prefix(input_str);
        assert_eq!(parse_pre, Ok(("<p>f", "asdfasdf")));
    }
    #[test]
    fn strip_suff() {
        let input_str = "asdfasdfHTML;f";
        let stripped = strip_suffix(input_str);
        assert_eq!(stripped, Ok(("HTML;f", "asdfasdf")));
    }
}
