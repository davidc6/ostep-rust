# Notes

- **trap** - a hardware control instruction to raise privilege to **kernel mode**. 
Control is handed over to **trap handler**. Once done, OS passed control over to 
the user via **return-from-trap** instruction. 
- **scheduling policy** - is a policy within the OS to decide ordering in which 
processes should run.

## The Process

- **registers** - some these form part of the process's machine state. PC (program 
counter) tells us which instruction the program will execute next. **Stack pointer** 
and associated **frame pointer**, these are used to manage function parameters, 
local variables and return addresses.

- Process API allows to make processes-related calls such as:
    - Create - to create a process
    - Destroy - to exit a process
    - Wait - wait for a process to stop running (before next steps)
    - Misc Control - e.g. suspend/resume a process
    - Status - a process status

- Process Lifecycle
    - Load code and static data (uninitialised vars) into the address space of 
    a process. In the past OSes would load everything at once before running the 
    program. Modern OSes do it lazily (using the machinery of **paging** and 
    **swapping**).
    - The OS will allocate memory for **(run-time) stack**.
    - The OS will allocate some memory for the **heap** (malloc() to allocate, 
    free() to deallocate).
    - IO (in Unix each process gets 3 open file descriptors stdin, stdout, stderr)
    - Jump to **main()** routine
    - OS transfers CPU control to the newly-created process and execution begins

- Process States
    - Running - running on a processor
    - Ready - ready to run but not running on a processor
    - Blocked - some operation was performed by the process and it is not yet
    ready to run (e.g. disk I/O operation)
    - OS scheduler makes process switching decisions

- Register Context - holds contents of registers for a stopped process. Once the 
registers are restored, the processes resumes. The switching back-and-forth process 
is called **context switching**. 

- Process list - contains info about processes in the system. Each entry is stored 
in a **Process Control Block** (aka process descriptor) a data structure such info as 
process id, state, registers, schedule-related information etc.
