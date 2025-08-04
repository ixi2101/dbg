use std::{env::Args, ffi::{CStr, CString}, os::unix::process::CommandExt, process, str::FromStr, thread::sleep, time::Duration};
use nix::{errno, sys::{ptrace, wait::{self, waitpid, WaitPidFlag}}, unistd::{execvp, execvpe, fork, ForkResult, Pid}};
use libc::{pid_t, PTRACE_ATTACH, PTRACE_O_TRACEEXEC, WCONTINUED};


/// Attach to a PID and return the PID
pub fn attach(pid: i32) -> Result<Pid, errno::Errno>{
    let pid_as_pid: Pid = Pid::from_raw(pid);
    match ptrace::attach(pid_as_pid) {
        Ok(_) => Ok(pid_as_pid),
        Err(e) => Err(e),
    }
}

/// Run and attach to a process
pub fn exec_attach(argv: Vec<&CStr>) -> Result<Pid, errno::Errno>{
    // Assume argv[0] is the desired program name
    match unsafe {fork()}{
        Ok(ForkResult::Parent { child, ..}) => {
       
            waitpid(child, None).expect("failed to wait");
            println!("WAITING");
            sleep(Duration::from_secs(5));
            println!("DONE WAITING");
           
           Ok(child)
        },
        Ok(ForkResult::Child) => {
            // We are the child. Become the debuggee
            println!("Exec child pid: {}", process::id());
            ptrace::traceme().expect("PTRACEME failed !");
            execvp(argv[0], &argv[1..]).unwrap();
            unreachable!("EXECVP FAIL");
        },
        Err(e) => {return Err(e);}
    }
}
