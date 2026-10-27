//! Safe public facade over a single-use DoomGeneric worker.
//! The only unsafe code is the private native module; no C-to-Rust callbacks.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub mod wad;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;
pub const FRAME_BYTES: usize = WIDTH * HEIGHT * 4;

#[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
mod native;
#[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
pub use native::Game;
