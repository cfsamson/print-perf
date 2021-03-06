# Easier print-optimization for Rust
[![Crates.io](https://img.shields.io/crates/v/print_perf.svg)](https://crates.io/crates/print_perf)
[![Chrono on docs.rs][docsrs-image]][docsrs]

[docsrs-image]: https://docs.rs/print_perf/badge.svg
[docsrs]: https://docs.rs/print_perf

You've probably heard of print debugging, but maybe not the lesser known
member of the print family: print-optimization. 
Sometimes it's interesting to measure the time some part of your code uses, 
but you won't set up everything you need for profiling your entire program 
(or you don't have that option in the environment you're working in).


Doing this in Rust requires some boilerplate at the moment, especially if 
you want to print out an easily readable output that you can navigate 
directly to the relevant lines of code from. This crate aims to make this
easier to do:

You can use two methods to measure the elapsed time:
1. Lap: measures elapsed time from the last lap (or the starting point if it's the first lap)
2. Split: measures elapsed time from the starting point where you call it in your code

Here's two examples:

```rust
use print_perf::*; 
// or explicit print_perf::{perf, Perf};
use std::time::Duration;
use std::thread::sleep;

fn add(a: i32, b: i32) -> i32 {
    sleep(Duration::from_millis(100));
    a + b
}

fn main() {
    let add_p = perf!("add fn");
    let result = add(4, 4);
    add_p.end();
    // ^-- prints: 0.100140446 (add fn) @ [src/main.rs:9]
    assert_eq!(result, 8);
}
```

You can also add split times like this:
```rust
 use print_perf::*;
# use std::time::Duration;
# use std::thread::sleep;
fn add(a: i32, b: i32) -> i32 {
   sleep(Duration::from_millis(100));
   a + b
}

fn main() {
    let p = perf!("add fn");
    let _result = add(4, 4);
    p.split("add");
    let _div = _result / 2;
    p.split("div");
    p.end();
}
```

# Dependecies

I don't think super-small convenience code bits like this should pull inn any dependencies so I try to avoid them. This crate currently depends on no other crates.

# Known bugs

The coloring will not output correctly on all windows terminals. The coloring is deactivated on windows release builds.

# Stability

The exact output printed by this macro should not be relied upon
and is subject to future changes.

# Panics

Panics if writing to `io::stderr` fails.
[stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)