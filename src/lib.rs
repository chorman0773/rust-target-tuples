#![deny(warnings, unsafe_code)]
#![cfg_attr(not(any(doc, test)), no_std)]

extern crate alloc;

mod pieces;

pub use pieces::*;
