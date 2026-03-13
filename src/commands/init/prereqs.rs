//! Prerequisite checking for `plit init`.
//!
//! Verifies that required external tools are available on PATH,
//! detects container environments, and determines sandbox mode.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};
use dialoguer::Confirm;

use super::config;
use crate::output;

pub struct Environment {
    pub sandbox_mode: String,
}

struct PrereqResult {
    name: &'static str,
    found: bool,
    version: String,
    install_hint: &'static str,
}

pub fn check_all() -> Result<Environment> {
    let container = detect_container();
    let in_container = container.is_some();

    if let Some(ref ctype) = container {
        output::status(&format!("  ✓ container detected ({})", ctype));
    }

    // Non-redis prereqs
    let checks = vec![
        check_binary(
            "python3",
            "python3",
            &["--version"],
            "Install Python 3: https://www.python.org/downloads/ or `sudo apt install python3`",
        ),
        check_binary(
            "python3-venv",
            "python3",
            &["-m", "venv", "-h"],
            "Install venv module: `sudo apt install python3-venv`",
        ),
        check_binary(
            "git",
            "git",
            &["--version"],
            "Install git: https://git-scm.com/downloads or `sudo apt install git`",
        ),
    ];

    // Handle redis/dragonfly separately
    let redis_result = check_redis_or_dragonfly();

    let mut any_missing = false;

    for result in &checks {
        if result.found {
            output::status(&format!("  ✓ {} ({})", result.name, result.version));
        } else {
            output::status(&format!("  ✗ {} — not found", result.name));
            any_missing = true;
        }
    }

    // Display redis result
    match &redis_result {
        Ok(result) => {
            if result.found {
                output::status(&format!("  ✓ {} ({})", result.name, result.version));
            } else {
                output::status(&format!("  ✗ {} — not found", result.name));
                any_missing = true;
            }
        }
        Err(e) => {
            output::status(&format!("  ✗ redis — error: {}", e));
            any_missing = true;
        }
    }

    if any_missing {
        output::status("");
        output::status("Missing prerequisites. Install instructions:");
        for result in &checks {
            if !result.found {
                output::status(&format!("  • {}: {}", result.name, result.install_hint));
            }
        }
        if let Ok(result) = &redis_result
            && !result.found
        {
            output::status(&format!("  • {}: {}", result.name, result.install_hint));
        }
        bail!("Missing required prerequisites. Install them and re-run `plit init`.");
    }

    // If redis_result was Err, we already set any_missing and bailed above won't reach here,
    // but handle it for safety
    if redis_result.is_err() {
        bail!("Failed to check redis prerequisite.");
    }

    let sandbox_mode = if in_container {
        output::status("  → sandbox mode: container");
        "container".to_string()
    } else if cfg!(target_os = "macos") {
        output::status("  ⚠ macOS detected — bubblewrap is not available on macOS");
        output::status("  → sandbox mode: container (code execution is not sandboxed)");
        "container".to_string()
    } else {
        detect_sandbox_mode()?
    };

    Ok(Environment { sandbox_mode })
}

/// Check for redis-server on PATH, then fall back to managed DragonflyDB binary,
/// and finally offer to download DragonflyDB if neither is found.
fn check_redis_or_dragonfly() -> Result<PrereqResult> {
    // 1. Try redis-server on PATH
    let redis = check_binary(
        "redis-server",
        "redis-server",
        &["--version"],
        "Install Redis: build from source (`make && make install PREFIX=~/.local`), \
         or use DragonflyDB, or `podman run -d -p 6379:6379 redis`",
    );
    if redis.found {
        return Ok(redis);
    }

    // 2. Check if managed DragonflyDB binary already exists
    let dragonfly_path = config::dragonfly_bin_path()?;
    if dragonfly_path.exists() {
        let version = get_dragonfly_version(&dragonfly_path);
        return Ok(PrereqResult {
            name: "redis-server",
            found: true,
            version: format!("DragonflyDB {}", version),
            install_hint: "",
        });
    }

    // 3. Offer to download DragonflyDB
    offer_dragonfly_download()
}

/// Run the dragonfly binary with --version and extract the version string.
fn get_dragonfly_version(path: &Path) -> String {
    match Command::new(path).arg("--version").output() {
        Ok(output) => {
            let raw = if output.stdout.is_empty() {
                String::from_utf8_lossy(&output.stderr).to_string()
            } else {
                String::from_utf8_lossy(&output.stdout).to_string()
            };
            raw.lines().next().unwrap_or("unknown").trim().to_string()
        }
        Err(_) => "unknown".to_string(),
    }
}

