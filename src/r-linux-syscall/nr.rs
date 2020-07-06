//! System Call Numbers
//!
//! This module provides symbols for all available system calls, which resolve
//! to the system call number of the respective system call. Only system calls
//! that resolve to the same call on all supported platforms are listed (this
//! guarantees that the symbols here are well defined and always available). If
//! you need access to all system calls of the compiled platform, use the
//! symbols from `arch::native::nr`.
//!
//! The symbols provided here always resolve to the system call of the native
//! platform of this compilation. If you need the symbols of a specific
//! platform, use `arch::<platform>::nr`.
//!
//! For a system call to be listed here, it needs to be available on all
//! platforms we support, have the same number of arguments and argument order
//! on all platforms, and have the same semantics everywhere. This usually
//! involves a manual auditing of each syscall. Therefore, this list is not
//! exhaustive, and a lot of legacy system calls might never make it on this
//! list. Therefore, resort to `arch::native::nr` for missing system calls. For
//! those you need to perform the respective audits yourself, though.

/// Restart System Call
///
/// `fn sys_restart_syscall() -> usize`
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
pub use crate::arch::native::nr::RESTART_SYSCALL;

/// Exit Task
///
/// `fn sys_exit(code: u32) -> !`
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
pub use crate::arch::native::nr::EXIT;

/// Fork Task
///
/// XXX
pub use crate::arch::native::nr::FORK;

/// Read from File-Descriptor
///
/// XXX
pub use crate::arch::native::nr::READ;

/// Write to File-Descriptor
///
/// XXX
pub use crate::arch::native::nr::WRITE;

/// Open File
///
/// XXX
pub use crate::arch::native::nr::OPEN;

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
pub use crate::arch::native::nr::CLOSE;

/// XXX
pub use crate::arch::native::nr::LSEEK;

/// XXX
pub use crate::arch::native::nr::GETPID;

/// XXX
pub use crate::arch::native::nr::PIPE2;

/// XXX
pub use crate::arch::native::nr::MEMFD_CREATE;

/// XXX
pub use crate::arch::native::nr::READLINKAT;

/// XXX
pub use crate::arch::native::nr::STATX;

/// XXX
pub use crate::arch::native::nr::COPY_FILE_RANGE;

/// XXX
pub use crate::arch::native::nr::DUP;

/// XXX
pub use crate::arch::native::nr::DUP2;

/// XXX
pub use crate::arch::native::nr::DUP3;
