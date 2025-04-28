#![cfg_attr(not(feature = "socketcan"), no_std)]

#![allow(non_snake_case, non_camel_case_types, dead_code)]
#[cfg(feature="socketcan")]
pub mod canbus;

pub mod canb;
pub mod canc;

#[cfg(feature="socketcan")]
pub use packed_struct;
#[cfg(feature="socketcan")]
pub use tokio_socketcan;
#[cfg(feature="socketcan")]
pub use tokio_socketcan_isotp;

