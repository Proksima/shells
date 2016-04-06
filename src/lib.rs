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
//!
//!     // Using the new `wrap_*` macros.
//!     assert_eq!(wrap_sh!("echo '{} + {}' | cat", 1, 3).unwrap(), "1 + 3\n");
//! }
//! ```
//!
//! A mnemotechnic to remember the ordering of the elements in the resulting tuple is the positions
//! of stdout and stderr, they correspond to the standard streams numbers: 1 and 2 respectively.
//!
//! The implementation for all the different shells is the same: the arguments of the macro is
//! passed directly to `format!` and the resulting string is passed to the shell using its '-c'
//! command line option. Thus you can use `sh!` and friends the same way you would use `format!` or
//! `println!`.
//!

/// Type returned by the `wrap_*` family of macros. Will either be `Ok(stdout)` or an error
/// containing code, stdout and stderr resulting from executing the command.
///
pub type Result = ::std::result::Result<String, Error>;

/// Struct holding the resulting environment after executing a failed command with the `wrap_*`
/// family of macros. It implements the Error trait and its implementation of the Display trait is
/// identical to the implementation of the Display trait of its `stderr` field.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "Unix command failed."
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.stderr)
    }
}

/// Macro to execute the given command using the Posix Shell.
///
#[macro_export]
macro_rules! sh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("sh", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the Almquist Shell.
///
#[macro_export]
macro_rules! ash {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("ash", &format!($( $cmd )*))
    }}; 
}

/// Macro to execute the given command using the C Shell.
///
#[macro_export]
macro_rules! csh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("csh", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the Korn Shell.
///
#[macro_export]
macro_rules! ksh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("ksh", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the Z Shell.
///
#[macro_export]
macro_rules! zsh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("zsh", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the Bourne Again Shell.
///
#[macro_export]
macro_rules! bash {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("bash", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the Debian Almquist Shell.
///
#[macro_export]
macro_rules! dash {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("dash", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the Fish Shell.
///
#[macro_export]
macro_rules! fish {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("fish", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the MirBSD Korn Shell.
///
#[macro_export]
macro_rules! mksh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("mksh", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the TENEX C Shell.
///
#[macro_export]
macro_rules! tcsh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("tcsh", &format!($( $cmd )*))
    }};
}

/// Macro to execute the given command using the Posix Shell and wraping the resulting tuple into a
/// Result.
///
#[macro_export]
macro_rules! wrap_sh {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("sh", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the Almquist Shell and wraping the resulting tuple
/// into a Result.
///
#[macro_export]
macro_rules! wrap_ash {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("ash", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the C Shell and wraping the resulting tuple into a
/// Result.
///
#[macro_export]
macro_rules! wrap_csh {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("csh", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the Korn Shell and wraping the resulting tuple into a
/// Result.
///
#[macro_export]
macro_rules! wrap_ksh {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("ksh", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the Z Shell and wraping the resulting tuple into a
/// Result.
///
#[macro_export]
macro_rules! wrap_zsh {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("zsh", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the Bourne Again Shell and wraping the resulting tuple
/// into a Result.
///
#[macro_export]
macro_rules! wrap_bash {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("bash", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the Debian Almquist Shell and wraping the resulting
/// tuple into a Result.
///
#[macro_export]
macro_rules! wrap_dash {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("dash", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the Fish Shell and wraping the resulting tuple into a
/// Result.
///
#[macro_export]
macro_rules! wrap_fish {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("fish", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the MirBSD Korn Shell and wraping the resulting tuple
/// into a Result.
///
#[macro_export]
macro_rules! wrap_mksh {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("mksh", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

/// Macro to execute the given command using the TENEX C Shell and wraping the resulting tuple into
/// a Result.
///
#[macro_export]
macro_rules! wrap_tcsh {
    ( $( $cmd:tt )* ) => {{
        match $crate::execute_with("tcsh", &format!($( $cmd )*)) {
            (0, stdout, _) => Ok(stdout),

            (code, stdout, stderr) => {
                Err($crate::Error {
                    code: code,
                    stdout: stdout,
                    stderr: stderr,
                })
            },
        }
    }};
}

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

        Err(e) => (126, String::new(), e.to_string()),
    }
}
