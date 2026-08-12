use std::ffi::{OsStr, OsString};
use std::fmt::{self, Write as FmtWrite};
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
#[cfg(feature = "spinners")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "spinners")]
use std::sync::Arc;
#[cfg(feature = "spinners")]
use std::thread::JoinHandle;
#[cfg(feature = "spinners")]
use std::time::Duration;

#[cfg(feature = "spinners")]
use spinners::Spinners;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Command specification to execute.
#[derive(Debug, Clone)]
pub struct Cmd {
    /// Program to execute.
    pub program: OsString,
    /// Program arguments.
    pub args: Vec<OsString>,
    /// Optional working directory.
    pub cwd: Option<PathBuf>,
    /// Environment overrides.
    pub env: Vec<(OsString, OsString)>,
}

impl Cmd {
    /**
     * Creates a new command builder.
     *
     * # Returns
     * Constructed `Cmd` with program name set.
     */
    pub fn new(program: impl Into<OsString>) -> Self {
        Self {
            program: program.into(),
            args: vec![],
            cwd: None,
            env: vec![],
        }
    }

    /**
     * Appends a single argument.
     *
     * # Returns
     * Updated `Cmd`.
     */
    #[must_use]
    pub fn arg(mut self, a: impl Into<OsString>) -> Self {
        self.args.push(a.into());
        self
    }

    /**
     * Appends multiple arguments.
     *
     * # Returns
     * Updated `Cmd`.
     */
    #[must_use]
    pub fn args<I, S>(mut self, it: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(it.into_iter().map(Into::into));
        self
    }

    /**
     * Sets the working directory.
     *
     * # Returns
     * Updated `Cmd`.
     */
    #[must_use]
    pub fn cwd(mut self, p: impl Into<PathBuf>) -> Self {
        self.cwd = Some(p.into());
        self
    }

    /**
     * Adds an environment override.
     *
     * # Returns
     * Updated `Cmd`.
     */
    #[must_use]
    pub fn env(mut self, k: impl Into<OsString>, v: impl Into<OsString>) -> Self {
        self.env.push((k.into(), v.into()));
        self
    }
}

/// Execution errors from running a command.
#[derive(Debug)]
pub enum ExecError {
    /// Process spawn failed.
    Io(std::io::Error),
    /// Process exited with failure status.
    Failed {
        /// Rendered command line for diagnostics.
        cmd: String,
        /// Exit status returned by the process.
        status: ExitStatus,
    },
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io(ref e) => write!(f, "process spawn failed: {e}"),
            Self::Failed {
                ref cmd,
                ref status,
            } => write!(f, "command failed: {cmd} - {status}"),
        }
    }
}

impl From<std::io::Error> for ExecError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::error::Error for ExecError {}

/// Command runner configuration.
#[derive(Debug, Clone, Copy)]
pub struct Runner {
    /// Verbosity level.
    pub verbose: u8,
    /// Whether to skip execution and only print the command.
    pub dry_run: bool,
}

impl Runner {
    /**
     * Runs a command using the configured runner behavior.
     *
     * # Errors
     * Returns `ExecError::Io` if the process cannot be spawned and
     * `ExecError::Failed` if the command exits unsuccessfully.
     */
    pub fn run(self, cmd: &Cmd) -> Result<(), ExecError> {
        self.run_inner(cmd, None)
    }

    /// Runs a command while displaying a spinner message during execution.
    ///
    /// # Errors
    /// Returns `ExecError::Io` if the process cannot be spawned and
    /// `ExecError::Failed` if the command exits unsuccessfully.
    pub fn run_with_spinner(self, cmd: &Cmd, message: &str) -> Result<(), ExecError> {
        self.run_inner(cmd, Some(message))
    }

