//! You've probably heard of print debugging, but maybe not the lesser known
//! member of the print family: print-optimization.
//! Sometimes it's interesting to measure the time some part of your code uses,
//! but you won't set up everything you need for profiling your entire program
//! (or you don't have that option in the environment you're working in).
//! Doing this in Rust requires some boilerplate at the moment, especially if
//! you want to print out an easily readable output that you can navigate.
//! directly to the relevant lines of code from. This crate aims to make this
//! easier to do:
//! Here's an example:
//!
//! ```rust
//! use print_perf::*;
//! # use std::time::Duration;
//! # use std::thread::sleep;
//! fn add(a: i32, b: i32) -> i32 {
//!    sleep(Duration::from_millis(100));
//!    a + b
//! }
//! 
//! fn main() {
//!     let add_p = perf!("add fn");
//!     let result = add(4, 4);
//!     add_p.end();
//!     // ^-- prints: 0.100140446 (add fn) @ [src/main.rs:9]
//!
//!     assert_eq!(result, 8);
//! }
//! ```
//! 
//! # Example with splits
//! 
//! ```rust
//!  use print_perf::*;
//! # use std::time::Duration;
//! # use std::thread::sleep;
//! fn add(a: i32, b: i32) -> i32 {
//!    sleep(Duration::from_millis(100));
//!    a + b
//! }
//! 
//! fn main() {
//!     let p = perf!("add fn");
//!     let _result = add(4, 4);
//!     p.split("add");
//!     let _div = _result / 2;
//!     p.split("div");
//!     p.end();
//! }
//! ```
//!
//! # Stability
//!
//! The exact output printed by this macro should not be relied upon
//! and is subject to future changes.
//!
//! # Panics
//!
//! Panics if writing to `io::stderr` fails.
//!
//!
//! [stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
//!
/// This is what you get returned from the macro. You probably won't create this directly.
pub struct Perf {
    start: std::time::Instant,
    start_line: String,
    ident: String,
}

impl Perf {
    pub fn new(ident: String, start_line: String) -> Self {
        Perf {
            start: std::time::Instant::now(),
            start_line,
            ident,
        }
    }

    pub fn split(&self, msg: &str) {
        let elapsed = self.start.elapsed();
        if cfg!(all(target_os = "windows", not(debug_assertions))) {
            eprintln!(
                "{}.{} ({} - {})",
                elapsed.as_secs(),
                format!("{:09}", elapsed.subsec_nanos()),
                self.ident,
                msg,
            );
        } else {
            eprintln!(
                "\x1B[33m\x1B[1m{}.{} ({} - {})\x1B[0m",
                elapsed.as_secs(),
                format!("{:09}", elapsed.subsec_nanos()),
                self.ident,
                msg
            );
        }
    }

    pub fn end(&self) {
        let elapsed = self.start.elapsed();
        if cfg!(all(target_os = "windows", not(debug_assertions))) {
            eprintln!(
                "{}.{} ({} - end) @ {}",
                elapsed.as_secs(),
                format!("{:09}", elapsed.subsec_nanos()),
                self.ident,
                self.start_line,
            );
        } else {
            eprintln!(
                "\x1B[33m\x1B[1m{}.{} ({} - end)\x1B[0m @ {}",
                elapsed.as_secs(),
                format!("{:09}", elapsed.subsec_nanos()),
                self.ident,
                self.start_line,
            );
        }
    }
}

/// Se crate documentation for example on how to use
#[macro_export]
macro_rules! perf {
    ($val:expr) => {
        match $val {
            i => {
                let ident = format!("{}", i);
                let start_line = format!("[{}:{}]", file!(), line!());
                Perf::new(ident, start_line)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    fn add(a: i32, b: i32) -> i32 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        a + b
    }
    #[test]
    fn basic_example() {
        // to see output use: cargo test -- --nocapture
        let p = perf!("add fn");
        let _result = add(4, 4);
        p.end();
    }

    #[test]
    fn split_test() {
        let p = perf!("add fn");
        let _result = add(4, 4);
        p.split("add");
        let _div = _result / 2;
        p.split("div");
        p.end();
    }

    fn add_fast(a: i32, b: i32) -> i32 {
        a + b
    }
    #[test]
    fn fast_example() {
        // to see output use: cargo test -- --nocapture
        let p = perf!("add fn");
        let _result = add_fast(4, 4);
        p.end();
    }
}
