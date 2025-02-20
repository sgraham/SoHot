use sohot;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        sohot::init(&args[2..], true).unwrap();
    } else if args[1] == "patch" {
        sohot::generate_patch_and_update(args[2].parse::<usize>().unwrap(), &args[3], &args[4])
            .unwrap();
    } else {
        println!("Unrecognized command {}", args[1]);
    }
}
