use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};

use super::init::config;
use super::init::config::DockerConfig;
use crate::output;

pub async fn run(dev: bool, foreground: bool) -> Result<()> {
    if config::is_docker_mode() {
        return run_docker(dev, foreground).await;
    }

    let pid_path = pid_file_path()?;
    if is_running(&pid_path) {
        bail!(
            "plit is already running (PID file: {}). Use `plit stop` first.",
            pid_path.display()
        );
    }

    let config_json = config::config_json_path()?;
    let env_path = config::dot_env_path()?;
    let pipelit_dir = config::pipelit_dir()?;
    let venv_dir = config::venv_dir()?;
    let platform_dir = pipelit_dir.join("platform");

    if !config_json.exists() {
        bail!(
            "Config not found at {}. Run `plit init` first.",
            config_json.display()
        );
    }
    if !platform_dir.exists() {
        bail!(
            "Pipelit not found at {}. Run `plit init` first.",
            platform_dir.display()
        );
    }

    let raw = std::fs::read_to_string(&config_json)
        .with_context(|| format!("Failed to read {}", config_json.display()))?;
    let cfg: serde_json::Value =
        serde_json::from_str(&raw).context("Failed to parse config.json")?;

    let pipelit_port = extract_pipelit_port(&cfg)?;
    let gateway_bin = find_gateway_binary()?;
    let venv_bin = venv_dir.join("bin");
    let honcho_bin = venv_bin.join("honcho");

    if !honcho_bin.exists() {
        bail!(
            "honcho not found at {}. Run `plit init` first.",
            honcho_bin.display()
        );
    }

    let procfile = generate_procfile(
        &gateway_bin,
        &config_json,
        &venv_bin,
        &env_path,
        pipelit_port,
        dev,
    );
    let procfile_path = config::config_dir()?.join("Procfile");
    std::fs::write(&procfile_path, &procfile)
        .with_context(|| format!("Failed to write {}", procfile_path.display()))?;

    output::status("Starting plit stack...");
    if dev {
        output::status("  (dev mode — frontend dev server included)");
    }

    if foreground {
        run_foreground(
            &honcho_bin,
            &procfile_path,
            &env_path,
            &platform_dir,
            &pid_path,
        )
        .await
    } else {
        run_background(
            &honcho_bin,
            &procfile_path,
            &env_path,
            &platform_dir,
            &pid_path,
        )
    }
}

async fn run_docker(dev: bool, foreground: bool) -> Result<()> {
    if dev || foreground {
        output::status("Note: --dev and --foreground flags are ignored in Docker mode.");
    }

    let docker_json_path = config::docker_json_path()?;
    let raw = std::fs::read_to_string(&docker_json_path)
        .with_context(|| format!("Failed to read {}", docker_json_path.display()))?;
    let cfg: DockerConfig = serde_json::from_str(&raw).context("Failed to parse docker.json")?;

    let running = Command::new("docker")
        .args(["ps", "-q", "-f", &format!("name={}", cfg.container_name)])
        .output()
        .context("Failed to run docker ps")?;

    if !running.stdout.is_empty() {
        bail!(
            "plit is already running (container '{}').",
            cfg.container_name
        );
    }

    let exists = Command::new("docker")
        .args(["ps", "-aq", "-f", &format!("name={}", cfg.container_name)])
        .output()
        .context("Failed to check for existing container")?;

    if !exists.stdout.is_empty() {
        output::status(&format!(
            "Starting existing container '{}'...",
            cfg.container_name
        ));
        let status = Command::new("docker")
            .args(["start", &cfg.container_name])
            .status()
            .context("Failed to start container")?;
        if !status.success() {
            bail!("Failed to start container '{}'", cfg.container_name);
        }
    } else {
        output::status("Starting plit container...");
        let mut args = vec![
            "run".to_string(),
            "-d".to_string(),
            "--name".to_string(),
            cfg.container_name.clone(),
            "--add-host".to_string(),
            "host.docker.internal:host-gateway".to_string(),
            "-p".to_string(),
            format!("{}:{}", cfg.gateway_port, cfg.gateway_port),
            "-p".to_string(),
            format!("{}:{}", cfg.pipelit_port, cfg.pipelit_port),
        ];

        for (key, val) in &cfg.env {
            args.push("-e".to_string());
            args.push(format!("{}={}", key, val));
        }

        args.push(cfg.image.clone());

        let status = Command::new("docker")
            .args(&args)
            .status()
            .context("Failed to run docker container")?;
        if !status.success() {
            bail!("Failed to start Docker container");
        }
    }

    output::status("Waiting for health check...");
    let health_url = format!("http://localhost:{}/health", cfg.gateway_port);
    let client = reqwest::Client::new();
    let mut healthy = false;

    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        if let Ok(resp) = client.get(&health_url).send().await
            && resp.status().is_success()
        {
            healthy = true;
            break;
        }
    }

    if healthy {
        output::status("plit started.");
        output::status(&format!("  Gateway: http://localhost:{}", cfg.gateway_port));
        output::status(&format!("  Pipelit: http://localhost:{}", cfg.pipelit_port));
    } else {
        output::status("plit container started but health check did not pass within 30s.");
        output::status("Check container logs: docker logs plit");
    }

    Ok(())
}

