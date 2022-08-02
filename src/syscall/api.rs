//! System Call API
//!
//! This module provides symbols for all available system calls, implementing a
//! uniform API to call into the kernel. Any architecture-peculiarities are
//! hidden from the caller, except if they leak into external data definitions.
//! That is, binary formatting of argument structures still need to be
//! performed by the caller. However, correct syscall invocation and splitting
//! across registers is performed by these helpers.
//!
//! Note that the kernel is particularly good in using any unused bits in input
//! and output values to assign new meaning to. Therefore, while we try to be
//! as specific in the type-system as possible, we must also make sure to be
//! future-proof and allow passing invalid values along just as well.

/// Error Number
///
/// The linux kernel commonly returns error information as an integer code
/// between 1 and 4096. These have associated symbolic names and are used each
/// for a wide range of possible errors, some more specific, some more generic.
///
/// We encode the error numbers as a u16, to better encapsulate their range.
/// This can be easily converted to the `i32` used by most C standard
/// libraries.
///
/// A value of 0 is not a valid error number, same as any value greater than
/// 4096. It depends on the context how these invalid values are treated.
pub type Errno = u16;

/// Task Identifier
///
/// Individual tasks all come with a set of identifiers to set them apart. They
/// have historically been called process identifiers, hence the name `PID`.
/// However, the most specific one on linux is the task identifier, a unique
/// number to identify each linux task (i.e., thread of execution). Other
/// IDs of this type include the thread- and process-group ID, as well as the
/// session ID.
///
/// Linux always represents these IDs with an `i32`. Note that 0, as well as
/// negative values are considered invalid. Technically, anything up to I32_MAX
/// can be a valid ID, but usually the kernel is configured with a soft limit
/// way below this (currently 4-million).
///
/// The `0` value is used by some syscalls to signal special conditions. For
/// instance `fork(2)` returns 0 in the child process, to distinguish it from
/// the return in the parent process.
/// Negative values are commonly used to identify task groups rather than
/// individual tasks. See the `kill(2)` system call for an example.
///
/// It depends on context whether "invalid" PID values have a meaning assigned.
pub type Pid = i32;

/// Exit Task
///
/// Stop the current execution and tear down this task. Other tasks of a
/// possible thread group are left around. See the linux task model for
/// information how threads and processes map to linux tasks.
///
/// Takes a single argument `code` which specifies the exit condition of the
/// running task.
///
/// This system call never returns, under no circumstances. This also implies
/// that this system call cannot be interrupted.
///
/// The kernel uses the lower byte of `code` as exit-code of the task. The
/// remaining bits of `code` are ignored.
pub unsafe fn exit(code: u32) -> ! {
    super::raw::syscall1(
        super::arch::native::nr::EXIT,
        code as usize,
    );
    core::unreachable!("`r_linux::syscall::api::exit()` unexpectedly returned");
}

/// Fork Task
///
/// Create a new thread of execution by forking the calling task. The calling
/// task will simply return from this system call and continue as normal. The
/// return value is the PID of the newly created task.
///
/// Unlike the calling task, the newly created task will start execution at
/// exactly the position where this system call was invoked and return with a
/// `None` value from this system call. The new task will be mostly a duplicate
/// of the original task. Some values, however, are not inherited and instead
/// reset to their default or freshly allocated.
///
/// See the `clone(2)` system-call for a more detailed description of the
/// creation of new tasks.
///
/// On error, an error-code is returned and no new process is created.
pub unsafe fn fork() -> Result<Option<Pid>, Errno> {
    super::raw::syscall0(
        super::arch::native::nr::FORK,
    ).to_result().map(|v| {
        let p = Pid::try_from(v).unwrap();
        match p {
            0 => None,
            _ => Some(p),
        }
    })
}

