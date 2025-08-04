use std::ffi::CStr;

use libdbg::process;
fn main() {
    let argv: Vec<&CStr> = vec![c"/bin/yes"];
    let pid = process::exec_attach(argv).unwrap();
    println!("{:?}", pid);
}
