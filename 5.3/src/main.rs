use std::process;
use libc::{__errno_location, execvp, fork, wait};
use std::ptr;

fn debug_arguments(arguments: [*const u8; 3]) {
    unsafe {
        let k = std::slice::from_raw_parts(arguments[1], 8);
        let k_un = std::str::from_utf8_unchecked(k);
        println!("PRINT {:?}", k_un);
    }
}

fn main() {
    println!("Hello (pid:{})", process::id());

    unsafe {
        let pid = fork();

        if pid < 0 {
            println!("Fork failed");
            process::exit(1);
        } else if pid == 0 {
            println!("Child (pid:{})", process::id());
            
            let mut arguments: [*const u8; 3] = [0 as *const u8; 3];

            arguments[0] = "wc\0".as_ptr();
            arguments[1] = "text.txt\0".as_ptr();
            arguments[2] = ptr::null();

            let result = execvp(arguments[0] as *const i8, arguments.as_ptr() as *const *const i8);
            if result == -1 {
                let err_location = __errno_location();
                println!("ERROR: {:?}", *err_location);
                return;
            }

            println!("Do not print this");

        } else {
            // Delay execution until child finishes executing.
            let child_pid = wait(ptr::null_mut());
            println!("Parent of {pid} (wait:{child_pid}) (pid: {})", process::id());
        }
    }
}

