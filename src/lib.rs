/// You've probably heard of print debugging, but maybe not the lesser known
/// member of the print family: print-optimization. 
/// Sometimes it's interesting to measure the time some part of your code uses, 
/// but you won't set up everything you need for profiling your entire program 
/// (or you don't have that option in the environment you're working in).
/// Doing this in Rust requires some boilerplate at the moment, especially if 
/// you want to printi out a easily readable output that you can navigate. 
/// directly to the relevant lines of code from. This crate aims to make this
/// easier to do:
/// Here's an example:
///
/// ```rust
/// use perf::*;
/// fn add(a: i32, b: i32) -> i32 {
///        std::thread::sleep(std::time::Duration::from_millis(100));
///        a + b
/// }
/// 
/// fn main() {
/// let add_p = perf!("add fn");
/// let result = add(4, 4);
/// add_p.end();
/// //     ^-- prints: 0.100140446 (add fn) [src/main.rs:9]->[src/main.rs:11]
///
/// assert_eq!(result, 8);
/// }
/// ```
///
/// # Stability
///
/// The exact output printed by this macro should not be relied upon
/// and is subject to future changes.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
///
/// [stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
///

pub struct Perf {
    start: std::time::Instant,
    start_line: String,
    ident: String,
}

impl Perf {

    pub fn new(ident: String) -> Self {
        let start_line = format!("[{}:{}]", file!(), line!());
                Perf {
                    start: std::time::Instant::now(),
                    start_line,
                    ident,
                }
    }
    pub fn end(&self) {
        let elapsed = self.start.elapsed();
        eprintln!(
            "{}.{} ({}) {}->[{}:{}]",
            elapsed.as_secs(),
            elapsed.subsec_nanos(),
            self.ident,
            self.start_line,
            file!(),
            line!()
        );
    }
}

#[macro_export]
macro_rules! perf {
    ($val:expr) => {
        match $val {
            i => {
                let ident = format!("{}", i);
                Perf::new(ident)
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
        let p = perf!("add fn");
        let result = add(4, 4);
        p.end();
    }
}