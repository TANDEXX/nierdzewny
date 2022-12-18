#!/bin/nano
//! module importing cpu support for currently compiled architecture

/// processor support for x86_64
//#[cfg(arch = "x86_64")]
pub mod x86_64;
//#[cfg(arch = "x86_64")]
pub use x86_64 as carch;

//#[cfg(not(arch = "x86_64"))]
//compile_error!{"this architecture isn't supported"}
