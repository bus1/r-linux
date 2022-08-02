//! Architecture Definitions for x86
//!
//! This module provides the linux-kernel API definitions specific
//! to x86.
//!
//! No documentation is provided for the individual symbols and definitions.
//! They are meant to match the official API of the linux kernel. Either see
//! the official linux kernel documentation for help, or look at the interfaces
//! exposed by the `api` module.

pub mod nr;
pub mod syscall;
