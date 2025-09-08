use std::process;
use libc::fork;

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
            println!("Parent of {pid} (pid: {})", process::id());
        }
    }
}
