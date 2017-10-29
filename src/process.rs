use std::ops::Deref;

use std::process::{Command, Stdio};
use std::process::{ChildStdin, ChildStdout, ChildStderr};

use result::*;

pub struct Process {
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>
}

impl Process {
    pub fn new_from_str<T>(cmd: T) -> Result<Process>
        where T: Deref<Target=str> {

        let mut args = cmd.split_whitespace();

        let child = Command::new(args.next().unwrap())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
            .spawn()
            .unwrap();

        let ps = Process {
            stdin: child.stdin,
            stdout: child.stdout,
            stderr: child.stderr
        };

        Ok(ps)
    }

    /// Example:
    /// ```
    /// match process.stdin() {
    ///     Some(ref mut r) => {
    ///         r.write(b"Test").unwrap();
    ///         r.flush().unwrap();
    ///     },
    ///     None => ()
    /// };
    /// ```
    pub fn stdin<'a>(&'a mut self) -> Option<&'a mut ChildStdin> {
        self.stdin.as_mut()
    }

    /// Example:
    /// ```
    /// match p.stdout() {
    ///     Some(ref mut r) => {
    ///         let mut buffer = String::new();
    ///         r.read_to_string(&mut buffer).unwrap();
    ///         println!("{}", buffer);
    ///     },
    ///     None => ()
    /// };
    /// ```
    pub fn stdout<'a>(&'a mut self) -> Option<&'a mut ChildStdout> {
        self.stdout.as_mut()
    }

    pub fn stderr<'a>(&'a mut self) -> Option<&'a mut ChildStderr> {
        self.stderr.as_mut()
    }

    pub fn close_stdin(&mut self) {
        self.stdin = None;
    }
}
