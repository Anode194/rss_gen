use crate::config::ConfigData;
use crate::input::enclose;
use std::io::BufReader;
use crate::config::Post;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub fn write_to_out_file(post: Post, output_file: &str, cd: ConfigData) {
        let post_link = format!("{}/{}",cd.link,cd.blog_dir);
        let mut out_str = format!(
       "<?xml version='1.0' encoding='UTF-8' ?>\n<rss version='2.0'>\n\t<channel>
       \t\t{}    
       \t\t{}    
       \t\t{}    
       \t\t<item>
       \t\t\t{}
       \t\t\t{}
       \t\t\t{}
       \t\t\t{}
       \t\t\t{}
       \t\t</item>
       \t</channel>
       </rss>",
       enclose(&cd.title,"title"),
       enclose(&cd.link,"link"),
       enclose(&cd.description,"description"),
       enclose(&post_link,"link"),
       enclose(&post.title,"title"),
       enclose(&cd.language,"language"),
       enclose(&post.description,"description"),
       enclose(&post.category,"category"),
        );
    if output_file == "default" {
        let mut file = match File::create("blog.xml") {
            Ok(x) =>x,
            Err(e) => panic!("couldn't open Default output file {:?}",e),
        };
        match file.write_all(out_str.as_bytes()) {
            Ok(x) =>x,
            Err(e) => panic!("couldn't write to output file. \n {}",e),
        };

    } else {
        let file = open_output_file(output_file);
        let file_contents = BufReader::new(file);
    }
}

fn open_output_file(output: &str) -> File {
    let config_file = match OpenOptions::new().read(true).write(true).open(output) {
        Ok(x) => x,
        Err(e) => panic!("couldn't open output file was it misspelled? {:?}",e),
    };
    config_file
}
