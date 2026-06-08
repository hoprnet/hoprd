//! Single-instance guard for a cluster control base.
//!
//! Each running cluster holds an exclusive advisory `flock(2)` on `<control-base>.lock`
//! (default `<data-dir>/cluster.lock`) for its whole lifetime. A second instance pointed
//! at the same control base fails fast instead of clobbering shared ports, identities, and
//! the summary file. The lock is released automatically by the OS when the holder exits —
//! including on `SIGKILL` or a crash — so a stale lock can never wedge a future run.
//!
//! The holder's pid is written into the file purely so the rejection message can
//! tell the operator which process to kill.
//!
//! Caveat: advisory `flock` is only reliable on local filesystems. If the control base
//! points at a Docker bind mount (gRPC-FUSE/virtiofs on macOS) or NFS, mutual exclusion
//! across that boundary is not guaranteed — point `--control-base` at a local path in that
//! case.

use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail};
use nix::{
    errno::Errno,
    fcntl::{Flock, FlockArg},
};

/// An acquired exclusive lock on a cluster data directory.
///
/// Holds the locked file handle; dropping it (i.e. process exit) releases the lock.
/// The lock file itself is intentionally left on disk: its pid contents are only ever
/// read while the lock is held (by a live holder), so a leftover pid never misleads,
/// and unlinking it on shutdown would open a race against a concurrently starting run.
pub struct ClusterLock {
    _handle: Flock<std::fs::File>,
}

impl ClusterLock {
    /// Lock file path for a control-base prefix (`<base>.lock`).
    pub fn path_for(control_base: &Path) -> PathBuf {
        let mut path = control_base.as_os_str().to_owned();
        path.push(".lock");
        PathBuf::from(path)
    }

    /// Acquire the exclusive lock for `control_base`, or fail if another live instance holds it.
    pub fn acquire(control_base: &Path) -> Result<Self> {
        let path = Self::path_for(control_base);
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&path)
            .with_context(|| format!("failed to open lock file {}", path.display()))?;

        let mut handle = match Flock::lock(file, FlockArg::LockExclusiveNonblock) {
            Ok(handle) => handle,
            Err((mut file, Errno::EWOULDBLOCK | Errno::EACCES)) => {
                let mut existing = String::new();
                let _ = file.read_to_string(&mut existing);
                let holder_pid = existing.trim().parse::<u32>().ok();

                // Machine-readable rejection on stdout for tooling; human message on stderr.
                println!(
                    "{}",
                    serde_json::json!({
                        "error": "lock_held",
                        "control_base": control_base.display().to_string(),
                        "holder_pid": holder_pid,
                    })
                );

                let who = match holder_pid {
                    Some(pid) => format!("another localcluster instance (pid {pid})"),
                    None => "another localcluster instance".to_string(),
                };
                bail!(
                    "{who} is already using control base {}; stop it{} or pass a different --control-base",
                    control_base.display(),
                    holder_pid
                        .map(|pid| format!(" (e.g. `kill {pid}`)"))
                        .unwrap_or_default(),
                );
            }
            Err((_, errno)) => {
                return Err(anyhow!(errno))
                    .with_context(|| format!("failed to lock {}", path.display()));
            }
        };

        let pid = std::process::id();
        handle.set_len(0).ok();
        handle.seek(SeekFrom::Start(0)).ok();
        write!(handle, "{pid}").ok();
        handle.flush().ok();

        Ok(Self { _handle: handle })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_acquire_fails_with_pid_hint() {
        let dir = tempfile::tempdir().unwrap();
        let _held = ClusterLock::acquire(dir.path()).expect("first acquire should succeed");

        let msg = match ClusterLock::acquire(dir.path()) {
            Ok(_) => panic!("second acquire should fail"),
            Err(err) => err.to_string(),
        };
        assert!(msg.contains("already using control base"));
        assert!(msg.contains(&std::process::id().to_string()));
    }

    #[test]
    fn reacquire_after_release_succeeds() {
        let dir = tempfile::tempdir().unwrap();
        {
            let _held = ClusterLock::acquire(dir.path()).unwrap();
        }
        assert!(
            ClusterLock::acquire(dir.path()).is_ok(),
            "lock should be free after drop"
        );
    }
}
