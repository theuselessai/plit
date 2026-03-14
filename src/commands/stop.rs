use std::process::Command;

use anyhow::{Context, Result, bail};

use super::init::config;
use super::init::config::DockerConfig;
use crate::output;

pub fn run() -> Result<()> {
    if config::is_docker_mode() {
        return stop_docker();
    }

    let pid_path = config::data_dir()?.join("plit.pid");

    if !pid_path.exists() {
        bail!(
            "plit is not running (no PID file at {})",
            pid_path.display()
        );
    }

    let content = std::fs::read_to_string(&pid_path)
        .with_context(|| format!("Failed to read {}", pid_path.display()))?;
    let pid: i32 = content
        .trim()
        .parse()
        .with_context(|| format!("Invalid PID in {}", pid_path.display()))?;

    if pid <= 0 {
        let _ = std::fs::remove_file(&pid_path);
        bail!("Invalid PID {} in {}", pid, pid_path.display());
    }

    output::status(&format!("Stopping plit (PID {})...", pid));

    #[cfg(unix)]
    {
        let ret = unsafe { libc::kill(-pid, libc::SIGTERM) };
        if ret != 0 {
            let err = std::io::Error::last_os_error();
            let _ = std::fs::remove_file(&pid_path);
            match err.raw_os_error() {
                Some(libc::EPERM) => {
                    bail!("Permission denied sending SIGTERM to process group {}", pid);
                }
                Some(libc::ESRCH) => {
                    output::status("Process already exited.");
                    return Ok(());
                }
                _ => {
                    bail!("Failed to stop process group {}: {}", pid, err);
                }
            }
        }

        let _ = std::fs::remove_file(&pid_path);
        output::status("Stopped.");
        Ok(())
    }

    #[cfg(not(unix))]
    {
        let _ = pid;
        bail!("plit stop is only supported on Unix systems");
    }
}

fn stop_docker() -> Result<()> {
    let docker_json_path = config::docker_json_path()?;
    let raw = std::fs::read_to_string(&docker_json_path)
        .with_context(|| format!("Failed to read {}", docker_json_path.display()))?;
    let cfg: DockerConfig = serde_json::from_str(&raw).context("Failed to parse docker.json")?;

    let exists = Command::new("docker")
        .args(["ps", "-aq", "-f", &format!("name={}", cfg.container_name)])
        .output()
        .context("Failed to check for container")?;

    if exists.stdout.is_empty() {
        bail!(
            "plit is not running (no container '{}' found)",
            cfg.container_name
        );
    }

    output::status(&format!("Stopping container '{}'...", cfg.container_name));
    let status = Command::new("docker")
        .args(["stop", &cfg.container_name])
        .status()
        .context("Failed to stop container")?;

    if !status.success() {
        bail!("Failed to stop container '{}'", cfg.container_name);
    }

    output::status("Stopped.");
    Ok(())
}
