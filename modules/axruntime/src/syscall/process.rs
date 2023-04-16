

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    axtask::exit(0);
}
