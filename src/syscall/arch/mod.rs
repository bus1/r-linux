//! Architecture Dependent Definitions
//!
//! Depending on the architecture, the linux ABI can differ greatly. This
//! module contains the architecture dependent definitions and entry-points
//! for each supported architecture.
//!
//! All code for all architectures is always available. Use the `native`
//! module to get a link to the architecture native to your compilation.
//! Alternatively, use `cfg(target_arch = "xyz")` to test for the architecture
//! of your choice.

// Target Architecture
//
// We need architecture-dependent assembly to invoke system calls. To avoid
// spurious linker errors in dependent crates, we check for supported
// architectures here and error out right away.
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
)))]
compile_error!("Target architecture not supported.");

pub mod x86;
pub mod x86_64;

/// Native Architecture
///
/// This module re-exports the symbols of the architecture native to this
/// compilation (i.e., the target architecture). That is, rather than gating
/// your access to symbols in the `arch::<id>` module via
/// `cfg(target_arch = "id")`, you can directly use `arch::native` and rely on
/// compile-time verification that the symbols are available.
///
/// Note that your code will become architecture-dependent when you use this
/// re-export, as it can and will expose differences between the ABIs of the
/// linux kernel architecture. However, in a lot of cases this might be what
/// you want, and it also can simplify bootstrapping applications quite a bit.
///
/// Note that for documentation reasons, this module shows the symbols of the
/// `x86_64` architecture. However, depending on what target you compile for,
/// other symbols will be exported.
#[cfg(doctest)]
pub mod native {
    pub use super::x86_64::*;
}

#[cfg(all(not(doctest), target_arch = "x86"))]
pub mod native {
    pub use super::x86::*;
}

#[cfg(all(not(doctest), target_arch = "x86_64"))]
pub mod native {
    pub use super::x86_64::*;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn native_check() {
        //
        // Verify the `native` module is available and links to an
        // actual architecture.
        //

        assert_ne!(native::nr::EXIT, 0);
    }

    #[test]
    fn arch_availability() {
        //
        // Verify all architectures are always compiled in and accessible. We
        // simply check for their hard-coded `nr::EXIT` symbols here.
        //

        assert_eq!(x86::nr::EXIT, 1);
        assert_eq!(x86_64::nr::EXIT, 60);
    }

    #[test]
    fn syscall_check() {
        //
        // Verify the `syscall0` to `syscall6` symbols are always available on
        // the native architectures with the same prototype. Note that
        // non-native prototypes are not available, since it would be non-sense
        // to compile their assembly.
        //

        let s0: unsafe fn(
            usize,
        ) -> usize = native::syscall::syscall0;
        let s1: unsafe fn(
            usize,
            usize,
        ) -> usize = native::syscall::syscall1;
        let s2: unsafe fn(
            usize,
            usize,
            usize,
        ) -> usize = native::syscall::syscall2;
        let s3: unsafe fn(
            usize,
            usize,
            usize,
            usize,
        ) -> usize = native::syscall::syscall3;
        let s4: unsafe fn(
            usize,
            usize,
            usize,
            usize,
            usize,
        ) -> usize = native::syscall::syscall4;
        let s5: unsafe fn(
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        ) -> usize = native::syscall::syscall5;
        let s6: unsafe fn(
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        ) -> usize = native::syscall::syscall6;

        assert_ne!(s0 as *const () as usize, 0);
        assert_ne!(s1 as *const () as usize, 0);
        assert_ne!(s2 as *const () as usize, 0);
        assert_ne!(s3 as *const () as usize, 0);
        assert_ne!(s4 as *const () as usize, 0);
        assert_ne!(s5 as *const () as usize, 0);
        assert_ne!(s6 as *const () as usize, 0);
    }

    #[test]
    fn arch_check() {
        //
        // Check for basic architecture properties that need to be satisfied by
        // all linux architectures and is relied upon.
        //

        assert!(core::mem::size_of::<usize>() >= 4);
        assert!(core::mem::align_of::<usize>() >= 4);
    }
}
