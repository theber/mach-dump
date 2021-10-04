//! mach-dump library for parsing Mach-O core dumps taken from macOS and iOS
//!
//! # Example
//! ```rust
//! use std::path::Path;
//! use mach_dump::macho::Macho;
//! fn main() {
//!     let args: Vec<String> = std::env::args().collect();
//! 
//!     if args.len() < 2 {
//!         panic!("Usage: {} <mach-o>", args[0]);
//!     }
//! 
//!     let macho = Macho::load(Path::new(&args[1])).unwrap();
//!     println!("{}", macho.header);
//!     for (i, lc) in macho.load_commands.into_iter().enumerate() {
//!         println!("LC {:02}: {:?}", i, lc);
//!     }
//!     for (i, seg) in macho.segments.into_iter().enumerate() {
//!         println!("Segment {:02}:\n{}", i, seg);
//!     }
//! }
//! ```
#![allow(non_snake_case)]

mod cpu;
mod filetype;
mod flag;
mod load_command;
mod mach_header;
pub mod macho;
mod segment;