    /// Shared runner implementation for optional spinner messaging.
    fn run_inner(self, cmd: &Cmd, spinner_message: Option<&str>) -> Result<(), ExecError> {
        let pretty = pretty_cmd(&cmd.program, &cmd.args, cmd.cwd.as_deref());
        if self.dry_run {
            if self.verbose >= 3 {
                eprintln!("{pretty}");
            }
            return Ok(());
        }

        let mut c = Command::new(&cmd.program);
        c.args(&cmd.args);

        if let Some(cwd) = cmd.cwd.as_deref() {
            c.current_dir(cwd);
        }

        for pair in &cmd.env {
            c.env(&pair.0, &pair.1);
        }

        if self.verbose >= 3 {
            reset_terminal_colors();
            eprintln!("{pretty}");
            c.stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
            let status = c.status()?;
            if !status.success() {
                return Err(ExecError::Failed {
                    cmd: pretty,
                    status,
                });
            }
            return Ok(());
        }

        if self.verbose == 2 {
            c.stdin(Stdio::inherit())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let mut spinner = start_spinner(self.verbose, self.dry_run, spinner_message);
            let output = c.output();
            if let Some(ref mut sp) = spinner {
                stop_spinner(sp, spinner_message);
            }
            let output = output?;
            if !output.status.success() {
                let mut err = StandardStream::stderr(ColorChoice::Auto);
                if write_red_line(&mut err, &pretty).is_err() {
                    drop(err);
                }
                if !output.stdout.is_empty() {
                    let mut out = std::io::stdout();
                    if write_raw_bytes(&mut out, &output.stdout).is_err() {
                        // best-effort output
                    }
                }
                if !output.stderr.is_empty() {
                    let mut err_stream = std::io::stderr();
                    if write_raw_bytes(&mut err_stream, &output.stderr).is_err() {
                        // best-effort output
                    }
                }
                return Err(ExecError::Failed {
                    cmd: pretty,
                    status: output.status,
                });
            }
            return Ok(());
        }

        c.stdin(Stdio::inherit())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let status = c.status()?;
        if !status.success() {
            return Err(ExecError::Failed {
                cmd: pretty,
                status,
            });
        }

        Ok(())
    }
}

/// Resets stdout/stderr color state before inheriting command output.
fn reset_terminal_colors() {
    let mut out = StandardStream::stdout(ColorChoice::Auto);
    if out.reset().is_err() {
        // best-effort reset
    }
    let mut err = StandardStream::stderr(ColorChoice::Auto);
    if err.reset().is_err() {
        // best-effort reset
    }

    let mut raw_out = std::io::stdout();
    if raw_out.write_all(b"\x1b[0m").is_err() {
        // best-effort reset
    }
    if raw_out.flush().is_err() {
        // best-effort reset
    }
    let mut raw_err = std::io::stderr();
    if raw_err.write_all(b"\x1b[0m").is_err() {
        // best-effort reset
    }
    if raw_err.flush().is_err() {
        // best-effort reset
    }
}

/// Spinner handle for optional UI feedback.
#[cfg(feature = "spinners")]
struct SpinnerHandle {
    /// Stop signal for the spinner thread.
    stop: Arc<AtomicBool>,
    /// Join handle for the spinner thread.
    join: Option<JoinHandle<()>>,
    /// Message printed alongside the spinner.
    message: String,
}
/// Spinner handle placeholder when the feature is disabled.
#[cfg(not(feature = "spinners"))]
type SpinnerHandle = ();

/// Starts a `SimpleDots` spinner for normal verbosity runs.
#[cfg(feature = "spinners")]
fn start_spinner(verbose: u8, dry_run: bool, message: Option<&str>) -> Option<SpinnerHandle> {
    if dry_run || verbose != 2 {
        return None;
    }
    let msg = message.map_or_else(String::new, |m| format!("kaze: {m}"));
    if msg.is_empty() {
        return None;
    }
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = Arc::clone(&stop);
    let msg_thread = msg.clone();
    let join = std::thread::spawn(move || {
        let spinner = Spinners::SimpleDots;
        let _ = spinner;
        let interval_ms: u64 = 400;
        let frames = [".  ", ".. ", "...", "   "];
        let mut frames = frames.iter().cycle();
        let mut out = StandardStream::stdout(ColorChoice::Auto);
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Green));
        while !stop_thread.load(Ordering::Relaxed) {
            let frame = frames.next().unwrap_or(&"");
            if out.set_color(&spec).is_ok() {
                if write!(out, "\r{msg_thread} {frame}").is_err() {
                    break;
                }
                if out.reset().is_err() {
                    break;
                }
                if out.flush().is_err() {
                    break;
                }
            }
            std::thread::sleep(Duration::from_millis(interval_ms));
        }
    });
    Some(SpinnerHandle {
        stop,
        join: Some(join),
        message: msg,
    })
}

