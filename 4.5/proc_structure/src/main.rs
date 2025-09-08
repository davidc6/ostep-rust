use std::{os::raw::c_char};

struct Context {
    eip: isize,
    esp: isize,
    ebx: isize
}

enum ProcessState {
    Unused,
    Embryo,
    Sleeping,
    Runnable,
    Running,
    Zombie
}

struct Process<'a> {
    mem: *mut c_char,
    size: usize,
    kernel_stack_bottom: *mut c_char,
    process_state: ProcessState,
    pid: isize,
    parent: &'a Process<'a>,
    char: bool,
    killed: isize,
    open_files: Vec<isize>,
    inode: bool,
    context: Context,
    trap_frame: bool,
}


fn main() {
    println!("Hello, world!");
}
