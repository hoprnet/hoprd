//! Node state belongs to the lifetime of the chain localcluster starts.

use std::{fs, io::ErrorKind, path::Path};

use anyhow::{Context, Result};

/// Clear only the harness-owned node directories when a fresh chain was started.
///
/// Call with the data-directory lock held, before spawning any nodes. Keep the
/// root itself, identities, logs and control files: unlinking a held lock would
/// let another harness acquire a different inode and run against the same data.
/// External chains and node restarts must retain their pending transactions.
pub fn prepare_node_state(data_dir: &Path, fresh_chain: bool) -> Result<()> {
    if !fresh_chain {
        return Ok(());
    }

    // Include nodes from a previous, larger cluster.
    for id in 0..crate::identity::MAX_NUM_NODES {
        let node_dir = data_dir.join(format!("db_{id}"));
        let metadata = match fs::symlink_metadata(&node_dir) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(error).context("reading node state directory"),
        };
        if metadata.is_dir() {
            fs::remove_dir_all(&node_dir)
        } else {
            // Do not follow a symlink outside the managed directory.
            fs::remove_file(&node_dir)
        }
        .with_context(|| format!("resetting node state {}", node_dir.display()))?;
    }
    Ok(())
}

/// Move the old CWD-based Curvy database when resuming an external chain.
/// Never choose between two databases or replace an existing destination.
pub fn adopt_legacy_curvy_state(node_dir: &Path, legacy_path: &Path) -> Result<()> {
    if !legacy_path.try_exists()? {
        return Ok(());
    }
    let destination = node_dir.join(
        legacy_path
            .file_name()
            .context("invalid Curvy state path")?,
    );
    anyhow::ensure!(
        !destination.try_exists()?,
        "Curvy state exists at both {} and {}; select the correct database before restarting",
        legacy_path.display(),
        destination.display(),
    );
    fs::create_dir_all(node_dir)?;
    fs::rename(legacy_path, &destination).with_context(|| {
        format!(
            "moving existing Curvy state from {} to {}; move it manually before restarting if the paths are on different filesystems",
            legacy_path.display(),
            destination.display(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_chain_adopts_legacy_curvy_state_without_overwriting_either_copy() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let legacy = dir.path().join("curvy-pix-address.redb");
        let node_dir = dir.path().join("db_0");
        let destination = node_dir.join(legacy.file_name().unwrap());
        fs::write(&legacy, "pending notes")?;

        adopt_legacy_curvy_state(&node_dir, &legacy)?;
        assert!(!legacy.exists());
        assert_eq!(fs::read_to_string(&destination)?, "pending notes");
        // Subsequent launches have nothing to migrate.
        adopt_legacy_curvy_state(&node_dir, &legacy)?;
        fs::write(&legacy, "different state")?;
        assert!(adopt_legacy_curvy_state(&node_dir, &legacy).is_err());
        assert_eq!(fs::read_to_string(&legacy)?, "different state");
        assert_eq!(fs::read_to_string(&destination)?, "pending notes");
        Ok(())
    }

    #[test]
    fn fresh_chain_clears_all_nodes_but_keeps_control_and_identity_files() -> Result<()> {
        let dir = tempfile::tempdir()?;
        for id in 0..crate::identity::MAX_NUM_NODES {
            let node_dir = dir.path().join(format!("db_{id}"));
            fs::create_dir_all(node_dir.join("node_db"))?;
            for file in [
                "node_db/state",
                "curvy-pix-address.redb",
                "pix-recovery.redb",
            ] {
                fs::write(node_dir.join(file), "old chain")?;
            }
        }
        for file in [
            "cluster.lock",
            "data.lock",
            "node_id_0.id",
            "hoprd_cfg_0.yaml",
        ] {
            fs::write(dir.path().join(file), "keep")?;
        }
        fs::create_dir(dir.path().join("logs"))?;
        fs::write(dir.path().join("logs/hoprd_0.log"), "keep")?;

        prepare_node_state(dir.path(), false)?;
        assert_eq!(
            fs::read_to_string(dir.path().join("db_0/curvy-pix-address.redb"))?,
            "old chain"
        );
        prepare_node_state(dir.path(), true)?;
        for id in 0..crate::identity::MAX_NUM_NODES {
            assert!(!dir.path().join(format!("db_{id}")).exists());
        }
        for file in [
            "cluster.lock",
            "data.lock",
            "node_id_0.id",
            "hoprd_cfg_0.yaml",
            "logs/hoprd_0.log",
        ] {
            assert_eq!(fs::read_to_string(dir.path().join(file))?, "keep");
        }
        // Also works on first launch or after an interrupted reset.
        prepare_node_state(dir.path(), true)
    }

    #[cfg(unix)]
    #[test]
    fn reset_does_not_follow_node_directory_symlinks() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let outside = tempfile::tempdir()?;
        fs::write(outside.path().join("state.redb"), "keep")?;
        std::os::unix::fs::symlink(outside.path(), dir.path().join("db_0"))?;

        prepare_node_state(dir.path(), true)?;

        assert!(!dir.path().join("db_0").exists());
        assert_eq!(
            fs::read_to_string(outside.path().join("state.redb"))?,
            "keep"
        );
        Ok(())
    }
}
