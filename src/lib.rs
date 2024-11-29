#![no_std]
#![allow(clippy::all)]

pub use cty::*;

pub mod base;

#[cfg(feature = "intrin")]
pub mod intrin;
#[cfg(feature = "netio")]
pub mod netio;
#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

