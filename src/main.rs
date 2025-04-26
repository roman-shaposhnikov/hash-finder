mod input;
mod hasher;

use std::{ env, process };
use std::sync::mpsc;

use input::Config;

fn main() {
    let mut args = env::args();
    args.next();

    let config = Config::build(args).unwrap_or_else(|mes| {
        println!("{}", mes);
        process::exit(1);
    });

    let threads_count = 10;
    let mut hashes_count = 0;
    let (tx, rx) = mpsc::channel();

    hasher::calculate_hashes(tx, config.n, threads_count);

    for pair in rx {
        if hashes_count < config.f {
            hashes_count += 1;
            println!("{}, \"{}\"", pair.0, pair.1);
        } else {
            break;
        }
    }
}
