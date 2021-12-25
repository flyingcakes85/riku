use std::{env, fs, path::PathBuf};
use toml::Value as TomlValue;

mod build;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let config_location = args[1].to_string();
        let config = fs::read_to_string(config_location)
            .unwrap()
            .parse::<TomlValue>()
            .unwrap();
        build::build(
            PathBuf::from(config["template_path"].as_str().unwrap().to_string()),
            config["img_width"]
                .as_integer()
                .unwrap()
                .try_into()
                .unwrap(),
            config["img_base_url"].as_str().unwrap().to_string(),
            config["img_small_base_url"].as_str().unwrap().to_string(),
            config["output_folder"].as_str().unwrap().to_string(),
        );
    } else {
        println!("Pass build parameter");
    }
}
