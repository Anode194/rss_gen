use crate::config::ConfigData;
use crate::input::enclose;
use crate::input::enclose_nl;
use std::io::BufReader;
use crate::config::Post;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

    //cd.link == link to blog 
    //cd.blog_dir == blogposts are found 
    //post.link == name of blogpost file
    
pub fn write_to_out_file(posts: Vec<Post>, output_file: &str, cd: ConfigData) {
        let out_str = format!(
       "<?xml version='1.0' encoding='UTF-8' ?>\n<rss version='2.0'>\n\t<channel>
       \t\t{}    
       \t\t{}    
       \t\t{}    
       {}
       \t</channel>
       </rss>",
       enclose(&cd.title,"title"),
       enclose(&cd.link,"link"),
       enclose(&cd.description,"description"),
       format_posts(posts, &cd.link),
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
fn format_posts(posts: Vec<Post>, link: &str) -> String {
    let mut out_str = String::new();
    for post in posts {
        let item = format! ("
       \t\t<item>
       \t\t\t{}
       \t\t\t{}
       \t\t\t{}
       \t\t\t{}
       \t\t\t{}
       \t\t</item>\n
        ",enclose(&post.title,"title"),
        enclose(&format!("{}/{}",link, &post.link),"link"),
        enclose(&post.language,"language"),
        enclose(&post.description,"description"),
        enclose(&post.category,"category")
        );
        out_str.push_str(&item);
    }

    out_str
}
