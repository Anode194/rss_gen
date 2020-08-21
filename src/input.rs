extern crate nom;
use nom::{
    bytes::complete::is_not,
    bytes::complete::tag,
    bytes::complete::take_until,
    //    branch::Alt,              needed for parsing multiple h tags for finding the heading. later
    sequence::delimited,
    IResult,
};
    //pub fn parse input_file() -> {
    //}
pub fn parse_title(input: &str) -> IResult<&str, &str> {
    delimited(tag("<h2>"), is_not("<"), tag("</h2>"))(input)
}
// TODO: need to implement a combinator to search for any h tag not just h2. priority: hi;
pub fn strip_till_title(input: &str) -> IResult<&str, &str> {
    take_until("<h2>")(input)
}
pub fn strip_prefix(input: &str) -> IResult<&str, &str> {
    take_until("<p>")(input)
}
pub fn strip_suffix(input: &str) -> IResult<&str, &str> {
    take_until("HTML;")(input)
}
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