/// No-op spinner initializer when the feature is disabled.
#[cfg(not(feature = "spinners"))]
const fn start_spinner(_: u8, _: bool, _: Option<&str>) -> Option<SpinnerHandle> {
    None
}

/// Stops the active spinner.
#[cfg(feature = "spinners")]
fn stop_spinner(spinner: &mut SpinnerHandle, _message: Option<&str>) {
    spinner.stop.store(true, Ordering::Relaxed);
    if let Some(join) = spinner.join.take() {
        if join.join().is_err() {
            // best-effort cleanup only
        }
    }
    let mut out = StandardStream::stdout(ColorChoice::Auto);
    if write!(out, "\r\x1b[2K").is_err() {
        return;
    }
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Green));
    if out.set_color(&spec).is_ok() {
        if writeln!(out, "{}", spinner.message).is_err() {
            // best-effort write
        }
        if out.reset().is_err() {
            // best-effort reset
        }
    } else if writeln!(out, "{}", spinner.message).is_err() {
        // best-effort write
    }
}

/// No-op spinner stop when the feature is disabled.
#[cfg(not(feature = "spinners"))]
const fn stop_spinner(_: &mut SpinnerHandle, _: Option<&str>) {}

/// Writes a red line to the provided colored output stream.
fn write_red_line(out: &mut dyn WriteColor, msg: &str) -> std::io::Result<()> {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Red));
    out.set_color(&spec)?;
    writeln!(out, "{msg}")?;
    out.reset()?;
    Ok(())
}

/// Writes bytes directly to the provided output stream without color changes.
fn write_raw_bytes(out: &mut dyn IoWrite, bytes: &[u8]) -> std::io::Result<()> {
    out.write_all(bytes)?;
    out.flush()?;
    Ok(())
}

/**
 * Formats a command for logging, including a `(cd ...)` wrapper when a
 * working directory is supplied.
 *
 * # Arguments
 * - `program`: Executable name.
 * - `args`: Argument list to render.
 * - `cwd`: Optional working directory to include in the output.
 *
 * # Returns
 * Rendered command string suitable for logs.
 */
fn pretty_cmd(program: &OsStr, args: &[OsString], cwd: Option<&Path>) -> String {
    let mut s = String::new();
    if let Some(cwd) = cwd {
        let _e = write!(s, "(cd {} && ", cwd.display());
    }

    s.push_str(&shellish(program));

    for a in args {
        s.push(' ');
        s.push_str(&shellish(a));
    }

    if cwd.is_some() {
        s.push(')');
    }

    s
}

/**
 * Returns a shell-friendly representation, quoting whitespace to preserve
 * argument boundaries.
 *
 * # Arguments
 * - `s`: Argument to render.
 *
 * # Returns
 * Shell-friendly string with whitespace preserved.
 */
