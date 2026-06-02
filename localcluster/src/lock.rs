//! Single-instance guard for a cluster data directory.
//!
//! Each running cluster holds an exclusive advisory `flock(2)` on
//! `<data-dir>/cluster.lock` for its whole lifetime. A second instance pointed at
//! the same data directory fails fast instead of clobbering shared ports,
//! identities, and the summary file. The lock is released automatically by the OS
//! when the holder exits — including on `SIGKILL` or a crash — so a stale lock can
//! never wedge a future run.
//!
//! The holder's pid is written into the file purely so the rejection message can
//! tell the operator which process to kill.
//!
//! Caveat: advisory `flock` is only reliable on local filesystems. If `--data-dir`
//! points at a Docker bind mount (gRPC-FUSE/virtiofs on macOS) or NFS, mutual
//! exclusion across that boundary is not guaranteed.

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
    /// Default lock path for a given data directory.
    pub fn path_for(data_dir: &Path) -> PathBuf {
        data_dir.join("cluster.lock")
    }

    /// Acquire the exclusive lock for `data_dir`, or fail if another live instance holds it.
    pub fn acquire(data_dir: &Path) -> Result<Self> {
        let path = Self::path_for(data_dir);
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
                let holder = existing.trim();
                let who = if holder.is_empty() {
                    "another localcluster instance".to_string()
                } else {
                    format!("another localcluster instance (pid {holder})")
                };
                bail!(
                    "{who} is already using data directory {}; stop it{} or pass a different --data-dir",
                    data_dir.display(),
                    holder
                        .parse::<i32>()
                        .map(|p| format!(" (e.g. `kill {p}`)"))
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
        assert!(msg.contains("already using data directory"));
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