async fn run_foreground(
    honcho_bin: &PathBuf,
    procfile_path: &PathBuf,
    env_path: &PathBuf,
    platform_dir: &PathBuf,
    pid_path: &PathBuf,
) -> Result<()> {
    let mut child = tokio::process::Command::new(honcho_bin)
        .args(["-f"])
        .arg(procfile_path)
        .args(["-e"])
        .arg(env_path)
        .arg("start")
        .current_dir(platform_dir)
        .spawn()
        .context("Failed to start honcho")?;

    if let Some(pid) = child.id() {
        let _ = std::fs::write(pid_path, pid.to_string());
    }

    let status = child.wait().await?;

    let _ = std::fs::remove_file(pid_path);

    if !status.success() {
        if status.code().is_none() {
            return Ok(());
        }
        bail!("plit exited with {}", status);
    }

    Ok(())
}

#[cfg(unix)]
fn run_background(
    honcho_bin: &PathBuf,
    procfile_path: &PathBuf,
    env_path: &PathBuf,
    platform_dir: &PathBuf,
    pid_path: &PathBuf,
) -> Result<()> {
    use std::fs::OpenOptions;
    use std::os::unix::process::CommandExt;

    let log_path = config::log_path()?;
    let data_dir = config::data_dir()?;
    std::fs::create_dir_all(&data_dir)
        .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .with_context(|| format!("Failed to open log file: {}", log_path.display()))?;

    let log_file_err = log_file
        .try_clone()
        .context("Failed to clone log file handle")?;

    let child = unsafe {
        std::process::Command::new(honcho_bin)
            .args(["-f"])
            .arg(procfile_path)
            .args(["-e"])
            .arg(env_path)
            .arg("start")
            .current_dir(platform_dir)
            .stdout(log_file)
            .stderr(log_file_err)
            .pre_exec(|| {
                libc::setsid();
                Ok(())
            })
            .spawn()
            .context("Failed to start honcho")?
    };

    let pid = child.id();
    std::fs::write(pid_path, pid.to_string())
        .with_context(|| format!("Failed to write PID file: {}", pid_path.display()))?;

    output::status(&format!("plit started (PID {})", pid));
    output::status(&format!("Logs: {}", log_path.display()));

    Ok(())
}

#[cfg(not(unix))]
fn run_background(
    _honcho_bin: &PathBuf,
    _procfile_path: &PathBuf,
    _env_path: &PathBuf,
    _platform_dir: &PathBuf,
    _pid_path: &PathBuf,
) -> Result<()> {
    bail!("Background mode is only supported on Unix. Use --foreground instead.");
}

fn generate_procfile(
    gateway_bin: &std::path::Path,
    config_json: &std::path::Path,
    venv_bin: &std::path::Path,
    env_path: &std::path::Path,
    pipelit_port: u16,
    dev: bool,
) -> String {
    let gw = gateway_bin.display();
    let cfg = config_json.display();
    let uvicorn = venv_bin.join("uvicorn").display().to_string();
    let rq = venv_bin.join("rq").display().to_string();

    let mut lines = vec![
        format!("gateway: GATEWAY_CONFIG={cfg} {gw}"),
        format!("pipelit: {uvicorn} main:app --host 0.0.0.0 --port {pipelit_port}"),
        format!(
            "scheduler: {rq} worker --worker-class worker_class.PipelitWorker workflows --with-scheduler"
        ),
        format!("worker: {rq} worker-pool workflows -w worker_class.PipelitWorker -n 4"),
    ];

    if uses_managed_dragonfly(env_path)
        && let Ok(dragonfly_path) = config::dragonfly_bin_path()
    {
        lines.insert(
            0,
            format!(
                "redis: {} --logtostderr --port 6399",
                dragonfly_path.display()
            ),
        );
    }

    if dev {
        lines.push("frontend: npm --prefix frontend run dev".to_string());
    }

    lines.join("\n") + "\n"
}

fn find_gateway_binary() -> Result<PathBuf> {
    let exe = std::env::current_exe().context("Could not determine current executable path")?;
    if let Some(dir) = exe.parent() {
        let candidate = dir.join("plit-gw");
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Ok(PathBuf::from("plit-gw"))
}

fn extract_pipelit_port(cfg: &serde_json::Value) -> Result<u16> {
    let url = cfg["backends"]["pipelit"]["inbound_url"]
        .as_str()
        .context("Missing backends.pipelit.inbound_url in config")?;

    let parsed: url::Url = url.parse().context("Invalid pipelit inbound_url")?;
    parsed
        .port()
        .context("No port in pipelit inbound_url — expected http://localhost:PORT/...")
}

fn pid_file_path() -> Result<PathBuf> {
    Ok(config::data_dir()?.join("plit.pid"))
}

fn uses_managed_dragonfly(env_path: &std::path::Path) -> bool {
    let Ok(content) = std::fs::read_to_string(env_path) else {
        return false;
    };
    content
        .lines()
        .any(|line| line.starts_with("REDIS_URL=") && line.contains(":6399"))
}

fn is_running(pid_path: &std::path::Path) -> bool {
    let Ok(content) = std::fs::read_to_string(pid_path) else {
        return false;
    };
    let Ok(pid) = content.trim().parse::<u32>() else {
        return false;
    };

    #[cfg(unix)]
    {
        let ret = unsafe { libc::kill(pid as i32, 0) };
        if ret == 0 {
            return true;
        }
        let err = std::io::Error::last_os_error();
        err.raw_os_error() == Some(libc::EPERM)
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        false
    }
}
