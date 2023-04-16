#![no_std]
#![no_main]

#[no_mangle]

fn main() {
    libax::write_str("test sys_write success!\n");
    libax::exit(0);
}
