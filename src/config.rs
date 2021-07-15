use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::DirBuilder;
use std::fs::OpenOptions;
use std::io::prelude::*;
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    pub title: String,
    pub link: String,        // <-- link to your blog
    pub description: String, // <-- description of your blog
    pub blog_dir: String,    // <-- place where blog posts are kepts
    pub language: String,    // <-- language of the website
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
        } else {
            println!(
                "It appears that the file already exists.\n
You can find the file on linux in your $HOME/.config \n
on macos in the $HOME/Library/preferences folder\n
and on windows in your your user\\name\\AppData\\Roaming folder"
            );
        }
    }
}
#[allow(dead_code)]
pub fn read_config() -> ConfigData {
    let mut config = match config_dir() {
        Some(x) => x,
        None => panic!(
            "coudn't access your configuration directory on linux this is home/user/.config\n"
        ),
    };
    config.push("rss_gen/config.json");
    let config_file_op = OpenOptions::new().read(true).open(config);
    let mut config_file = match config_file_op {
        Ok(i) => i,
        Err(e) => panic!(
            "couln't open your config file. on linux this is located at home/user/.config\n{:?}",
            e
        ),
    };
    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(x) => x,
        Err(e) => panic!("couldn't parse config file, check to make sure the syntax is correct and run again.\n{:?}", e)
    };

    let cd:ConfigData = match serde_json::from_str(config_string.as_mut_str()) {
        Ok(i) => i,
        Err(e) => panic!("couldn't parse config file, check to make sure the syntax is correct and run again.\n{:?}", e)
    };

    cd
}

#[allow(dead_code)]
pub struct Post {
    pub title: String,
    pub language: String,
    pub description: String, //<-- body of the rss post.
    pub category: String,    //what kind of post is it about eg coding art etc.
    pub link: String,        //file name of the post
}
#[allow(dead_code)]
impl Post {
    pub fn new(
        title: &str,
        language: &str,
        description: &str,
        category: &str,
        file_name: &str,
    ) -> Post {
        Post {
            title: title.to_string(),
            language: language.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            link: file_name.to_string(),
        }
    }
    pub fn default() -> Post {
        Post {
            title: String::new(),
            language: String::new(),
            description: String::new(),
            category: String::new(),
            link: String::new(),
        }
    }
}
