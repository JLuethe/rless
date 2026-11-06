use std::ffi::OsString;
use std::io::Write;
use std::os::fd::OwnedFd;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::net::UnixStream;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Work(PathBuf);
impl Work {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "rless-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_rless"));
        command
            .current_dir(&self.0)
            .env_clear()
            .env("HOME", &self.0)
            .env("LC_ALL", "C")
            .env("PATH", "/usr/bin:/bin")
            .env("LESSHISTFILE", "-");
        command
    }

    fn run(&self, args: &[&str], input: &[u8]) -> Output {
        let mut child = self
            .command()
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input).unwrap();
        drop(stdin);
        child.wait_with_output().unwrap()
    }
}
impl Drop for Work {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn redirected_stdin_preserves_all_byte_values_and_unterminated_lines() {
    let work = Work::new();
    let input: Vec<u8> = (0..=255).chain(b"last line".iter().copied()).collect();
    let output = work.run(&["-N", "-R", "-S"], &input);
    assert!(output.status.success());
    assert_eq!(output.stdout, input);
    assert!(output.stderr.is_empty());
}

#[test]
fn operands_accept_non_utf8_names() {
    let work = Work::new();
    let name = OsString::from_vec(b"quote' and invalid-\xff".to_vec());
    std::fs::write(work.0.join(&name), b"\xff\0contents\r\n").unwrap();
    let output = work.command().arg("--").arg(name).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"\xff\0contents\r\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn later_open_errors_follow_the_reference_output_descriptor() {
    let work = Work::new();
    std::fs::write(work.0.join("valid"), b"data").unwrap();
    let first = work.run(&["missing", "valid"], b"");
    assert!(first.status.success());
    assert_eq!(first.stdout, b"data");
    assert_eq!(first.stderr, b"missing: No such file or directory\n");
    let last = work.run(&["valid", "missing"], b"");
    assert!(last.status.success());
    assert_eq!(last.stdout, b"datamissing: No such file or directory\n");
    assert!(last.stderr.is_empty());
}

#[test]
fn copy_does_not_truncate_a_large_unterminated_line() {
    let work = Work::new();
    let data = vec![b'x'; 1024 * 1024 + 17];
    std::fs::write(work.0.join("long"), &data).unwrap();
    let output = work.run(&["long"], b"");
    assert!(output.status.success());
    assert_eq!(output.stdout, data);
    assert!(output.stderr.is_empty());
}

fn broken_pipe(ignore: bool) -> Output {
    let work = Work::new();
    std::fs::write(work.0.join("data"), vec![b'x'; 1024 * 1024]).unwrap();
    let (writer, reader) = UnixStream::pair().unwrap();
    drop(reader);
    let fd: OwnedFd = writer.into();
    let mut command = work.command();
    command
        .arg("data")
        .stdout(Stdio::from(fd))
        .stderr(Stdio::piped());
    unsafe {
        command.pre_exec(move || {
            libc::signal(
                libc::SIGPIPE,
                if ignore { libc::SIG_IGN } else { libc::SIG_DFL },
            );
            Ok(())
        });
    }
    command.output().unwrap()
}

#[test]
fn a_closed_output_pipe_preserves_default_sigpipe() {
    assert_eq!(broken_pipe(false).status.signal(), Some(libc::SIGPIPE));
}

#[test]
fn an_inherited_ignored_sigpipe_stays_ignored() {
    let result = broken_pipe(true);
    // Ignore remains inherited; the resulting EPIPE is now an ordinary I/O
    // failure rather than being silently reported as successful output.
    assert_eq!(result.status.signal(), None);
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(result.stderr, b"rless: write error\n");
}

#[test]
fn lessmsg_rejects_mismatched_native_format_conversions_before_use() {
    let work = Work::new();
    for (index, replacement) in [
        "unknown_action_X %p %p %p %p %p %p %p %p\n",
        "unknown_action_X %s %s %s %s %s %s %s %s\n",
        "unknown_action_X %n\n",
        "unknown_action_X %1$s\n",
        "unknown_action_X %*s\n",
    ]
    .iter()
    .enumerate()
    {
        let message_path = work.0.join(format!("messages-{index}"));
        std::fs::write(&message_path, replacement).unwrap();
        for secure in [false, true] {
            let mut command = work.command();
            command
                .env("LESSMSG", &message_path)
                .env("LESSKEY_CONTENT", "x definitely-not-an-action");
            if secure {
                command
                    .env("LESSSECURE", "1")
                    .env("LESSSECURE_ALLOW", "lesskey");
            }
            let output = command.arg("--").arg("missing").output().unwrap();
            assert!(output.status.code().is_some(), "{replacement:?}");
            let diagnostics = [output.stdout, output.stderr].concat();
            assert!(
                diagnostics
                    .windows(b"invalid message format placeholders".len())
                    .any(|window| window == b"invalid message format placeholders"),
                "{replacement:?}: {diagnostics:?}"
            );
            assert!(!diagnostics.windows(2).any(|window| window == b"0x"));
        }
    }
}

#[test]
fn empty_lessmsg_replacement_does_not_create_a_null_slice() {
    let work = Work::new();
    let message_path = work.0.join("empty-message");
    std::fs::write(&message_path, b"press_RETURN \n").unwrap();
    for secure in [false, true] {
        let mut command = work.command();
        command.env("LESSMSG", &message_path);
        if secure {
            command.env("LESSSECURE", "1");
        }
        let output = command.arg("--").arg("missing").output().unwrap();
        assert!(output.status.code().is_some(), "{output:?}");
        let diagnostics = [output.stdout, output.stderr].concat();
        assert!(
            !diagnostics
                .windows(b"unsafe precondition".len())
                .any(|window| window == b"unsafe precondition"),
            "{diagnostics:?}"
        );
    }
}
