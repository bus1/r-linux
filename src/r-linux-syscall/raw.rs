//! Raw System Calls
//!
//! This module provides raw and direct access to system calls on linux
//! platforms. It exports 7 different functions, one for each possible number
//! of arguments you can pass to a syscall (`syscall0` through `syscall6`). It
//! is always safe to use `syscall6()` and set unused arguments to any value.
//! For performance reasons, you might want to prefer the matching call,
//! though.
//!
//! Linux system calls take between 0 and 6 arguments, each argument is passed
//! as a native integer. Furthermore, every system call has a return value,
//! which also is a native integer. Depending on the platform you run on, the
//! actual underlying datatype will have different size constraints (e.g., on
//! 32bit machines all arguments are usually 32bit integers, but on 64bit
//! machines you pass 64bit integers). You should consult the documentation of
//! each system call to understand how individual arguments are passed. Be
//! warned, there are even system calls that flip argument order depending on
//! the architecture (for historical reasons, trying to provide binary
//! compatibility to existing platforms). Use the wrapper definitions in the
//! `api` module to get a verified function prototype for each system call.
//!
//! The return value of a system call is limited to a native integer.
//! Furthermore, 4096 values are reserved for error codes. For most syscalls
//! it is enough to interpret the return value as signed integer and consider
//! any negative value as error. However, in some special cases this is not
//! correct. Therefore, the `Retval` type provides small accessors to check
//! whether the return value is an error code or not. If performance is not
//! a concern, it also provides a conversion to `Result`.

/// System Call Return Value
///
/// On linux platforms, system calls return native integers as result. A
/// special range is used for errors. This type wraps this result type and
/// provides accessors to its underlying values.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Retval(usize);

impl Retval {
    /// Create a new return-value from raw data
    #[inline(always)]
    pub const fn from_usize(v: usize) -> Retval {
        Retval(v)
    }

    /// Return raw underlying data of a return-value
    #[inline(always)]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Check whether this is an error-return
    #[inline(always)]
    pub const fn is_error(self) -> bool {
        self.0 > core::usize::MAX - 4096
    }

    /// Check whether this is a success-return
    #[inline(always)]
    pub const fn is_success(self) -> bool {
        !self.is_error()
    }

    /// Return the error-code unchecked
    ///
    /// # Safety
    ///
    /// This does not verify that `self` is actually an error-return. It
    /// assumes the caller verified it.
    #[inline(always)]
    pub unsafe fn error_unchecked(self) -> usize {
        !self.0 + 1
    }

    /// Return the error-code
    ///
    /// If `self` is not actually an error-return, this will panic.
    pub fn error(self) -> usize {
        if self.is_error() {
            unsafe { self.error_unchecked() }
        } else {
            panic!("called `r_linux_syscall::Retval::error()` on a success value")
        }
    }

    /// Return the success-value unchecked
    ///
    /// # Safety
    ///
    /// This does not verify that `self` is actually a success-return. It
    /// assumes the caller verified it.
    #[inline(always)]
    pub unsafe fn unwrap_unchecked(self) -> usize {
        self.0
    }

    /// Return the success value
    ///
    /// If `self` is not a success-return, this will panic.
    pub fn unwrap(self) -> usize {
        if self.is_success() {
            unsafe { self.unwrap_unchecked() }
        } else {
            panic!("called `r_linux_syscall::Retval::unwrap()` on an error value")
        }
    }

    /// Convert into a Result
    ///
    /// This converts the return value into a rust-native Result type. This
    /// maps the error-return to `Err(code)` and the success-return
    /// to `Ok(usize)`. This allows using the rich convenience library of the
    /// `Result` type, rather than re-implementing them for this native type.
    pub fn to_result(self) -> Result<usize, usize> {
        if self.is_error() {
            Err(!self.0 + 1)
        } else {
            Ok(self.0)
        }
    }
}

