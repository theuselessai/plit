use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;
use dialoguer::Confirm;

use super::init::config;
use crate::output;

pub fn run(force: bool) -> Result<()> {
    let config_dir = config::config_dir()?;
    let data_dir = config::data_dir()?;
    let pipelit_config_dir = dirs::config_dir()
        .map(|d| d.join("pipelit"))
        .unwrap_or_default();

    let mut warnings: Vec<String> = Vec::new();

    // --- Detect running processes ---
    let pid_path = data_dir.join("plit.pid");
    let pid_running = is_pid_running(&pid_path);

    let gateway_port = read_gateway_port(&config_dir);
    let pipelit_port = read_pipelit_port(&config_dir);

    let gateway_listening = gateway_port.is_some_and(is_port_listening);
    let pipelit_listening = pipelit_port.is_some_and(is_port_listening);

    if pid_running || gateway_listening || pipelit_listening {
        output::status("WARNING: Running processes detected:");
        if pid_running {
            output::status("  - plit stack is running (PID file exists)");
        }
        if gateway_listening {
            output::status(&format!(
                "  - Gateway is listening on port {}",
                gateway_port.unwrap()
            ));
        }
        if pipelit_listening {
            output::status(&format!(
                "  - Pipelit is listening on port {}",
                pipelit_port.unwrap()
            ));
        }
        output::status("");

        if !force {
            let stop_first = Confirm::new()
                .with_prompt("Stop running processes before uninstalling?")
                .default(true)
                .interact()?;

            if stop_first {
                output::status("Stopping plit stack...");
                if let Err(e) = super::stop::run() {
                    output::error(&format!("Failed to stop: {}", e));
                    warnings
                        .push("Could not stop all processes — they may still be running".into());
                }
                output::status("");
            } else {
                warnings.push("Processes were left running — ports may remain bound".into());
            }
        } else {
            output::status("Stopping plit stack (--force)...");
            let _ = super::stop::run();
            output::status("");
        }
    }

    // --- Detect standalone Pipelit ---
    if pipelit_config_dir.exists() && !pipelit_config_dir.join(".plit-managed").exists() {
        output::status("WARNING: ~/.config/pipelit/ was NOT installed by plit.");
        output::status("  This may be a standalone Pipelit installation.");
        output::status("  Removing it could destroy independently managed data.");
        output::status("");
        warnings.push("Pipelit config directory is not plit-managed".into());
    }

    // --- Collect directories to remove ---
    let mut dirs_to_remove: Vec<(&str, PathBuf)> = Vec::new();

    if config_dir.exists() {
        dirs_to_remove.push(("plit config", config_dir.clone()));
    }
    if data_dir.exists() {
        dirs_to_remove.push(("plit data", data_dir.clone()));
    }
    if pipelit_config_dir.exists() {
        dirs_to_remove.push(("pipelit config", pipelit_config_dir.clone()));
    }

    if dirs_to_remove.is_empty() {
        output::status("Nothing to remove — plit is not installed.");
        return Ok(());
    }

    // --- Data inventory ---
    output::status("This will remove:");
    for (label, path) in &dirs_to_remove {
        let size = dir_size(path);
        output::status(&format!(
            "  {} — {} ({})",
            label,
            path.display(),
            format_size(size)
        ));
    }
    output::status("");

    let db_path = data_dir.join("pipelit.db");
    let venv_path = config::venv_dir().unwrap_or_default();
    let pipelit_clone = config::pipelit_dir().unwrap_or_default();

    let mut inventory: Vec<String> = Vec::new();
    if db_path.exists() {
        inventory.push(format!(
            "SQLite database ({})",
            format_size(file_size(&db_path))
        ));
    }
    if config_dir.join("config.json").exists() {
        inventory.push("Gateway configuration (tokens, credentials, routes)".into());
    }
    if config_dir.join(".env").exists() {
        inventory.push("Pipelit environment (secrets, API keys)".into());
    }
    if pipelit_clone.exists() {
        inventory.push("Pipelit git clone".into());
    }
    if venv_path.exists() {
        inventory.push("Python virtualenv".into());
    }

    let skills_dir = pipelit_config_dir.join("workspaces");
    if skills_dir.exists() {
        let workspace_count = count_subdirs(&skills_dir);
        if workspace_count > 0 {
            inventory.push(format!(
                "{} workspace(s) (skills, agent config)",
                workspace_count
            ));
        }
    }

    if !inventory.is_empty() {
        output::status("Data that will be lost:");
        for item in &inventory {
            output::status(&format!("  - {}", item));
        }
        output::status("");
    }

    if !warnings.is_empty() {
        for w in &warnings {
            output::status(&format!("WARNING: {}", w));
        }
        output::status("");
    }

    // --- Confirm ---
    if !force {
        let confirm = Confirm::new()
            .with_prompt("Permanently delete all of the above?")
            .default(false)
            .interact()?;

        if !confirm {
            output::status("Cancelled.");
            return Ok(());
        }
    }

    // --- Remove ---
    for (label, path) in &dirs_to_remove {
        match std::fs::remove_dir_all(path) {
            Ok(()) => output::status(&format!("Removed {} ({})", label, path.display())),
            Err(e) => output::error(&format!("Failed to remove {}: {}", path.display(), e)),
        }
    }

    output::status("");
    output::status(
        "Done. Redis data was not touched — run `redis-cli FLUSHALL` if you want a full reset.",
    );

    Ok(())
}

fn is_pid_running(pid_path: &Path) -> bool {
    let Ok(content) = std::fs::read_to_string(pid_path) else {
        return false;
    };
    let Ok(pid) = content.trim().parse::<i32>() else {
        return false;
    };
    if pid <= 0 {
        return false;
    }

    #[cfg(unix)]
    {
        let ret = unsafe { libc::kill(pid, 0) };
        if ret == 0 {
            return true;
        }
        std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM)
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        false
    }
}

fn is_port_listening(port: u16) -> bool {
    TcpStream::connect_timeout(
        &format!("127.0.0.1:{}", port).parse().unwrap(),
        Duration::from_millis(200),
    )
    .is_ok()
}

fn read_gateway_port(config_dir: &Path) -> Option<u16> {
    let raw = std::fs::read_to_string(config_dir.join("config.json")).ok()?;
    let cfg: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let listen = cfg["gateway"]["listen"].as_str()?;
    listen.rsplit(':').next()?.parse().ok()
}

fn read_pipelit_port(config_dir: &Path) -> Option<u16> {
    let raw = std::fs::read_to_string(config_dir.join("config.json")).ok()?;
    let cfg: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let url = cfg["backends"]["pipelit"]["inbound_url"].as_str()?;
    let parsed: url::Url = url.parse().ok()?;
    parsed.port()
}

fn dir_size(path: &Path) -> u64 {
    walkdir(path)
}

fn walkdir(path: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let ft = entry.file_type();
            if let Ok(ft) = ft {
                if ft.is_file() {
                    total += entry.metadata().map(|m| m.len()).unwrap_or(0);
                } else if ft.is_dir() {
                    total += walkdir(&entry.path());
                }
            }
        }
    }
    total
}

fn file_size(path: &Path) -> u64 {
    std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} bytes", bytes)
    }
}

fn count_subdirs(path: &Path) -> usize {
    std::fs::read_dir(path)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                .count()
        })
        .unwrap_or(0)
}
