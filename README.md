## Shells for Rust

Wrapper around std::process::Command which make the use of Rust for shell scripting more
appealing.

### Simple example

```rust
let (code, stdout, stderr) = sh!("echo '{} + {}' | bc", 1, 3);

assert_eq!(code, 0);
assert_eq!(&stdout[..], "4");
assert_eq!(&stderr[..], "");
```

A mnemotechnic to remember the ordering of the elements in the resulting tuple is the positions
of stdout and stderr, they correspond to the standard streams numbers: 1 and 2 respectively.

If I a missing your favorite (at least partially) POSIX-compliant shell, submit a pull request!
<<<<<<< HEAD

=======
>>>>>>> 3bc8c81a70aa04652f710d9339227da07f2bd43e
