use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use tokio::time::sleep;

use crate::{DarkCoreClient, DarkRustError};

const BUILD_DARK_CORE_EXECUTABLE: &str = env!("DARKFACTORY_DARK_CORE_EXECUTABLE");
const BUILD_DARK_CORE_WORKDIR: &str = env!("DARKFACTORY_DARK_CORE_WORKDIR");

#[derive(Debug, Clone)]
pub struct DarkCoreLaunchConfig {
    pub tmux_session_name: String,
    pub executable_path: Option<PathBuf>,
    pub workdir: Option<PathBuf>,
    pub restart_existing_session: bool,
    pub wait_timeout: Duration,
    pub wait_interval: Duration,
}

impl Default for DarkCoreLaunchConfig {
    fn default() -> Self {
        Self {
            tmux_session_name: "dark-core".to_string(),
            executable_path: None,
            workdir: None,
            restart_existing_session: true,
            wait_timeout: Duration::from_secs(30),
            wait_interval: Duration::from_millis(350),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnsureDarkCoreState {
    AlreadyRunning,
    LaunchedTmux,
    RestartedTmux,
    WaitingForTmuxSession,
}

pub fn is_local_dark_core_url(base_url: &str) -> bool {
    let normalized = base_url.trim().to_ascii_lowercase();
    normalized.starts_with("http://localhost:")
        || normalized.starts_with("https://localhost:")
        || normalized.starts_with("http://127.0.0.1:")
        || normalized.starts_with("https://127.0.0.1:")
}

pub async fn ensure_dark_core_in_tmux_if_needed(
    base_url: &str,
    launch_config: DarkCoreLaunchConfig,
) -> Result<EnsureDarkCoreState, DarkRustError> {
    if !is_local_dark_core_url(base_url) {
        return Ok(EnsureDarkCoreState::AlreadyRunning);
    }

    ensure_tmux_available()?;

    let session_exists = tmux_session_exists(&launch_config.tmux_session_name)?;

    if check_dark_core_health(base_url).await {
        if session_exists && launch_config.restart_existing_session {
            // Continue so we recycle the managed tmux session.
        } else {
            return Ok(EnsureDarkCoreState::AlreadyRunning);
        }
    }

    let workdir = resolve_workdir(
        launch_config.workdir,
        launch_config.executable_path.as_ref(),
    )?;
    let executable_path =
        resolve_or_build_executable_path(launch_config.executable_path, &workdir)?;

    if session_exists {
        // Prefer a clean restart when a managed session already exists so each
        // boot attempt reuses a fresh dark_core process in tmux.
        kill_tmux_session(&launch_config.tmux_session_name)?;
    }

    launch_dark_core_tmux(
        &launch_config.tmux_session_name,
        &workdir,
        &executable_path,
        base_url,
    )?;

    let became_healthy = wait_for_dark_core_health(
        base_url,
        launch_config.wait_timeout,
        launch_config.wait_interval,
    )
    .await;

    if !became_healthy {
        let attach_command = format!(
            "tmux attach -t {}",
            shell_escape_single_quotes(&launch_config.tmux_session_name)
        );
        let tmux_tail = capture_tmux_tail(&launch_config.tmux_session_name, 40)
            .unwrap_or_else(|_| "<unable to capture tmux output>".to_string());
        return Err(DarkRustError::Runtime {
            message: format!(
                "dark_core did not become healthy in {:?} (baseUrl={base_url}, session={}). Inspect with: {attach_command}. Recent tmux output: {tmux_tail}",
                launch_config.wait_timeout, launch_config.tmux_session_name,
            ),
        });
    }

    Ok(if session_exists {
        EnsureDarkCoreState::RestartedTmux
    } else {
        EnsureDarkCoreState::LaunchedTmux
    })
}

async fn wait_for_dark_core_health(base_url: &str, timeout: Duration, interval: Duration) -> bool {
    let deadline = Instant::now() + timeout;

    while Instant::now() < deadline {
        if check_dark_core_health(base_url).await {
            return true;
        }

        sleep(interval).await;
    }

    false
}

async fn check_dark_core_health(base_url: &str) -> bool {
    let api = DarkCoreClient::new(base_url.to_string());
    match api.system_health().await {
        Ok(response) => (200..300).contains(&response.status),
        Err(_) => false,
    }
}

fn ensure_tmux_available() -> Result<(), DarkRustError> {
    let output =
        Command::new("tmux")
            .arg("-V")
            .output()
            .map_err(|error| DarkRustError::Runtime {
                message: format!("tmux is required but unavailable (error={error})"),
            })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(DarkRustError::Runtime {
            message: format!("tmux check failed (stderr={stderr})"),
        });
    }

    Ok(())
}

fn tmux_session_exists(session_name: &str) -> Result<bool, DarkRustError> {
    let output = Command::new("tmux")
        .args(["has-session", "-t", session_name])
        .output()
        .map_err(|error| DarkRustError::Runtime {
            message: format!("failed to query tmux session (session={session_name},error={error})"),
        })?;

    Ok(output.status.success())
}

fn launch_dark_core_tmux(
    session_name: &str,
    workdir: &PathBuf,
    executable_path: &PathBuf,
    base_url: &str,
) -> Result<(), DarkRustError> {
    let launch_command = format!(
        "cd '{}' && '{}'",
        shell_escape_single_quotes(&workdir.display().to_string()),
        shell_escape_single_quotes(&executable_path.display().to_string())
    );

    let output = Command::new("tmux")
        .args([
            "new-session",
            "-d",
            "-s",
            session_name,
            "/bin/sh",
            "-lc",
            &launch_command,
        ])
        .output()
        .map_err(|error| DarkRustError::Runtime {
            message: format!(
                "failed to create tmux session for dark_core (session={session_name},baseUrl={base_url},error={error})"
            ),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(DarkRustError::Runtime {
            message: format!(
                "failed to launch dark_core tmux session (session={session_name},stderr={stderr})"
            ),
        });
    }

    Ok(())
}

fn kill_tmux_session(session_name: &str) -> Result<(), DarkRustError> {
    let output = Command::new("tmux")
        .args(["kill-session", "-t", session_name])
        .output()
        .map_err(|error| DarkRustError::Runtime {
            message: format!("failed to kill tmux session (session={session_name},error={error})"),
        })?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(DarkRustError::Runtime {
        message: format!("failed to kill tmux session (session={session_name},stderr={stderr})"),
    })
}

fn capture_tmux_tail(session_name: &str, lines: usize) -> Result<String, DarkRustError> {
    let start = format!("-{}", lines.max(1));
    let output = Command::new("tmux")
        .args(["capture-pane", "-pt", session_name, "-S", &start])
        .output()
        .map_err(|error| DarkRustError::Runtime {
            message: format!(
                "failed to capture tmux pane output (session={session_name},error={error})"
            ),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(DarkRustError::Runtime {
            message: format!(
                "failed to capture tmux pane output (session={session_name},stderr={stderr})"
            ),
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn resolve_or_build_executable_path(
    explicit: Option<PathBuf>,
    workdir: &PathBuf,
) -> Result<PathBuf, DarkRustError> {
    let candidates = executable_candidates(explicit, workdir);

    if let Some(path) = candidates.iter().find(|path| path.exists()).cloned() {
        return Ok(path);
    }

    run_dark_core_build_exec(workdir)?;

    if let Some(path) = candidates.iter().find(|path| path.exists()).cloned() {
        return Ok(path);
    }

    let expected = workdir.join("darkcore");
    Err(DarkRustError::Runtime {
        message: format!(
            "dark_core executable missing after auto-build (expected={}). Set DARK_CORE_EXECUTABLE or build manually with `bun run build:exec` in dark_core",
            expected.display()
        ),
    })
}

fn resolve_workdir(
    explicit: Option<PathBuf>,
    explicit_executable: Option<&PathBuf>,
) -> Result<PathBuf, DarkRustError> {
    if let Some(path) = explicit {
        if path.exists() {
            return Ok(path);
        }
    }

    if let Some(env_path) = env::var_os("DARK_CORE_WORKDIR") {
        let path = PathBuf::from(env_path);
        if path.exists() {
            return Ok(path);
        }
    }

    let build_path = PathBuf::from(BUILD_DARK_CORE_WORKDIR);
    if build_path.exists() {
        return Ok(build_path);
    }

    if let Some(executable_path) = explicit_executable {
        if let Some(parent) = executable_path.parent() {
            return Ok(parent.to_path_buf());
        }
    }

    Err(DarkRustError::Runtime {
        message: "unable to resolve dark_core workdir; set DARK_CORE_WORKDIR".to_string(),
    })
}

fn shell_escape_single_quotes(value: &str) -> String {
    value.replace('\'', "'\\''")
}

fn executable_candidates(explicit: Option<PathBuf>, workdir: &PathBuf) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(path) = explicit {
        candidates.push(path);
    }

    if let Some(env_path) = env::var_os("DARK_CORE_EXECUTABLE") {
        candidates.push(PathBuf::from(env_path));
    }

    candidates.push(PathBuf::from(BUILD_DARK_CORE_EXECUTABLE));
    candidates.push(workdir.join("darkcore"));
    candidates
}

fn run_dark_core_build_exec(workdir: &PathBuf) -> Result<(), DarkRustError> {
    let output = Command::new("bun")
        .args(["run", "build:exec"])
        .current_dir(workdir)
        .output()
        .map_err(|error| DarkRustError::Runtime {
            message: format!(
                "failed to auto-build dark_core executable with bun run build:exec (workdir={},error={error})",
                workdir.display()
            ),
        })?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Err(DarkRustError::Runtime {
        message: format!(
            "auto-build failed for dark_core executable (workdir={},stdout={},stderr={})",
            workdir.display(),
            if stdout.is_empty() {
                "<empty>"
            } else {
                &stdout
            },
            if stderr.is_empty() {
                "<empty>"
            } else {
                &stderr
            }
        ),
    })
}
