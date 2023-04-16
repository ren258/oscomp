pub mod process;

use process::*;

pub fn write_str(s: &str) {
    sys_write(1, s.as_bytes());
}

pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
}
