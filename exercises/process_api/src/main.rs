use std::{process, ptr::{null_mut}};
use libc::{exit, fork, wait, EXIT_FAILURE};

/// Q: What value is the variable in the child process?
/// A: If set in parent to 100 then child is 100 since
/// fork, create a copy of the current process. If we set value is the child
/// process then it will be equal to that.
///
/// Q: What happens to the variable when both the child and parent change
/// the value of x?
/// A: They run in different memory spaces so changing one does not affect the other.
fn exercise_one() {
    let mut x = 100;

    unsafe {
        let return_code = fork();

        if return_code == 0 {
            println!("Child Running");
            // Child process
            x = 50;
            println!("{x} for child process {}", process::id());
        } else if return_code < 0 {
            println!("fork failed");
            exit(EXIT_FAILURE);
        } else {
            println!("Parent Running");
            println!("{x} for parent process {}", process::id());
            // Parent process
            x = 80;
            // wait() is used to delay parents execution until child is finished.
            // child will execute first, parent will execute second.
            println!("Parent Waiting");
            wait(null_mut());
            println!("Parent Running");
            println!("{x} for parent process {}", process::id());
        }
    }
}

fn main() {
    exercise_one();
}
