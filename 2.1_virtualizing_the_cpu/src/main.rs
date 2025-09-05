use std::env;
use std::thread::sleep;
use std::process::exit;
use std::time::Duration;

fn spin(secs: u64) {
    sleep(Duration::from_secs(secs));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: CPU <string>");
        exit(1);
    }

    let val = args.get(1).unwrap();

    loop {
        spin(1);
        print!("{val}\n");
    }
}

