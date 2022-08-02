//! System Calls on x86_64
//!
//! This implements the syscall entries for x86_64. One function for each
//! possible number of arguments is provided: syscall0 to syscall6.
//!
//! The implementation uses the x86_64-`syscall` instruction to enter the
//! kernel, as it is the recommended way to enter the linux kernel on x86_64 as
//! of this time.
//!
//! Arguments are passed as:
//!     Nr: rax
//!     Args: rdi, rsi, rdx, r10, r8, r9
//! Return value is in:
//!     Ret: rax
//! Always clobbered:
//!     rcx, r11
//!
//! The entry-points are currently not marked as `readonly`. That is, the
//! system calls are allowed to modify memory. If necessary, alternative calls
//! with `readonly` (or maybe even `pure`) can be provided in the future.

#[cfg(target_arch = "x86_64")]
#[inline]
#[export_name = "r_linux_asm_syscall0"]
pub unsafe fn syscall0(
    nr: usize,
) -> usize {
    let mut r: usize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") nr => r,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    r
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[export_name = "r_linux_asm_syscall1"]
pub unsafe fn syscall1(
    nr: usize,
    arg0: usize,
) -> usize {
    let mut r: usize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") nr => r,
        in("rdi") arg0,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    r
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[export_name = "r_linux_asm_syscall2"]
pub unsafe fn syscall2(
    nr: usize,
    arg0: usize,
    arg1: usize,
) -> usize {
    let mut r: usize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") nr => r,
        in("rdi") arg0,
        in("rsi") arg1,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    r
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[export_name = "r_linux_asm_syscall3"]
pub unsafe fn syscall3(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> usize {
    let mut r: usize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") nr => r,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    r
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[export_name = "r_linux_asm_syscall4"]
pub unsafe fn syscall4(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    let mut r: usize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") nr => r,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    r
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[export_name = "r_linux_asm_syscall5"]
pub unsafe fn syscall5(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let mut r: usize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") nr => r,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    r
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[export_name = "r_linux_asm_syscall6"]
pub unsafe fn syscall6(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let mut r: usize;

    core::arch::asm!(
        "syscall",
        inlateout("rax") nr => r,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        in("r9") arg5,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );

    r
}
