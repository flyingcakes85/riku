use std::{env, path::PathBuf};

mod build;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        if args[1] == "build" {
            build::build(PathBuf::from(args[2].clone()));
        }
    } else {
        println!("Pass build parameter");
    }
}