/// Restart System Call
///
/// This system call continues an interrupted system call with the same
/// parameters it was initially called, adjusted only for the time difference
/// between the original syscall and now.
///
/// This system call is used by the kernel itself to resume system calls of
/// frozen processes. Whenever a system call is interrupted, the kernel saves
/// the system call parameters and restarts the system call with the same
/// parameters once the task is resumed again. However, for system calls that
/// take relative time-frames as arguments, the kernel usually needs to adjust
/// these relative time-frames for the elapsed time. For those system calls,
/// the kernel refrains from restarting the system call directly and instead
/// changes the system call number of the to-be-restarted call to this system
/// call. When this system call is then invoked, the kernel fetches the
/// original system call and its parameters from its internal state, adjusts
/// the relative timeout, and then restarts the original system call.
///
/// There is usually no reason why you would ever invoke this system call from
/// user-space. Moreover, even when the kernel triggers a syscall restart with
/// this system call, it never leaves kernel space, and thus user-space should
/// never see this system call at all. Tracing debuggers might see it, though.
/// And they are the only ones that might reasonable interfere with it.
///
/// If no system call is to be resumed, this system call returns `EINTR`.
/// Otherwise, it resumes the original system call with adjusted relative time
/// parameters and returns the result of the resumed system call.
pub unsafe fn restart_syscall() -> Result<usize, Errno> {
    super::raw::syscall0(
        super::arch::native::nr::RESTART_SYSCALL,
    ).to_result()
}

/// Read from File-Descriptor
///
/// XXX
pub use crate::syscall::arch::native::nr::READ;

/// Write to File-Descriptor
///
/// XXX
pub use crate::syscall::arch::native::nr::WRITE;

/// Open File
///
/// XXX
pub use crate::syscall::arch::native::nr::OPEN;

/// Close File Descriptor
///
/// `fn sys_close(fd: u32) -> i32`
///
/// Close the file-descriptor specified by the first argument. First, the
/// file-descriptor is unlinked from the file-descriptor table of the calling
/// task, then the reference count of the open file-description is decremented
/// and possibly released thereafter.
///
/// This system call always unlinks the file-descriptor from the
/// file-descriptor table of the calling task. That is, if the passed
/// file-descriptor is valid, it is always invalidated by this system call,
/// regardless of the return code, even if `EINTR` is returned. You must never
/// repeat or restart this system call.
///
/// Takes a single argument `fd` which specifies the file-descriptor to close.
/// Unlike most other system calls, this type is `unsigned`, but that should
/// make no observable difference to the caller.
///
/// This system call returns `EBADF` if the specified file-descriptor was
/// invalid. In this case, this system call was a no-op. In all other cases,
/// regardless of the return code, the system call actually closed the
/// file-descriptor. Moreover, if this did not release the underlying open
/// file-description, then this will always return 0.
/// However, if this system call ends up releasing the underlying open
/// file-description, the teardown operation of just this can trigger any kind
/// of writeback, cache-invalidation, resource relinking, rcu grace period,
/// etc., and thus might take a considerable amount of time. Furthermore, for
/// historical reasons, this final teardown can also return arbitrary error
/// codes from deep down in the kernel device drivers (even confusingly
/// allowing `EBADF`). Given that, you should never check the return value of
/// this system call, but always assume it succeeded.
///
/// Lastly, you must never assume that a call to this operation actually
/// performs a final teardown of the underlying open file-description. Any
/// temporary, parallel kernel maintenance thread might pin the same open
/// file-description for a short moment, and thus delay the teardown for an
/// arbitrary amount of time. This especially means you *MUST NOT* rely on this
/// function implying an `fsync()`, unless you verified this via the kernel
/// sources yourself.
pub use crate::syscall::arch::native::nr::CLOSE;

/// XXX
pub use crate::syscall::arch::native::nr::LSEEK;

/// XXX
pub use crate::syscall::arch::native::nr::GETPID;

/// XXX
pub use crate::syscall::arch::native::nr::PIPE2;

/// XXX
pub use crate::syscall::arch::native::nr::MEMFD_CREATE;

/// XXX
pub use crate::syscall::arch::native::nr::READLINKAT;

/// XXX
pub use crate::syscall::arch::native::nr::STATX;

/// XXX
pub use crate::syscall::arch::native::nr::COPY_FILE_RANGE;

/// XXX
pub use crate::syscall::arch::native::nr::DUP;

/// XXX
pub use crate::syscall::arch::native::nr::DUP2;

/// XXX
pub use crate::syscall::arch::native::nr::DUP3;
