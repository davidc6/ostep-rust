use std::{process};
use libc::{fork, wait};

fn main() {
    println!("Hello (pid:{})", process::id());

    unsafe {
        let pid = fork();

        if pid < 0 {
            println!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            println!("Child (pid:{})", process::id());
        } else {
            // Delay execution until child finishes executing.
            let child_pid = wait(core::ptr::null_mut());
            println!("Parent of {pid} (wait:{child_pid}) (pid: {})", process::id());
        }
    }
}
