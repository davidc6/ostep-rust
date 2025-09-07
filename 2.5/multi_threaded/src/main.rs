use std::{env, process::exit, sync::{Arc, RwLock}, thread};

    fn worker(num_of_loops: u32, counter: Arc<RwLock<u32>>) {
    let mut count = 0;
    loop {
        if count >= num_of_loops {
            return;
        }

        count += 1;

        let mut c = counter.write().unwrap();
        *c += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: thread <value>");
        exit(1);
    }

    let num_loops = args.get(1).unwrap().parse::<u32>().unwrap();
    println!("{num_loops}");

    let counter = Arc::new(RwLock::new(0));

    let v = counter.clone();
    let v = v.read().unwrap();
    println!("Initial value: {}", *v);
    drop(v);
   
    let c1 = counter.clone();
    let t1= thread::spawn(move || worker(num_loops, c1));
    let c2 = counter.clone();
    let t2= thread::spawn(move || worker(num_loops, c2));

    let _ = t1.join();
    let _ = t2.join();

    let v = counter.clone();
    let v = v.read().unwrap();
    println!("Final value: {}", *v);
}
