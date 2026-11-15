//! Descriptor-relative parent lookup for persistent pager files.
use std::ffi::{CStr, CString};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

/// Resolve every parent component without following symlinks and pin the final
/// directory. A private directory or a trusted sticky directory prevents a
/// different user from replacing one of our entries after it is created.
pub(crate) unsafe fn open_parent(
    path: *const libc::c_char,
) -> Result<(OwnedFd, CString), libc::c_int> {
    if path.is_null() {
        return Err(libc::EINVAL);
    }
    let bytes = CStr::from_ptr(path).to_bytes();
    let (parent, basename) = match bytes.iter().rposition(|byte| *byte == b'/') {
        Some(index) => (&bytes[..index], &bytes[index + 1..]),
        None => (&b""[..], bytes),
    };
    if basename.is_empty() || basename == b"." || basename == b".." {
        return Err(libc::EINVAL);
    }
    let destination = CString::new(basename).map_err(|_| libc::EINVAL)?;
    let start = if bytes.first() == Some(&b'/') {
        c"/"
    } else {
        c"."
    };
    let fd = libc::open(
        start.as_ptr(),
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    );
    if fd < 0 {
        return Err(*libc::__errno_location());
    }
    let mut directory = OwnedFd::from_raw_fd(fd);
    for component in parent.split(|byte| *byte == b'/') {
        if component.is_empty() || component == b"." {
            continue;
        }
        let name = CString::new(component).map_err(|_| libc::EINVAL)?;
        let fd = libc::openat(
            directory.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        );
        if fd < 0 {
            return Err(*libc::__errno_location());
        }
        directory = OwnedFd::from_raw_fd(fd);
    }
    let mut stat: libc::stat = std::mem::zeroed();
    if libc::fstat(directory.as_raw_fd(), &mut stat) != 0 {
        return Err(*libc::__errno_location());
    }
    let trusted_owner = stat.st_uid == libc::geteuid() || stat.st_uid == 0;
    let private = trusted_owner && stat.st_mode & 0o022 == 0;
    let sticky = trusted_owner && stat.st_mode & libc::S_ISVTX != 0;
    if !private && !sticky {
        return Err(libc::EPERM);
    }
    Ok((directory, destination))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::{symlink, PermissionsExt};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn rejects_intermediate_symlinks_and_pins_a_replaced_directory() {
        let root = std::env::temp_dir().join(format!(
            "rless-parent-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&root).unwrap();
        let first = root.join("first");
        std::fs::create_dir(&first).unwrap();
        let alias = root.join("alias");
        symlink(&first, &alias).unwrap();
        let linked_path = CString::new(alias.join("log").as_os_str().as_bytes()).unwrap();
        assert!(unsafe { open_parent(linked_path.as_ptr()) }.is_err());

        let log_path = CString::new(first.join("log").as_os_str().as_bytes()).unwrap();
        let (pinned, basename) = unsafe { open_parent(log_path.as_ptr()) }.unwrap();
        let moved = root.join("moved");
        std::fs::rename(&first, &moved).unwrap();
        std::fs::create_dir(&first).unwrap();
        let fd = unsafe {
            libc::openat(
                pinned.as_raw_fd(),
                basename.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC,
                0o600,
            )
        };
        assert!(fd >= 0);
        assert_eq!(unsafe { libc::close(fd) }, 0);
        assert!(moved.join("log").exists());
        assert!(!first.join("log").exists());

        std::fs::set_permissions(&first, std::fs::Permissions::from_mode(0o777)).unwrap();
        assert!(unsafe { open_parent(log_path.as_ptr()) }.is_err());
        std::fs::set_permissions(&first, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }
}
