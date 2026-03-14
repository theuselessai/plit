use std::collections::HashMap;
use std::process::Command;

use anyhow::{Result, bail};

use super::InitArgs;
use super::config::{self, DockerConfig};
use super::prompts;
use crate::output;

const IMAGE: &str = "ghcr.io/theuselessai/plit:latest";
const CONTAINER_NAME: &str = "plit";

pub async fn run(args: InitArgs) -> Result<()> {
    output::status("Non-Linux OS detected — using Docker mode.");
    output::status("");

    check_docker_installed()?;
    check_docker_running()?;

    output::status("Configuration:");
    let inputs = prompts::collect_docker(&args).await?;
    output::status("");

    output::status("Pulling Docker image...");
    let status = Command::new("docker")
        .args(["pull", IMAGE])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run docker pull: {}", e))?;
    if !status.success() {
        bail!("Failed to pull Docker image {}", IMAGE);
    }
    output::status("");

    let mut env = HashMap::new();
    env.insert("ADMIN_USERNAME".to_string(), inputs.admin_username.clone());
    env.insert("ADMIN_PASSWORD".to_string(), inputs.admin_password.clone());
    env.insert("LLM_PROVIDER".to_string(), inputs.llm_provider.clone());
    env.insert("LLM_API_KEY".to_string(), inputs.llm_api_key.clone());
    env.insert("LLM_MODEL".to_string(), inputs.llm_model.clone());
    env.insert("LLM_BASE_URL".to_string(), inputs.llm_base_url.clone());

    let docker_config = DockerConfig {
        container_name: CONTAINER_NAME.to_string(),
        image: IMAGE.to_string(),
        gateway_port: inputs.gateway_port,
        pipelit_port: inputs.pipelit_port,
        env,
    };

    let docker_json_path = config::docker_json_path()?;
    let cfg_dir = config::config_dir()?;
    std::fs::create_dir_all(&cfg_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create config directory: {}", e))?;

    let json = serde_json::to_string_pretty(&docker_config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize docker config: {}", e))?;
    std::fs::write(&docker_json_path, &json)
        .map_err(|e| anyhow::anyhow!("Failed to write {}: {}", docker_json_path.display(), e))?;

    output::status("Setup complete!");
    output::status("");
    output::status(&format!("  Config: {}", docker_json_path.display()));
    output::status("");
    output::status("Run `plit start` to launch the Docker container.");

    Ok(())
}

fn check_docker_installed() -> Result<()> {
    match Command::new("docker").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout);
            output::status(&format!(
                "  ✓ {}",
                version.lines().next().unwrap_or("docker").trim()
            ));
            Ok(())
        }
        _ => {
            output::error("Docker is not installed.");
            output::status("");
            if cfg!(target_os = "macos") {
                output::status(
                    "Install Docker Desktop: https://docs.docker.com/desktop/install/mac-install/",
                );
            } else if cfg!(target_os = "windows") {
                output::status(
                    "Install Docker Desktop: https://docs.docker.com/desktop/install/windows-install/",
                );
            } else {
                output::status("Install Docker: https://docs.docker.com/engine/install/");
            }
            bail!("Docker is required for Docker mode. Install it and re-run `plit init`.");
        }
    }
}

fn check_docker_running() -> Result<()> {
    let output = Command::new("docker")
        .arg("info")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match output {
        Ok(status) if status.success() => {
            output::status("  ✓ Docker daemon is running");
            Ok(())
        }
        _ => {
            bail!("Docker daemon is not running. Start Docker Desktop and try again.");
        }
    }
}
