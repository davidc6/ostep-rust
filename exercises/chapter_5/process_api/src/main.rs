use std::{ffi::c_void, process, ptr::null_mut};
use libc::{__errno_location, close, exit, fork, open, wait, write, EXIT_FAILURE, O_APPEND, O_CREAT, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY, S_IRGRP, S_IROTH, S_IRUSR, S_IRWXU, S_IWUSR};

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

/// Q: Can both child and parent access the file descriptor returned by open()?
/// A: Yes, both can since we make a copy of the parent processor by calling fork().
///
/// Q: What happens when they are writing to the file concurrently?
/// A: Both have a different file descriptor but share the underlying open file table entry.
/// Multiple processes can write to the same file but unless a coordination between
/// processes or locking happen, there might be data corruption. Unless O_APPEND is applied,
/// it can reduce the chance of interleaving. Proper locking must be observed.
fn exercise_two() {
    unsafe {
        //let filepath = "text.txt";
        let open_fd = open("text2.txt\0".as_ptr() as *const i8, O_CREAT | O_WRONLY | O_TRUNC, S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);

        if open_fd < 0 {
            let e = __errno_location();
            println!("{}", *e);
            exit(EXIT_FAILURE);
        }

        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            println!("Child");
            println!("{}", open_fd);
            write(open_fd, "Child\0".as_ptr() as *const c_void, 6);
        } else {
            println!("Parent");
            wait(null_mut());
            println!("{}", open_fd);
            let written = write(open_fd, "Parent\0".as_ptr() as *const c_void, 7);
            println!("Written {}", written);
            close(open_fd);
        }
    }
}

fn main() {
    // exercise_one();
    exercise_two();
}
