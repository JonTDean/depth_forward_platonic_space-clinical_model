use std::{
    env,
    path::{Path, PathBuf},
};

use dotenvy::Error as DotEnvError;
use thiserror::Error;

/// Outcome of loading an environment file for a specific namespace/profile.
#[derive(Debug, Clone)]
pub struct EnvLoadOutcome {
    pub namespace: String,
    pub profile: String,
    pub files: Vec<PathBuf>,
}

/// Attempt to load the `.env.<namespace>.<profile>` file for the current crate.
///
/// * `namespace` follows the directory structure (e.g., `app.web.api`).
/// * `profile` is resolved from `DFPS_ENV` / `APP_ENV`, defaulting to `dev`.
/// * `DFPS_ENV_FILE`, if set, overrides the filename entirely (relative to workspace).
pub fn load_env(namespace: &str) -> Result<EnvLoadOutcome, EnvLoadError> {
    let profile = env_profile();
    let workspace_root = workspace_root()?;
    let explicit_file = env::var("DFPS_ENV_FILE").ok();

    let mut loaded_files = Vec::new();
    if let Some(filename) = explicit_file {
        let resolved = resolve_relative(&workspace_root, &filename);
        load_file(&resolved)?;
        loaded_files.push(resolved);
    } else {
        let primary = workspace_root.join(format!(".env.{namespace}.{profile}"));
        if primary.exists() {
            load_file(&primary)?;
            loaded_files.push(primary);
        } else {
            // fall back to `.env.<namespace>.local`
            let fallback = workspace_root.join(format!(".env.{namespace}.local"));
            if fallback.exists() {
                load_file(&fallback)?;
                loaded_files.push(fallback);
            }
        }
    }

    Ok(EnvLoadOutcome {
        namespace: namespace.to_string(),
        profile,
        files: loaded_files,
    })
}

fn load_file(path: &Path) -> Result<(), EnvLoadError> {
    dotenvy::from_filename(path)
        .map(|_| ())
        .map_err(|source| EnvLoadError::DotEnv {
            path: path.to_path_buf(),
            source,
        })
}

fn env_profile() -> String {
    env::var("DFPS_ENV")
        .or_else(|_| env::var("APP_ENV"))
        .unwrap_or_else(|_| "dev".to_string())
}

fn workspace_root() -> Result<PathBuf, EnvLoadError> {
    if let Ok(root) = env::var("DFPS_WORKSPACE_ROOT") {
        let candidate = PathBuf::from(root);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    let mut dir = env::current_dir().map_err(EnvLoadError::CurrentDir)?;
    loop {
        if dir.join("Cargo.lock").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    Err(EnvLoadError::WorkspaceRootNotFound)
}

fn resolve_relative(root: &Path, filename: &str) -> PathBuf {
    let candidate = Path::new(filename);
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    }
}

#[derive(Debug, Error)]
pub enum EnvLoadError {
    #[error("failed to determine current directory: {0}")]
    CurrentDir(std::io::Error),
    #[error("workspace root (containing Cargo.lock) not found")]
    WorkspaceRootNotFound,
    #[error("failed to load dotenv file {path:?}: {source}")]
    DotEnv { path: PathBuf, source: DotEnvError },
}
