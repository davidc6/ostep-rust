use std::process::{self, Command};

fn main() {
    let pid = process::id();
    println!("Hello (pid:{})", pid);

    let child = Command::new("echo")
        .arg("")
        .spawn();

    match child {
        Ok(child_process) => {
            let pid = child_process.id();
            
            println!("Child (pid:{})", pid);
            return;
        }
        Err(e) => {
            println!("Spawn failed {}", e);
            return;
        }
    }
}

