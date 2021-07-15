extern crate nom;
use crate::config::Post;
use crate::config::ConfigData;
use crate::config::read_config;
use std::vec::Vec;
use nom::{
    bytes::complete::is_not,
    bytes::complete::tag,
    bytes::complete::take_until,
    //    branch::Alt,              needed for parsing multiple h tags for finding the heading. later
    sequence::delimited,
    IResult,
};
use std::fs;
use std::io;
use std::path::Path;
//pub fn parse input_file() -> {
//}
pub fn parse_title(input: &str) -> IResult<&str, &str> {
    delimited(tag("<h2>"), is_not("<"), tag("</h2>"))(input)
}
// TODO: need to implement a combinator to search for any h tag not just h2. priority: hi;
pub fn strip_till_title(input: &str) -> IResult<&str, &str> {
    take_until("<h2>")(input)
}
#[allow(dead_code)]
pub fn strip_prefix(input: &str) -> IResult<&str, &str> {
    take_until("<p>")(input)
}
pub fn strip_suffix(input: &str) -> IResult<&str, &str> {
    take_until("HTML;")(input)
}
#[allow(dead_code)]
pub fn parse_body(input: &str) -> IResult<&str, &str> {
    delimited(tag("<<<HTML"), is_not("1>"), tag("</h"))(input)
}

pub fn enclose(content: &str, tag_type: &str) -> String {
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
//for description being on their own lines as well as comments etc.
pub fn enclose_nl(content: &str, tag_type: &str) -> String {
    let mut tag = String::from("<");
    let end_tag_start = "\n</";
    let tag_end = ">";
    let tag_end_nl = ">\n";
    tag.push_str(tag_type);
    tag.push_str(tag_end_nl);
    tag.push_str(content);
    tag.push_str(end_tag_start);
    tag.push_str(tag_type);
    tag.push_str(tag_end);

    tag
}
pub fn post_dir(dir: &Path) -> Vec<Post> { //TODO: create  test
    let mut post_vec = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir) {
            for files in entry {
                let file = files.unwrap();
                let path = file.path();
                if !path.is_dir() {
                let mut contents =
                    fs::read_to_string(&path).expect("Something went wrong opening the file.");
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
                let mut conf = read_config();
                let link = path.file_name().unwrap();
                let post = Post::new(title, conf.language.as_mut_str(), body, "", link.to_str().unwrap());
                post_vec.push(post);
                } else {
                    continue;
                }
            }
        }
    } else {
        panic!("file passed in is not a directory, please provide a directory for the -d flag");
    }
    post_vec
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
