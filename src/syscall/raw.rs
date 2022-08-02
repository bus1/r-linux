//! Raw System Calls
//!
//! This module provides raw and direct access to system calls on linux
//! platforms. It exports 7 different functions, one for each possible number
//! of arguments you can pass to a syscall (`syscall0` through `syscall6`). It
//! is always safe to use `syscall6()` and set unused arguments to any value.
//! For performance reasons, you might want to prefer the matching call,
//! though.
//!
//! This implementation is optimized to allow inlining of the system-call
//! invocation into the calling function. That is, when xLTO is used, the
//! syscall setup and instruction will be inlined into the caller, and thus
//! allows fast and efficient kernel calls. On common architectures, the inline
//! assembly is now stable rust, so no cross-language LTO is needed, anyway.
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
//! compatibility to existing platforms). Use the wrapper definitions to get a
//! verified function prototype for each system call.
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
    pub const fn from_usize(v: usize) -> Retval {
        Retval(v)
    }

    /// Return raw underlying data of a return-value
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Check whether this is an error-return
    pub const fn is_error(self) -> bool {
        self.0 > core::usize::MAX - 4096
    }

    /// Check whether this is a success-return
    pub const fn is_success(self) -> bool {
        !self.is_error()
    }

    /// Return the error-code unchecked
    ///
    /// # Safety
    ///
    /// This does not verify that `self` is actually an error-return. It
    /// assumes the caller verified it.
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
            panic!("called `r_linux::syscall::raw::Retval::error()` on a success value")
        }
    }

    /// Return the success-value unchecked
    ///
    /// # Safety
    ///
    /// This does not verify that `self` is actually a success-return. It
    /// assumes the caller verified it.
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
            panic!("called `r_linux::syscall::raw::Retval::unwrap()` on an error value")
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
            Err(unsafe { self.error_unchecked() })
        } else {
            Ok(unsafe { self.unwrap_unchecked() })
        }
    }
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
pub unsafe fn syscall0(
    nr: usize,
) -> Retval {
    Retval::from_usize(
        super::arch::native::syscall::syscall0(
            nr,
        )
    )
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
pub unsafe fn syscall1(
    nr: usize,
    arg0: usize,
) -> Retval {
    Retval::from_usize(
        super::arch::native::syscall::syscall1(
            nr,
            arg0,
        )
    )
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
pub unsafe fn syscall2(
    nr: usize,
    arg0: usize,
    arg1: usize,
) -> Retval {
    Retval::from_usize(
        super::arch::native::syscall::syscall2(
            nr,
            arg0,
            arg1,
        )
    )
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
pub unsafe fn syscall3(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Retval {
    Retval::from_usize(
        super::arch::native::syscall::syscall3(
            nr,
            arg0,
            arg1,
            arg2,
        )
    )
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
pub unsafe fn syscall4(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Retval {
    Retval::from_usize(
        super::arch::native::syscall::syscall4(
            nr,
            arg0,
            arg1,
            arg2,
            arg3,
        )
    )
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
pub unsafe fn syscall5(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Retval {
    Retval::from_usize(
        super::arch::native::syscall::syscall5(
            nr,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        )
    )
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
pub unsafe fn syscall6(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Retval {
    Retval::from_usize(
        super::arch::native::syscall::syscall6(
            nr,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        )
    )
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
    fn syscall0_check() {
        //
        // Test validity of `syscall0()`.
        //
        // Tested syscall: GETPID
        //

        let r0 = unsafe { syscall0(crate::syscall::arch::native::nr::GETPID) };
        assert_eq!(r0.unwrap() as u32, std::process::id());
    }

    #[test]
    fn syscall1_check() {
        //
        // Test validity of `syscall1()`.
        //
        // Tested syscall: CLOSE
        //
        // We run `pipe2()` and verify the `close()` syscall accepts the values
        // without complaint.
        //

        let mut p0: [u32; 2] = [0, 0];

        let r0 = unsafe {
            syscall2(
                crate::syscall::arch::native::nr::PIPE2,
                p0.as_mut_ptr() as usize,
                0,
            ).unwrap()
        };
        assert_eq!(r0, 0);
        assert!(p0[0] > 2);
        assert!(p0[1] > 2);
        assert_ne!(p0[0], p0[1]);

        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                p0[0] as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);
        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                p0[1] as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);
    }

    #[test]
    fn syscall2_check() {
        //
        // Test validity of `syscall2()`.
        //
        // Tested syscall: PIPE2
        //
        // We run `pipe2()` and verify the `close()` syscall accepts the values
        // without complaint.
        //

        let mut p0: [u32; 2] = [0, 0];

        let r0 = unsafe {
            syscall2(
                crate::syscall::arch::native::nr::PIPE2,
                p0.as_mut_ptr() as usize,
                0,
            ).unwrap()
        };
        assert_eq!(r0, 0);
        assert!(p0[0] > 2);
        assert!(p0[1] > 2);
        assert_ne!(p0[0], p0[1]);

        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                p0[0] as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);
        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                p0[1] as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);
    }

    #[test]
    fn syscall3_check() {
        //
        // Test validity of `syscall3()`.
        //
        // Tested syscall: WRITE / READ
        //
        // We create a pipe, write to one end and verify we can read the same
        // data from the other end.
        //

        let mut p0: [u32; 2] = [0, 0];
        let mut b0: [u8; 16] = [0; 16];

        let r0 = unsafe {
            syscall2(
                crate::syscall::arch::native::nr::PIPE2,
                p0.as_mut_ptr() as usize,
                0,
            ).unwrap()
        };
        assert_eq!(r0, 0);
        assert!(p0[0] > 2);
        assert!(p0[1] > 2);
        assert_ne!(p0[0], p0[1]);

        let r0 = unsafe {
            syscall3(
                crate::syscall::arch::native::nr::WRITE,
                p0[1] as usize,
                "foobar".as_ptr() as usize,
                6 as usize,
            )
            .unwrap()
        };
        assert_eq!(r0, 6);

        let r0 = unsafe {
            syscall3(
                crate::syscall::arch::native::nr::READ,
                p0[0] as usize,
                b0.as_mut_ptr() as usize,
                6 as usize,
            )
            .unwrap()
        };
        assert_eq!(r0, 6);
        assert_eq!(core::str::from_utf8(&b0[..6]), Ok("foobar"));

        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                p0[0] as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);
        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                p0[1] as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);
    }

    #[test]
    fn syscall4_check() {
        //
        // Test validity of `syscall4()`.
        //
        // Tested syscall: READLINKAT
        //
        // We create a memfd and then query `/proc` for the link-value of the
        // memfd. This is ABI and needs to be the (annotated) name that we
        // passed to `memfd_create()`.
        //

        let mut b0: [u8; 128] = [0; 128];

        let f0 = unsafe {
            syscall2(
                crate::syscall::arch::native::nr::MEMFD_CREATE,
                "foobar\x00".as_ptr() as usize,
                0,
            ).unwrap()
        };
        assert!(f0 > 2);

        let r0 = unsafe {
            syscall4(
                crate::syscall::arch::native::nr::READLINKAT,
                core::usize::MAX - 100 + 1, // AT_FDCWD
                format!("/proc/self/fd/{}\x00", f0).as_str().as_ptr() as usize,
                b0.as_mut_ptr() as usize,
                128 - 1,
            )
            .unwrap()
        };
        assert_eq!(r0, 23);
        assert_eq!(
            core::str::from_utf8(&b0[..23]).unwrap(),
            "/memfd:foobar (deleted)",
        );

        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                f0,
            ).unwrap()
        };
        assert_eq!(r0, 0);
    }

    #[test]
    fn syscall5_check() {
        //
        // Test validity of `syscall5()`.
        //
        // Tested syscall: STATX
        //
        // Run `statx()` on `STDIN`, but pass `AT_SYMLINK_NOFOLLOW`. This
        // means we instead get information on the symlink in `/proc`. Check
        // that this was correctly interpreted by the kernel and verify the
        // `S_IFLNK` flag is set on the result.
        //

        let mut b0: [u32; 1024] = [0; 1024];

        let r0 = unsafe {
            syscall5(
                crate::syscall::arch::native::nr::STATX,
                core::usize::MAX - 100 + 1, // AT_FDCWD
                "/proc/self/fd/0".as_ptr() as usize,
                0x100, // AT_SYMLINK_NOFOLLOW
                0x1,   // STATX_TYPE
                b0.as_mut_ptr() as usize,
            )
            .unwrap()
        };
        assert_eq!(r0, 0);
        assert_ne!(b0[0] & 0x1, 0);
        assert_eq!(
            unsafe {
                core::ptr::read_unaligned(
                    (b0.as_ptr() as *const u16).offset(14)
                )
            } & 0o170000, // S_IFMT
            0o120000, // S_IFLNK
        );
    }

    #[test]
    fn syscall6_check() {
        //
        // Test validity of `syscall6()`.
        //
        // Tested syscall: COPY_FILE_RANGE
        //
        // Create two memfd instances, write a text into the first one. Use
        // the `copy_file_range()` syscall to copy the data over into the other
        // memfd and then verify via `read()`. Use `lseek()` to reset the file
        // position between calls.
        //

        let mut b0: [u8; 128] = [0; 128];

        let f0 = unsafe {
            syscall2(
                crate::syscall::arch::native::nr::MEMFD_CREATE,
                "foobar\x00".as_ptr() as usize,
                0,
            ).unwrap()
        };
        let f1 = unsafe {
            syscall2(
                crate::syscall::arch::native::nr::MEMFD_CREATE,
                "foobar\x00".as_ptr() as usize,
                0,
            ).unwrap()
        };
        assert!(f0 > 2);
        assert!(f1 > 2);
        assert_ne!(f0, f1);

        let r0 = unsafe {
            syscall3(
                crate::syscall::arch::native::nr::WRITE,
                f0 as usize,
                "foobar".as_ptr() as usize,
                6 as usize,
            )
            .unwrap()
        };
        assert_eq!(r0, 6);

        let r0 = unsafe {
            syscall3(
                crate::syscall::arch::native::nr::LSEEK,
                f0 as usize,
                0 as usize,
                0 as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);

        let r0 = unsafe {
            syscall6(
                crate::syscall::arch::native::nr::COPY_FILE_RANGE,
                f0 as usize,
                0 as usize,
                f1 as usize,
                0 as usize,
                6 as usize,
                0 as usize,
            )
            .unwrap()
        };
        assert_eq!(r0, 6);

        let r0 = unsafe {
            syscall3(
                crate::syscall::arch::native::nr::LSEEK,
                f1 as usize,
                0 as usize,
                0 as usize,
            ).unwrap()
        };
        assert_eq!(r0, 0);

        let r0 = unsafe {
            syscall3(
                crate::syscall::arch::native::nr::READ,
                f1 as usize,
                b0.as_mut_ptr() as usize,
                6 as usize,
            )
            .unwrap()
        };
        assert_eq!(r0, 6);
        assert_eq!(core::str::from_utf8(&b0[..6]), Ok("foobar"));

        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                f1,
            ).unwrap()
        };
        assert_eq!(r0, 0);
        let r0 = unsafe {
            syscall1(
                crate::syscall::arch::native::nr::CLOSE,
                f0,
            ).unwrap()
        };
        assert_eq!(r0, 0);
    }
}
