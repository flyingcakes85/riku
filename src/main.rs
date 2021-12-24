use std::env;

mod build;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "build" {
            build::build();
        }
    } else {
        println!("Pass build parameter");
    }
}
