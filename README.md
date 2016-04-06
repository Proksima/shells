## Shells for Rust

Wrapper around std::process::Command which make the use of Rust for shell scripting more
appealing.

[![](http://meritbadge.herokuapp.com/shells)](https://crates.io/crates/shells)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Travis Build Status](https://travis-ci.org/Proksima/shells.svg?branch=master)](https://travis-ci.org/Proksima/shells)

### Documentation

http://proksima.github.io/shells-doc/shells/index.html

### Simple example

```rust
#[macro_use]
extern crate shells;

fn main() {
    let (code, stdout, stderr) = sh!("echo '{} + {}' | bc", 1, 3);

    assert_eq!(code, 0);
    assert_eq!(&stdout[..], "4\n");
    assert_eq!(&stderr[..], "");
}
```

A mnemotechnic to remember the ordering of the elements in the resulting tuple is the positions
of stdout and stderr, they correspond to the standard streams numbers: 1 and 2 respectively.

If I a missing your favorite (at least partially) POSIX-compliant shell, submit a pull request!

