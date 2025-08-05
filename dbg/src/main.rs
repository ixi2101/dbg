use std::ffi::CStr;
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: OpMode,
}

#[derive(Subcommand, Debug)]
enum OpMode {
    Attach {pid: i32},
    Launch {program: String},
}

use libdbg::process::{self, Process};
fn main() {
    let args = Args::parse();
    let process: Process = match args.command {
        OpMode::Attach { pid } => Process::attach_process(pid).unwrap(),
        OpMode::Launch { program } => Process::launch_process(program).unwrap(),
    };
    println!("{:?}", process.pid);
}
