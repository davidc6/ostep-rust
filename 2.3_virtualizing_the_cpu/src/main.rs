use std::{alloc::{alloc, Layout}, process, thread::sleep, time::Duration};

fn spin(secs: u64) {
    sleep(Duration::from_secs(secs));
}

fn unsafe_version() {
    unsafe {
        let memory_layout = Layout::new::<u32>();
        let pointer = alloc(memory_layout);

        println!("({}) address pointed to by p: {:p}", process::id(), pointer);

        *pointer = 0;

        loop {
            spin(1);

            *pointer = *pointer + 1;
            println!("({}) p: {}", process::id(), *pointer);
        }
    }
}

fn safe_version() {
    let mut value = Box::new(1);

    println!("({}) address pointed to by p: {:p}", process::id(), value);

    *value = 0;

    loop {
        spin(1);

        *value += 1;
        println!("({}) p: {}", process::id(), *value);
    }
}

/// Address-space randomisation should be disabled in order to 
/// see the same address when running two processes concurrently.
/// It is a technique used as a defence mechanism against certain 
/// security flaws.
fn main() {
    // println!("Unsafe version");
    // unsafe_version();
    // println!("======");
    // println!("Safe version");

    safe_version();
}
