//! Default data-directory resolution.
//!
//! Resolution order, in priority:
//! 1. `--data-dir` flag value (handled by callers, validated here).
//! 2. `MCPMUX_DATA_DIR` environment variable (operator override).
//! 3. `$XDG_STATE_HOME/mcpmux` (Linux/macOS standard).
//! 4. `~/.local/state/mcpmux` (XDG default when `XDG_STATE_HOME` is unset).
//! 5. Platform fallback (`~/Library/Application Support/mcpmux` on macOS,
//!    `%LOCALAPPDATA%/mcpmux` on Windows, `~/.local/state/mcpmux` elsewhere).

use std::path::{Path, PathBuf};

use crate::error::RuntimeError;

/// Directory name under the XDG / platform data root.
pub const DATA_DIR_NAME: &str = "mcpmux";

/// Resolve the default data directory using the XDG / platform conventions.
///
/// Does NOT touch the filesystem. Callers pass the result to
/// `RuntimeBuilder::with_data_dir` (which validates and creates the
/// directory) or use it directly to discover an existing install.
pub fn default_data_dir() -> PathBuf {
    if let Some(env) = std::env::var_os("MCPMUX_DATA_DIR") {
        if !env.is_empty() {
            return PathBuf::from(env);
        }
    }

    if let Some(state_home) = std::env::var_os("XDG_STATE_HOME") {
        if !state_home.is_empty() {
            return PathBuf::from(state_home).join(DATA_DIR_NAME);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            return home.join("Library/Application Support").join(DATA_DIR_NAME);
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(local) = dirs::data_local_dir() {
            return local.join(DATA_DIR_NAME);
        }
    }

    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local/state")
        .join(DATA_DIR_NAME)
}

/// Resolve and validate the data directory for the runtime.
///
/// `override_path` (typically the `--data-dir` flag) wins when `Some`.
/// Otherwise the XDG default is used. The path is converted to an absolute
/// canonical form (resolving `..`, symlinks where the OS allows) so the
/// lockfile path is stable regardless of how the operator launched us.
pub fn resolve_data_dir(override_path: Option<&Path>) -> Result<PathBuf, RuntimeError> {
    let raw = match override_path {
        Some(p) => p.to_path_buf(),
        None => default_data_dir(),
    };

    if raw.as_os_str().is_empty() {
        return Err(RuntimeError::InvalidDataDir {
            path: raw,
            reason: "empty path".to_string(),
        });
    }

    let abs = if raw.is_absolute() {
        raw.clone()
    } else {
        std::env::current_dir()
            .map_err(|e| RuntimeError::InvalidDataDir {
                path: raw.clone(),
                reason: format!("could not resolve relative path: {}", e),
            })?
            .join(&raw)
    };

    let canonical = abs.canonicalize().unwrap_or_else(|_| abs.clone());

    Ok(canonical)
}

/// Directory holding the daemon's control socket: `$XDG_RUNTIME_DIR/mcpmux`.
///
/// `XDG_RUNTIME_DIR` is the correct home for a user-scoped, non-persistent
/// socket — it is created by systemd-logind with `0700` permissions and is
/// cleared on logout. When it is unset (unusual on a systemd host, possible
/// in a bare SSH session), fall back to `<data_dir>/run` so the daemon and
/// CLI still agree on a single path.
pub fn control_dir(data_dir: &Path) -> PathBuf {
    match std::env::var_os("XDG_RUNTIME_DIR") {
        Some(rt) if !rt.is_empty() => PathBuf::from(rt).join(DATA_DIR_NAME),
        _ => data_dir.join("run"),
    }
}

/// Path to the daemon control socket: `<control_dir>/control.sock`.
pub fn control_socket_path(data_dir: &Path) -> PathBuf {
    control_dir(data_dir).join("control.sock")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    fn environment_lock() -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn restore_environment_variable(key: &str, value: Option<std::ffi::OsString>) {
        unsafe {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }

    #[test]
    fn default_is_under_xdg_state_home_or_home() {
        let _environment = environment_lock();
        let dir = default_data_dir();
        assert!(dir.ends_with(DATA_DIR_NAME));
    }

    #[test]
    fn xdg_state_home_takes_precedence() {
        let _environment = environment_lock();
        let previous = std::env::var_os("XDG_STATE_HOME");
        unsafe {
            std::env::set_var("XDG_STATE_HOME", "/custom/state");
        }
        let dir = default_data_dir();
        assert_eq!(dir, PathBuf::from("/custom/state").join(DATA_DIR_NAME));
        restore_environment_variable("XDG_STATE_HOME", previous);
    }

    #[test]
    fn env_override_wins_over_xdg() {
        let _environment = environment_lock();
        let previous_data_dir = std::env::var_os("MCPMUX_DATA_DIR");
        let previous_state_home = std::env::var_os("XDG_STATE_HOME");
        unsafe {
            std::env::set_var("MCPMUX_DATA_DIR", "/explicit/override");
            std::env::set_var("XDG_STATE_HOME", "/should/be/ignored");
        }
        let dir = default_data_dir();
        assert_eq!(dir, PathBuf::from("/explicit/override"));
        restore_environment_variable("MCPMUX_DATA_DIR", previous_data_dir);
        restore_environment_variable("XDG_STATE_HOME", previous_state_home);
    }

    #[test]
    fn empty_override_path_is_rejected() {
        let _environment = environment_lock();
        let err = resolve_data_dir(Some(Path::new(""))).unwrap_err();
        assert!(matches!(err, RuntimeError::InvalidDataDir { .. }));
    }

    #[test]
    fn control_socket_prefers_xdg_runtime_dir() {
        let _environment = environment_lock();
        let previous = std::env::var_os("XDG_RUNTIME_DIR");
        unsafe {
            std::env::set_var("XDG_RUNTIME_DIR", "/run/user/1000");
        }
        let data_dir = Path::new("/var/lib/mcpmux");
        assert_eq!(
            control_socket_path(data_dir),
            PathBuf::from("/run/user/1000/mcpmux/control.sock")
        );
        restore_environment_variable("XDG_RUNTIME_DIR", previous);
    }

    #[test]
    fn control_socket_falls_back_under_data_dir() {
        let _environment = environment_lock();
        let previous = std::env::var_os("XDG_RUNTIME_DIR");
        unsafe {
            std::env::remove_var("XDG_RUNTIME_DIR");
        }
        let data_dir = Path::new("/var/lib/mcpmux");
        assert_eq!(
            control_socket_path(data_dir),
            PathBuf::from("/var/lib/mcpmux/run/control.sock")
        );
        restore_environment_variable("XDG_RUNTIME_DIR", previous);
    }
}
