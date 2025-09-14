use std::process;
use std::ptr::{null, null_mut};
use std::ffi::{c_void, CString};
use libc::{__errno_location, close, execl, execle, execlp, execv, execvp, execvpe, exit, fork, open, sleep, wait, write};
use libc::{EXIT_FAILURE, O_CREAT, O_TRUNC, O_WRONLY, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR};

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

/// Q: You should try to ensure that the child process always prints first; can you do this
/// without calling wait() in the parent?
/// A: Yes. We can use sleep() system call causes the calling thread to sleep until the number of 
/// seconds specified is elapsed or a signal arrives.
fn exercise_three() {
    unsafe {
        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            println!("Child. Hello.");
        } else {
            // We can make the thread sleep but this will only work if the child thread takes
            // less time than the duration specified in this sleep function call.
            sleep(1);
            println!("Parent. Goodbye");
        }
    }
}

/// Q: Run the program "/bin/ls" using variants of exec().
fn exercise_four_1() {
    unsafe {
        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            // CString style
            let ls = CString::new("/usr/bin/ls").unwrap();
            let ls_ptr = ls.as_ptr();

            let ls_cmd = CString::new("ls").unwrap();
            let ls_cmd_ptr = ls_cmd.as_ptr();
           
            // Raw version
            // let ls_ptr: *const i8 = "/usr/bin/ls\0".as_ptr();
            //
            // let ls_cmd_ptr: *const i8 = ls_ptr.as_ptr();

            // Note: argument list has to be null terminated
            let return_value = execl(ls_ptr, ls_cmd_ptr as *const i8, null::<i8>());

            let errno = __errno_location();
            println!("Child process. {return_value} {}", *errno);
        } else {
            println!("Parent process");
        }
    }
}

fn exercise_four_2() {
    unsafe {
        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            // CString style
            let ls = CString::new("/usr/bin/env").unwrap();
            let ls_ptr = ls.as_ptr();

            let ls_cmd = CString::new("env").unwrap();
            let ls_cmd_ptr = ls_cmd.as_ptr();

            // Note: argument list has to be null terminated
            let env_var = CString::new("FOO=bar").unwrap();
            let env_var_ptr = env_var.as_ptr();

            let envs = [env_var_ptr, null::<i8>()].as_ptr();
            let return_value = execle(ls_ptr, ls_cmd_ptr, null::<i8>(), envs);

            let errno = __errno_location();
            println!("Child process. {return_value} {}", *errno);
        } else {
            println!("Parent process");
        }
    }
}

fn exercise_four_3() {
    unsafe {
        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            // CString style
            let ls = CString::new("ls").unwrap();
            let ls_ptr = ls.as_ptr();

            // Raw version
            // let ls_ptr: *const i8 = "ls\0".as_ptr();

            // Note: argument list has to be null terminated
            let return_value = execlp(ls_ptr, ls_ptr, null::<i8>());

            let errno = __errno_location();
            println!("Child process. {return_value} {}", *errno);
        } else {
            println!("Parent process");
        }
    }
}

fn exercise_four_4() {
    unsafe {
        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            // CString style
            let ls = CString::new("ls").unwrap();
            let ls_ptr = ls.as_ptr();

            let ls_path = CString::new("/usr/bin/ls").unwrap();
            let ls_path_ptr = ls_path.as_ptr();

            let args = [ls_ptr, null()].as_ptr();

            // Note: argument list has to be null terminated
            let return_value = execv(ls_path_ptr, args);

            let errno = __errno_location();
            println!("Child process. {return_value} {}", *errno);
        } else {
            println!("Parent process");
        }
    }
}

fn exercise_four_5() {
    unsafe {
        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            // CString style
            let ls = CString::new("ls").unwrap();
            let ls_ptr = ls.as_ptr();

            let ls_path = CString::new("/usr/bin/ls").unwrap();
            let ls_path_ptr = ls_path.as_ptr();

            let args = [ls_ptr, null()].as_ptr();

            // Note: argument list has to be null terminated
            let return_value = execvp(ls_ptr, args);

            let errno = __errno_location();
            println!("Child process. {return_value} {}", *errno);
        } else {
            println!("Parent process");
        }
    }
}

fn exercise_four_6() {
    unsafe {
        let return_value = fork();

        if return_value < 0 {
            println!("Fork failed");
            exit(EXIT_FAILURE);
        } else if return_value == 0 {
            // CString style
            let ls = CString::new("env").unwrap();
            let ls_ptr = ls.as_ptr();

            let ls_path = CString::new("/usr/bin/env").unwrap();
            let ls_path_ptr = ls_path.as_ptr();

            let args = [ls_ptr, null()].as_ptr();

            // Note: argument list has to be null terminated
            let env_var = CString::new("FOO=bar").unwrap();
            let env_var_ptr = [env_var.as_ptr(), null()].as_ptr();

            // Note: argument list has to be null terminated
            let return_value = execvpe(ls_ptr, args,env_var_ptr);

            let errno = __errno_location();
            println!("Child process. {return_value} {}", *errno);
        } else {
            println!("Parent process");
        }
    }
}


fn main() {
    // exercise_one();
    // exercise_two();
    // exercise_three();
    // exercise_four_1();
    // exercise_four_2();
    // exercise_four_3();
    // exercise_four_4();
    // exercise_four_5();
    exercise_four_6();
}
