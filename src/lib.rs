#![cfg_attr(not(feature = "threaded"), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "threaded")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "threaded")]
pub mod channel;
pub mod job_queue;
#[cfg(feature = "threaded")]
pub mod lock;
