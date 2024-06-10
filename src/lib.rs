use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename: String = args[1].clone();
        let query: String = args[2].clone();

        Config { filename, query }
    }
}

pub fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("Error reading file");
    println!("{}", &content[..100]);
    let found: Vec<&str> = search(&config.query, &content);
    for line in found {
        println!("{}", line);
    }
}

// lifetime = tiempo de vida

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query){
            result.push(line);
        }
    }

    result
}