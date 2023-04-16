use crate_interface::{call_interface, def_interface};

#[def_interface]
pub trait TrapHandler {
    fn handle_irq(irq_num: usize);
    // more e.g.: handle_page_fault();

    fn syscall(syscall_id: usize, args: [usize; 3]) -> isize;
}

/// Call the external IRQ handler.
#[allow(dead_code)]
pub(crate) fn handle_irq_extern(irq_num: usize) {
    call_interface!(TrapHandler::handle_irq, irq_num);
}

#[allow(dead_code)]
pub(crate) fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    call_interface!(TrapHandler::syscall, syscall_id, args)
}
