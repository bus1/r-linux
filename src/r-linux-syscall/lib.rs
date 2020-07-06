//! Linux Kernel System Calls
//!
//! This crate provides raw and direct access to system calls on linux
//! platforms. Furthermore, it provides the system call numbers of all
//! available linux system calls, named prototypes for all APIs, and
//! definitions of the datatypes used to communicate with the kernel.

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod api;
pub mod arch;
pub mod nr;
pub mod raw;