// Syscall Assembly
//
// These symbols are provided by our native code, because there is currently no
// stable way to inline assembly into rust code. Once inline-assembly is stable,
// we can provide these symbols as native-rust code.
extern {
    fn r_linux_asm_syscall0(
        nr: usize,
    ) -> usize;
    fn r_linux_asm_syscall1(
        nr: usize,
        arg0: usize,
    ) -> usize;
    fn r_linux_asm_syscall2(
        nr: usize,
        arg0: usize,
        arg1: usize,
    ) -> usize;
    fn r_linux_asm_syscall3(
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
    ) -> usize;
    fn r_linux_asm_syscall4(
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    ) -> usize;
    fn r_linux_asm_syscall5(
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    ) -> usize;
    fn r_linux_asm_syscall6(
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize;
}

/// Invoke System Call With 0 Arguments
///
/// This invokes the system call with the specified system-call-number. No
/// arguments are passed to the system call.
///
/// # Safety
///
/// * System calls can have arbitrary side-effects. It is the responsibility of
///   the caller to consider all effects of a system call and take required
///   precautions.
#[inline(always)]
pub unsafe fn syscall0(
    nr: usize,
) -> Retval {
    #[allow(unused_unsafe)]
    unsafe {
        Retval::from_usize(r_linux_asm_syscall0(nr))
    }
}

/// Invoke System Call With 1 Argument
///
/// This invokes the system call with the specified system-call-number. The
/// provided argument is passed to the system call unmodified.
///
/// # Safety
///
/// * System calls can have arbitrary side-effects. It is the responsibility of
///   the caller to consider all effects of a system call and take required
///   precautions.
#[inline(always)]
pub unsafe fn syscall1(
    nr: usize,
    arg0: usize,
) -> Retval {
    #[allow(unused_unsafe)]
    unsafe {
        Retval::from_usize(r_linux_asm_syscall1(nr, arg0))
    }
}

/// Invoke System Call With 2 Arguments
///
/// This invokes the system call with the specified system-call-number. The
/// provided arguments are passed to the system call unmodified.
///
/// # Safety
///
/// * System calls can have arbitrary side-effects. It is the responsibility of
///   the caller to consider all effects of a system call and take required
///   precautions.
#[inline(always)]
pub unsafe fn syscall2(
    nr: usize,
    arg0: usize,
    arg1: usize,
) -> Retval {
    #[allow(unused_unsafe)]
    unsafe {
        Retval::from_usize(r_linux_asm_syscall2(nr, arg0, arg1))
    }
}

/// Invoke System Call With 3 Arguments
///
/// This invokes the system call with the specified system-call-number. The
/// provided arguments are passed to the system call unmodified.
///
/// # Safety
///
/// * System calls can have arbitrary side-effects. It is the responsibility of
///   the caller to consider all effects of a system call and take required
///   precautions.
#[inline(always)]
pub unsafe fn syscall3(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Retval {
    #[allow(unused_unsafe)]
    unsafe {
        Retval::from_usize(r_linux_asm_syscall3(nr, arg0, arg1, arg2))
    }
}

/// Invoke System Call With 4 Arguments
///
/// This invokes the system call with the specified system-call-number. The
/// provided arguments are passed to the system call unmodified.
///
/// # Safety
///
/// * System calls can have arbitrary side-effects. It is the responsibility of
///   the caller to consider all effects of a system call and take required
///   precautions.
#[inline(always)]
pub unsafe fn syscall4(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Retval {
    #[allow(unused_unsafe)]
    unsafe {
        Retval::from_usize(r_linux_asm_syscall4(nr, arg0, arg1, arg2, arg3))
    }
}

/// Invoke System Call With 5 Arguments
///
/// This invokes the system call with the specified system-call-number. The
/// provided arguments are passed to the system call unmodified.
///
/// # Safety
///
/// * System calls can have arbitrary side-effects. It is the responsibility of
///   the caller to consider all effects of a system call and take required
///   precautions.
#[inline(always)]
pub unsafe fn syscall5(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Retval {
    #[allow(unused_unsafe)]
    unsafe {
        Retval::from_usize(r_linux_asm_syscall5(nr, arg0, arg1, arg2, arg3, arg4))
    }
}

/// Invoke System Call With 6 Arguments
///
/// This invokes the system call with the specified system-call-number. The
/// provided arguments are passed to the system call unmodified.
///
/// # Safety
///
/// * System calls can have arbitrary side-effects. It is the responsibility of
///   the caller to consider all effects of a system call and take required
///   precautions.
#[inline(always)]
pub unsafe fn syscall6(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Retval {
    #[allow(unused_unsafe)]
    unsafe {
        Retval::from_usize(r_linux_asm_syscall6(nr, arg0, arg1, arg2, arg3, arg4, arg5))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retval_check() {
        //
        // Check basic functionality of the `Retval` type and verify it has
        // the same semantics as the system call ABI.
        //

        let success_values = [
            0, 1, 2, 3,
            254, 255, 256, 257,
            65534, 65535, 65536, 65537,
            core::usize::MAX / 2,
            core::usize::MAX / 2 + 1,
            core::usize::MAX - 4097,
            core::usize::MAX - 4096,
        ];

        for v in &success_values {
            let r = Retval::from_usize(*v);

            assert_eq!(r, Retval(*v));
            assert_eq!(r.as_usize(), *v);
            assert_eq!(r.is_success(), true);
            assert_eq!(r.is_error(), false);
            assert_eq!(unsafe { r.unwrap_unchecked() }, *v);
            assert_eq!(r.unwrap(), *v);
            assert_eq!(r.to_result(), Ok(*v));
        }

        let error_values = [
            (4096, core::usize::MAX - 4095),
            (4095, core::usize::MAX - 4094),
            (4094, core::usize::MAX - 4093),
            (4093, core::usize::MAX - 4092),
            (4, core::usize::MAX - 3),
            (3, core::usize::MAX - 2),
            (2, core::usize::MAX - 1),
            (1, core::usize::MAX),
        ];

        for (c, v) in &error_values {
            let r = Retval::from_usize(*v);

            assert_eq!(r, Retval(*v));
            assert_eq!(r.as_usize(), *v);
            assert_eq!(r.is_success(), false);
            assert_eq!(r.is_error(), true);
            assert_eq!(unsafe { r.error_unchecked() }, *c);
            assert_eq!(r.error(), *c);
            assert_eq!(r.to_result(), Err(*c));
        }

        let r = Retval::from_usize(71);

        // verify copy behavior
        let copy = r;
        assert_ne!(&copy as *const _, &r as *const _);
        assert_eq!(copy, r);

        // verify clone support
        let clone = r.clone();
        assert_ne!(&clone as *const _, &r as *const _);
        assert_eq!(clone, r);

        // verify comparisons
        let (com1, com2) = (Retval::from_usize(71), Retval::from_usize(72));
        assert_eq!(r, com1);
        assert_ne!(r, com2);
    }

    #[test]
    #[should_panic]
    fn retval_error_panic() {
        //
        // Verify `Retval::error()` panics on success-values.
        //

        Retval::from_usize(0).error();
    }

    #[test]
    #[should_panic]
    fn retval_unwrap_panic() {
        //
        // Verify `Retval::unwrap()` panics on error-values.
        //

        Retval::from_usize(core::usize::MAX).unwrap();
    }

    #[test]
    fn link_check() {
        //
        // Simply check that the linked assembly is actually available. This
        // pulls in the symbols and prevents the dead-code-elimination from
        // hiding missing symbols.
        //

        assert_ne!(r_linux_asm_syscall0 as *const () as usize, 0);
        assert_ne!(r_linux_asm_syscall1 as *const () as usize, 0);
        assert_ne!(r_linux_asm_syscall2 as *const () as usize, 0);
        assert_ne!(r_linux_asm_syscall3 as *const () as usize, 0);
        assert_ne!(r_linux_asm_syscall4 as *const () as usize, 0);
        assert_ne!(r_linux_asm_syscall5 as *const () as usize, 0);
        assert_ne!(r_linux_asm_syscall6 as *const () as usize, 0);
    }

    #[test]
    fn syscall_check() {
        // Test validity of `syscall0()`.
        let r0 = unsafe { syscall0(crate::nr::GETPID) };
        assert_eq!(r0.unwrap() as u32, std::process::id());

        // Test validity of `syscall1()`.
        let r1 = unsafe { syscall1(crate::nr::DUP, 0) };
        assert!(r1.unwrap() > 2);
        let r2 = unsafe { syscall1(crate::nr::CLOSE, r1.unwrap() as usize) };
        assert_eq!(r2.unwrap(), 0);

        // Test validity of `syscall2()`.
        let r3 = unsafe { syscall2(crate::nr::DUP2, 0, r1.unwrap() as usize) };
        assert_eq!(r3.unwrap(), r1.unwrap());
        let r4 = unsafe { syscall1(crate::nr::CLOSE, r3.unwrap() as usize) };
        assert_eq!(r4.unwrap(), 0);

        // Test validity of `syscall3()`.
        // XXX: We should pass `O_CLOEXEC` and verify it is set. Otherwise, we
        //      do not really test for the 3rd argument, as 0 is just a too
        //      common value.
        let r5 = unsafe { syscall3(crate::nr::DUP3, 0, r1.unwrap() as usize, 0) };
        assert_eq!(r5.unwrap(), r1.unwrap());
        let r6 = unsafe { syscall1(crate::nr::CLOSE, r5.unwrap() as usize) };
        assert_eq!(r6.unwrap(), 0);

        // XXX: Tests missing for syscall{4,5,6}().
    }
}
