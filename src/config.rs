use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::DirBuilder;
use std::fs::OpenOptions;
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    pub title: String,
    pub link: String,        // <-- link to your blog
    pub description: String, // <-- description of your blog
    pub blog_dir: String,    // <-- place where blog posts are kepts
    pub language: String,    // <-- place where blog posts are kepts
}
#[allow(dead_code)]
impl ConfigData {
    pub fn new(
        title: &str,
        link: &str,
        description: &str,
        blog_dir: &str,
        lang: &str,
    ) -> ConfigData {
        ConfigData {
            title: title.to_string(),
            link: link.to_string(),
            description: description.to_string(),
            blog_dir: blog_dir.to_string(),
            language: lang.to_string(),
        }
    }
    // write new json config file to .config/rss_gen/config.json an error will be thrown if the
    // file exists should only be run on first startup. 
    pub fn write_new_config(self) {         
        let mut config_path = match config_dir() {
            Some(x) => x,
            None => panic!(
                "coudn't access your configuration directory on linux this is home/user/.config\n"
            ),
        };
        config_path.push("rss_gen");
        if !config_path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(&config_path)
                .unwrap();
        }
        println!("{:?}\n", config_path.to_str());
        config_path.push("config.json");
        let config_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(config_path)
            .unwrap();

        let _j = match serde_json::to_writer_pretty(config_file, &self) {
            Ok(i) => i,
            Err(e) => panic!("{:?}", e),
        };
    }
}
#[allow(dead_code)]
pub struct Post {
    pub title: String,
    pub language: String,
    pub description: String, //<-- body of the rss post.
    pub category: String,    //what kind of post is it about eg coding art etc.
}
#[allow(dead_code)]
impl Post {
    pub fn new(title: &str, language: &str, description: &str, category: &str) -> Post {
        Post {
            title: title.to_string(),
            language: language.to_string(),
            description: description.to_string(),
            category: category.to_string(),
        }
    }
    pub fn default() -> Post {
        Post {
            title: String::new(),
            language: String::new(),
            description: String::new(),
            category: String::new(),
        }
    }
}
