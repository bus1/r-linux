//! Freestanding Syscall
//!
//! This example shows a freestanding linux application with no runtime nor
//! standard library linked. It directly provides the `_start` entry-point
//! picked by the linux linker as ELF entry-point. It then simply invokes the
//! `EXIT` syscall with an exit-code of 71.
//!
//! We need to provide a panic-handler for rust-core to link successfully. For
//! simplicity we just loop in case of panic. Since no core-symbols are called,
//! anyway, this is just about linking successfully. In case you did not
//! specify `abort` as panic-strategy, we also need to provide the exception
//! handler personality routine. Similarly to the panic-handler, we also just
//! provide a dummy, since we never raise exceptions. Note that the exception
//! handler requires unstable rust, so you need to compile via nightly or
//! specify `panic-strategy = "abort"`.
//!
//! Note that our test-suite uses this example to check for cross-language
//! link-time-optimization. It uses the exported symbol-list as reference, so
//! be careful not to pull in symbols from this example other than the ones
//! already there.
//!
//! Note that this example is guarded by the `freestanding` flag, since it
//! cannot be linked with the standard runtime (crt0), as we do not provide the
//! necessary hooks. Instead, you must compile it with `-nostartfiles`. Make
//! sure to provide this when enabling the `freestanding` feature.

#![cfg_attr(feature = "unstable", feature(lang_items))]

#![no_main]
#![no_std]

use r_linux;

#[cfg(feature = "unstable")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "_start"]
pub extern "C" fn entrypoint() -> ! {
    unsafe {
        r_linux::syscall::arch::native::syscall::syscall1(
            r_linux::syscall::arch::native::nr::EXIT,
            71,
        );
    }
    loop {}
}
