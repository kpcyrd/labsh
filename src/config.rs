extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

use build::Task;

#[derive(Debug)]
pub struct Config {
    pub hostname: String,
    builds: BTreeMap<String, Build>,
}

impl Config {
    fn from_toml(file: &toml::Value) -> Config {
        let meta = file.get("meta").unwrap();

        let hostname = match meta.get("hostname") {
            Some(hostname) => {
                hostname.as_str().unwrap()
            },
            None => {
                "localhost"
            },
        };

        Config {
            hostname: String::from(hostname),
            builds: BTreeMap::new(),
        }
    }

    fn add_tasks_from_toml(&mut self, file: toml::Value) {
        let tasks = file.get("build").unwrap().as_table().unwrap();
        for (name, options) in tasks.into_iter() {
            let build = Build::from_toml(options.to_owned());
            self.builds.insert(name.to_owned(), build);
        }
    }

    pub fn get_build(&self, key: &str) -> Option<&Build> {
        self.builds.get(key)
    }
}

#[derive(Debug)]
pub struct Build {
    pub tasks: Vec<Task>,
}

impl Build {
    fn from_toml(config: toml::Value) -> Build {
        let mut tasks = Vec::new();

        let path = config.get("path").unwrap().as_str().unwrap();
        let pull = match config.get("pull") {
            Some(pull) => {
                pull.as_bool().unwrap_or(false)
            },
            None => {
                false
            }
        };

        if pull {
            tasks.push(Task::new("git", vec!["-C", path, "pull"]));
        }

        match config.get("image") {
            Some(image) => {
                let image = image.as_str().unwrap();
                tasks.push(Task::new("docker", vec!["build", "--pull", "-t", image, "--", path]));
                tasks.push(Task::new("docker", vec!["push", "--", image]));
            },
            None => {},
        };

        Build {
            tasks: tasks
        }
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

fn read_config(path: &str) -> toml::Value {
    let mut file = File::open(path)
        .expect("failed to open config");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read config");

    toml::from_str(&contents)
        .expect("failed to parse config")
}

pub fn load(path: &str) -> Config {
    let file = read_config(path);

    let mut config = Config::from_toml(&file);
    config.add_tasks_from_toml(file);
    config
}
