use topgrep;
use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};

struct CommandUnderTest {
    raw: Command,
    stdin: Vec<u8>,
    run: bool,
    stdout: String,
    stderr: String,
}

impl CommandUnderTest {
    fn new() -> CommandUnderTest {
        let mut binary_path = env::current_exe().expect("need current binary path to find binary to test");
        loop {
            {
                let parent = binary_path.parent();
                if parent.is_none() {
                    panic!("Failed to locate binary path from original path: {:?}", env::current_exe());
                }
                let parent = parent.unwrap();
                if parent.is_dir() && parent.file_name().unwrap() == "target" {
                    break;
                }
            }
            binary_path.pop();
        }

        binary_path.push(
            if cfg!(target_os = "windows") {
                format!("{}.exe", env!("CARGO_PKG_NAME"))
            } else {
                env!("CARGO_PKG_NAME").to_string()
            });

        let mut cmd = Command::new(binary_path);

        let mut work_dir = PathBuf::new();
        work_dir.push(env!("CARGO_MANIFEST_DIR"));
        work_dir.push("tests");
        work_dir.push("fixtures");

        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .current_dir(work_dir);

        CommandUnderTest {
            raw: cmd,
            run: false,
            stdin: Vec::new(),
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    fn keep_env(&mut self) -> &mut Self {
        self.raw.envs(env::vars());
        self
    }

    fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.raw.arg(arg);
        self
    }

    fn args<I, S>(&mut self, args: I) -> &mut Self
        where I: IntoIterator<Item=S>,
        S: AsRef<OsStr>
    {
        self.raw.args(args);
        self
    }

    fn pipe_in(&mut self, fixture: &str) -> &mut Self {
        self.stdin = Vec::from(fixture);
        self.raw.stdin(Stdio::piped());
        self
    }

    fn run(&mut self) -> ExitStatus {
        let mut child = self.raw.spawn().expect("failed to run command");

        if self.stdin.len() > 0 {
            let stdin = child.stdin.as_mut().expect("failed to open stdin");
            stdin.write_all(&self.stdin).expect("failed to write to stdin")
        }

        let output = child.wait_with_output().expect("failed waiting for command to complete");
        self.stdout = String::from_utf8(output.stdout).unwrap();
        self.stderr = String::from_utf8(output.stderr).unwrap();
        self.run = true;
        output.status
    }

    fn fails(&mut self) -> &mut Self {
        assert!(!self.run().success(), "expected command to fail");
        self
    }

    fn succeeds(&mut self) -> &mut Self {
        let status = self.run();
        assert!(status.success(), format!(
            "expected command to succeed, but it failed.\nexit code: {}\nstdout: {}\nstderr:{}\n",
            status.code().unwrap(),
            self.stdout,
            self.stderr,
        ));
        self
    }

    fn no_stdout(&mut self) -> &mut Self {
        assert!(self.run, "command has not yet been run, use succeeds()/fails()");
        assert!(self.stdout.is_empty(), format!("expected no stdout, got {}", self.stdout));
        self
    }

    fn no_stderr(&mut self) -> &mut Self {
        assert!(self.run, "command has not yet been run, use succeeds()/fails()");
        assert!(self.stderr.is_empty(), format!("expected no stderr, got {}", self.stderr));
        self
    }

    fn stdout_is(&mut self, expected: &str) -> &mut Self {
        assert!(self.run, "command has not yet been run, use succeeds()/fails()");
        assert_eq!(&self.stdout[..], expected, "stdout does not match expected");
        self
    }

    fn stderr_is(&mut self, expected: &str) -> &mut Self {
        assert!(self.run, "command has not yet been run, use succeeds()/fails()");
        assert_eq!(&self.stderr[..], expected, "stderr does not match expected");
        self
    }
}

#[test]
fn test_1_occurrence() {
    assert_eq!(1, 1);
}