/// Detect architecture, query latest release, ask user, download + extract via curl+tar.
fn offer_dragonfly_download() -> Result<PrereqResult> {
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        other => bail!(
            "Unsupported architecture for DragonflyDB auto-download: {}. \
             Install redis-server manually.",
            other
        ),
    };

    let api_output = Command::new("curl")
        .args([
            "-sS",
            "https://api.github.com/repos/dragonflydb/dragonfly/releases/latest",
        ])
        .output()
        .context("Failed to run curl — is it installed?")?;

    if !api_output.status.success() {
        bail!(
            "Failed to query DragonflyDB releases: {}",
            String::from_utf8_lossy(&api_output.stderr)
        );
    }

    let release: serde_json::Value = serde_json::from_slice(&api_output.stdout)
        .context("Failed to parse GitHub release response")?;

    let version = release["tag_name"]
        .as_str()
        .context("Could not find tag_name in GitHub release response")?;

    output::status("");
    output::status(&format!(
        "  Redis not found. DragonflyDB {} is available for {}.",
        version, arch
    ));

    let consent = Confirm::new()
        .with_prompt(format!(
            "Download DragonflyDB {} for {}? (~18 MB)",
            version, arch
        ))
        .default(true)
        .interact()?;

    if !consent {
        return Ok(PrereqResult {
            name: "redis-server",
            found: false,
            version: String::new(),
            install_hint: "Install Redis: build from source (`make && make install PREFIX=~/.local`), \
                          or use DragonflyDB, or `podman run -d -p 6379:6379 redis`",
        });
    }

    let url = format!(
        "https://github.com/dragonflydb/dragonfly/releases/download/{}/dragonfly-{}.tar.gz",
        version, arch
    );

    let dragonfly_path = config::dragonfly_bin_path()?;
    let config_dir = config::config_dir()?;
    std::fs::create_dir_all(&config_dir)
        .with_context(|| format!("Failed to create directory: {}", config_dir.display()))?;

    let target_name = format!("dragonfly-{}", arch);

    output::status("  Downloading and extracting...");

    let status = Command::new("sh")
        .args([
            "-c",
            &format!(
                "curl -fSL '{}' | tar xz -C '{}' '{}'",
                url,
                config_dir.display(),
                target_name,
            ),
        ])
        .status()
        .context("Failed to run curl | tar — are curl and tar installed?")?;

    if !status.success() {
        bail!("Failed to download and extract DragonflyDB from {}", url);
    }

    let extracted_path = config_dir.join(&target_name);
    std::fs::rename(&extracted_path, &dragonfly_path).with_context(|| {
        format!(
            "Failed to rename {} to {}",
            extracted_path.display(),
            dragonfly_path.display()
        )
    })?;

    // Set executable permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dragonfly_path, std::fs::Permissions::from_mode(0o755))
            .with_context(|| {
                format!(
                    "Failed to set executable permissions on {}",
                    dragonfly_path.display()
                )
            })?;
    }

    let version_str = get_dragonfly_version(&dragonfly_path);

    output::status(&format!(
        "  Done. Installed to {}",
        dragonfly_path.display()
    ));

    Ok(PrereqResult {
        name: "redis-server",
        found: true,
        version: format!("DragonflyDB {}", version_str),
        install_hint: "",
    })
}

/// Determine sandbox mode when not in a known container.
///
/// Attempts a bwrap test run (`bwrap --ro-bind / / true`) to check if
/// bubblewrap actually works — not just whether the binary exists.
///
/// - Test succeeds → `"bwrap"`
/// - Binary exists but test fails → already inside a sandboxed env
///   (e.g. bwrap with Alpine rootfs, no CAP_SYS_ADMIN) → `"container"`
/// - Binary not found → show install instructions and bail
fn detect_sandbox_mode() -> Result<String> {
    match Command::new("bwrap")
        .args(["--ro-bind", "/", "/", "/bin/true"])
        .output()
    {
        Ok(out) if out.status.success() => {
            output::status("  ✓ bwrap (test run passed)");
            output::status("  → sandbox mode: bwrap");
            Ok("bwrap".to_string())
        }
        Ok(_) => {
            // Binary exists but can't create namespaces — already sandboxed
            output::status("  ✓ already sandboxed (bwrap test failed — lacking privileges)");
            output::status("  → sandbox mode: container");
            Ok("container".to_string())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            output::status("  ✗ bwrap — not found");
            output::status("");
            output::status("Bubblewrap (bwrap) is required for code sandboxing.");
            output::status("Install it and re-run `plit init`:");
            output::status("  • Ubuntu/Debian: sudo apt install bubblewrap");
            output::status("  • Fedora: sudo dnf install bubblewrap");
            output::status("  • Arch: sudo pacman -S bubblewrap");
            output::status("  • From source: https://github.com/containers/bubblewrap");
            bail!("bubblewrap (bwrap) is not installed.");
        }
        Err(e) => {
            bail!("Failed to check bwrap: {e}");
        }
    }
}

fn detect_container() -> Option<String> {
    if std::env::var("CODESPACES").ok().as_deref() == Some("true") {
        return Some("codespaces".to_string());
    }
    if std::env::var("GITPOD_WORKSPACE_ID").is_ok() {
        return Some("gitpod".to_string());
    }
    if Path::new("/.dockerenv").exists() {
        return Some("docker".to_string());
    }
    if std::env::var("container").ok().as_deref() == Some("podman") {
        return Some("podman".to_string());
    }
    if let Ok(cgroup) = std::fs::read_to_string("/proc/1/cgroup") {
        if cgroup.contains("docker") {
            return Some("docker".to_string());
        }
        if cgroup.contains("kubepods") {
            return Some("kubernetes".to_string());
        }
        if cgroup.contains("containerd") {
            return Some("containerd".to_string());
        }
    }
    None
}

/// Check if a binary is available and capture its version string.
fn check_binary(
    name: &'static str,
    cmd: &str,
    version_args: &[&str],
    install_hint: &'static str,
) -> PrereqResult {
    match Command::new(cmd).args(version_args).output() {
        Ok(output) if output.status.success() => {
            let raw = if output.stdout.is_empty() {
                String::from_utf8_lossy(&output.stderr).to_string()
            } else {
                String::from_utf8_lossy(&output.stdout).to_string()
            };
            let version = raw.lines().next().unwrap_or("unknown").trim().to_string();
            PrereqResult {
                name,
                found: true,
                version,
                install_hint,
            }
        }
        Ok(_) | Err(_) => PrereqResult {
            name,
            found: false,
            version: String::new(),
            install_hint,
        },
    }
}
