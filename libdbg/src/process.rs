use std::{ffi::CStr, os::unix::process::CommandExt, process::{self, Command}, thread::sleep, time::Duration};
use nix::{errno, sys::{ptrace, wait::waitpid}, unistd::{execvp, fork, ForkResult, Pid}};
use anyhow::{Context, Result};


/// Attach to a PID and return the PID
pub fn attach(pid: i32) -> Result<Pid, errno::Errno>{
    let pid_as_pid: Pid = Pid::from_raw(pid);
    match ptrace::attach(pid_as_pid) {
        Ok(_) => Ok(pid_as_pid),
        Err(e) => Err(e),
    }
}

/// Run and attach to a process
pub fn exec_attach(progname: String) -> Result<Pid, errno::Errno>{
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
            let _ = Command::new(progname).exec();
            unreachable!("EXECVP FAIL");
        },
        Err(e) => {return Err(e);}
    }
}

enum ProcessState{
    UNKNOWN,
}

pub struct Process {
    pub pid: Pid,
    state: ProcessState
}
impl Process {
    pub fn launch_process(program_name: String) -> Result<Process> {
        let pid_ = exec_attach(program_name)?;
        Ok(Process{pid: pid_, state: ProcessState::UNKNOWN})
    }

    pub fn attach_process(pid: i32) -> Result<Process> {
        let pid_ = attach(pid)?;
        Ok(Process{pid: pid_, state: ProcessState::UNKNOWN})
    }
}