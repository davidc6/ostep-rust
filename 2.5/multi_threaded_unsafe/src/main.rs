use std::{env, process::exit, thread};

#[derive(Clone)]
struct Wrapper(*mut i32);
unsafe impl Send for Wrapper {}

fn worker(num_of_loops: u32, counter: Wrapper) {
    let mut count = 0;
    loop {
        if count >= num_of_loops {
            return;
        }

        count += 1;

        unsafe {
            *counter.0 += 1;
        }
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

    let mut counter = 0;
    let pointer = Wrapper(&mut counter as *mut i32);

    unsafe {
        println!("Initial value: {}", *pointer.0);
    }
    
    let p1 = pointer.clone();
    let t1= thread::spawn(move || { 
        worker(num_loops, p1);
    });
    let p2 = pointer.clone();
    let t2= thread::spawn(move || {
        worker(num_loops, p2);
    });

    let _ = t1.join();
    let _ = t2.join();

    // let v = counter.clone();
    // let v = v.read().unwrap();
    unsafe {
        println!("Final value: {}", *pointer.0);
    }
}