fn shellish(s: &OsStr) -> String {
    let t = s.to_string_lossy();
    if t.chars().any(char::is_whitespace) {
        format!("{t:?}")
    } else {
        t.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::process::ExitStatusExt;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn dry_run_does_not_execute() {
        let cmd = Cmd::new("definitely-does-not-exist").arg("--help");
        let runner = Runner {
            verbose: 1,
            dry_run: true,
        };
        runner.run(&cmd).expect("dry run should succeed");
    }

    #[test]
    fn run_executes_when_not_dry_run() {
        let cmd = Cmd::new("definitely-does-not-exist").arg("--help");
        let runner = Runner {
            verbose: 0,
            dry_run: false,
        };
        let err = runner.run(&cmd).expect_err("spawn fails");
        assert!(matches!(err, ExecError::Io(_)));
    }

    #[test]
    fn cmd_builder_sets_fields() {
        let cmd = Cmd::new("tool")
            .arg("a")
            .args(["b", "c"])
            .cwd("dir")
            .env("KEY", "VALUE");
        assert_eq!(cmd.program, OsString::from("tool"));
        assert_eq!(
            cmd.args,
            vec![
                OsString::from("a"),
                OsString::from("b"),
                OsString::from("c")
            ]
        );
        assert_eq!(cmd.cwd, Some(PathBuf::from("dir")));
        assert_eq!(
            cmd.env,
            vec![(OsString::from("KEY"), OsString::from("VALUE"))]
        );
    }

    #[test]
    fn pretty_cmd_includes_cwd_and_quotes_whitespace() {
        let cwd = PathBuf::from("/tmp/my dir");
        let cmd = Cmd::new("tool").arg("arg one").arg("arg2").cwd(&cwd);
        let out = pretty_cmd(&cmd.program, &cmd.args, cmd.cwd.as_deref());
        assert!(out.starts_with("(cd "));
        assert!(out.contains("/tmp/my dir"));
        assert!(out.contains("\"arg one\""));
        assert!(out.ends_with(')'));
    }

    #[test]
    fn exec_error_display_formats_messages() {
        let err = ExecError::Failed {
            cmd: "tool --bad".to_owned(),
            status: std::process::ExitStatus::from_raw(1 << 8),
        };
        let msg = err.to_string();
        assert!(msg.contains("command failed:"));
        assert!(msg.contains("tool --bad"));
    }

    #[test]
    fn run_verbose_three_succeeds() {
        let cmd = Cmd::new("sh").args(["-c", "true"]);
        let runner = Runner {
            verbose: 3,
            dry_run: false,
        };
        runner.run(&cmd).expect("run ok");
    }

    #[test]
    fn run_verbose_three_fails() {
        let cmd = Cmd::new("sh").args(["-c", "exit 1"]);
        let runner = Runner {
            verbose: 3,
            dry_run: false,
        };
        let err = runner.run(&cmd).expect_err("run fails");
        assert!(matches!(err, ExecError::Failed { .. }));
    }

    #[test]
    fn run_verbose_three_dry_run_no_spawn() {
        let cmd = Cmd::new("definitely-does-not-exist");
        let runner = Runner {
            verbose: 3,
            dry_run: true,
        };
        runner.run(&cmd).expect("dry run ok");
    }

    #[test]
    fn run_verbose_two_reports_failure_with_output() {
        let cmd = Cmd::new("sh").args(["-c", "echo out; echo err 1>&2; exit 2"]);
        let runner = Runner {
            verbose: 2,
            dry_run: false,
        };
        let err = runner.run(&cmd).expect_err("run fails");
        assert!(matches!(err, ExecError::Failed { .. }));
    }

    #[test]
    fn run_with_spinner_succeeds() {
        let cmd = Cmd::new("sh").args(["-c", "true"]);
        let runner = Runner {
            verbose: 2,
            dry_run: false,
        };
        runner.run_with_spinner(&cmd, "Doing work").expect("run ok");
    }

    #[test]
    fn reset_terminal_colors_best_effort() {
        reset_terminal_colors();
    }

    #[cfg(feature = "spinners")]
    #[test]
    fn spinner_none_when_dry_run_or_not_verbose() {
        assert!(start_spinner(2, true, Some("x")).is_none());
        assert!(start_spinner(1, false, Some("x")).is_none());
    }

    #[cfg(feature = "spinners")]
    #[test]
    fn spinner_starts_and_stops() {
        let mut spinner = start_spinner(2, false, Some("Working")).expect("spinner started");
        stop_spinner(&mut spinner, Some("Working"));
    }

    #[test]
    fn write_raw_bytes_preserves_content() {
        let mut buf = Vec::new();
        write_raw_bytes(&mut buf, b"oops").expect("write bytes");
        assert_eq!(buf, b"oops");
    }
}
