#![no_main]
#![no_std]

use r_linux_syscall;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "_start"]
pub extern "C" fn entrypoint() -> ! {
    unsafe {
        r_linux_syscall::raw::syscall1(60, 71);
    }
    loop {}
}
