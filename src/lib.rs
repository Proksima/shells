//! Wrapper around std::process::Command which make the use of Rust for shell scripting more
//! appealing.
//!
//! ## Simple example
//!
//! ```rust
//! #[macro_use]
//! extern crate shells;
//!
//! fn main() {
//!     let (code, stdout, stderr) = sh!("echo '{} + {}' | cat", 1, 3);
//!
//!     assert_eq!(code, 0);
//!     assert_eq!(&stdout[..], "1 + 3\n");
//!     assert_eq!(&stderr[..], "");
//! }
//! ```
//!
//! A mnemotechnic to remember the ordering of the elements in the resulting tuple is the positions
//! of stdout and stderr, they correspond to the standard streams numbers: 1 and 2 respectively.
//!
//! If I a missing your favorite (at least partially) POSIX-compliant shell, submit a pull request!
//!

/// Macro to execute the given command using the Posix Shell.
///
#[macro_export]
macro_rules! sh   { ( $( $cmd:tt )* ) => { $crate::execute_with("sh",   &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the Almquist Shell.
///
#[macro_export]
macro_rules! ash  { ( $( $cmd:tt )* ) => { $crate::execute_with("ash",  &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the C Shell.
///
#[macro_export]
macro_rules! csh  { ( $( $cmd:tt )* ) => { $crate::execute_with("csh",  &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the Korn Shell.
///
#[macro_export]
macro_rules! ksh  { ( $( $cmd:tt )* ) => { $crate::execute_with("ksh",  &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the Z Shell.
///
#[macro_export]
macro_rules! zsh  { ( $( $cmd:tt )* ) => { $crate::execute_with("zsh",  &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the Bourne Again Shell.
///
#[macro_export]
macro_rules! bash { ( $( $cmd:tt )* ) => { $crate::execute_with("bash", &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the Debian Almquist Shell.
///
#[macro_export]
macro_rules! dash { ( $( $cmd:tt )* ) => { $crate::execute_with("dash", &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the Fish Shell.
///
#[macro_export]
macro_rules! fish { ( $( $cmd:tt )* ) => { $crate::execute_with("fish", &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the MirBSD Korn Shell.
///
#[macro_export]
macro_rules! mksh { ( $( $cmd:tt )* ) => { $crate::execute_with("mksh", &format!($( $cmd )*)) }; }

/// Macro to execute the given command using the TENEX C Shell.
///
#[macro_export]
macro_rules! tcsh { ( $( $cmd:tt )* ) => { $crate::execute_with("tcsh", &format!($( $cmd )*)) }; }

#[doc(hidden)]
pub fn execute_with(shell: &str, cmd: &String) -> (i32, String, String) {
    let mut command = {
        let mut command = ::std::process::Command::new(shell);
        command.arg("-c").arg(cmd);
        command
    };

    match command.output() {
        Ok(output) => {
            (output.status.code().unwrap_or(if output.status.success() { 0 } else { 1 }),
             String::from_utf8_lossy(&output.stdout[..]).into_owned(),
             String::from_utf8_lossy(&output.stderr[..]).into_owned())
        },

        Err(e) => (127, String::new(), e.to_string()),
    }
}
