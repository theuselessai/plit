use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Subcommand;
use serde::{Deserialize, Serialize};

use super::init::config::config_dir;
use crate::output;

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Log in to a Pipelit instance
    Login {
        /// Pipelit backend URL (e.g. http://localhost:8000)
        #[arg(long)]
        url: String,

        /// Username (prompted interactively if omitted)
        #[arg(long)]
        username: Option<String>,

        /// Password (prompted interactively if omitted)
        #[arg(long)]
        password: Option<String>,

        /// Use an existing API token directly (skip login flow)
        #[arg(long)]
        token: Option<String>,
    },

    /// Show current auth status
    Status,

    /// Log out and clear stored credentials
    Logout,
}

#[derive(Serialize, Deserialize)]
struct AuthConfig {
    token: String,
    username: String,
    pipelit_url: String,
}

fn auth_json_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("auth.json"))
}

fn load_auth() -> Result<AuthConfig> {
    let path = auth_json_path()?;
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("Failed to parse {}", path.display()))
}

fn save_auth(config: &AuthConfig) -> Result<()> {
    let path = auth_json_path()?;
    let dir = path.parent().context("Invalid auth.json path")?;
    std::fs::create_dir_all(dir)
        .with_context(|| format!("Failed to create directory {}", dir.display()))?;

    let json = serde_json::to_string_pretty(config).context("Failed to serialize auth config")?;
    std::fs::write(&path, &json).with_context(|| format!("Failed to write {}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
            .with_context(|| format!("Failed to set permissions on {}", path.display()))?;
    }

    Ok(())
}

fn clear_auth() -> Result<()> {
    let path = auth_json_path()?;
    if path.exists() {
        std::fs::remove_file(&path)
            .with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

pub async fn run(cmd: AuthCommands) -> Result<()> {
    match cmd {
        AuthCommands::Login {
            url,
            username,
            password,
            token,
        } => login(&url, username, password, token).await,
        AuthCommands::Status => status().await,
        AuthCommands::Logout => logout(),
    }
}

async fn login(
    url: &str,
    username_arg: Option<String>,
    password_arg: Option<String>,
    token_arg: Option<String>,
) -> Result<()> {
    let client = reqwest::Client::new();

    let (key, username) = if let Some(token) = token_arg {
        let username = verify_token(&client, url, &token).await?;
        (token, username)
    } else {
        let username = match username_arg {
            Some(u) => u,
            None => dialoguer::Input::new()
                .with_prompt("Username")
                .interact_text()?,
        };

        let password = match password_arg {
            Some(p) => p,
            None => dialoguer::Password::new()
                .with_prompt("Password")
                .interact()?,
        };

        let key = authenticate(&client, url, &username, &password).await?;
        (key, username)
    };

    verify_token(&client, url, &key).await?;

    save_auth(&AuthConfig {
        token: key,
        username: username.clone(),
        pipelit_url: url.to_string(),
    })?;

    output::status(&format!("Logged in as {username} at {url}"));
    Ok(())
}

async fn authenticate(
    client: &reqwest::Client,
    url: &str,
    username: &str,
    password: &str,
) -> Result<String> {
    let resp = client
        .post(format!("{}/api/v1/auth/token/", url))
        .json(&serde_json::json!({"username": username, "password": password}))
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow::anyhow!("Cannot connect to {url}. Is the server running?")
            } else {
                anyhow::anyhow!("Request failed: {e}")
            }
        })?;

    let status = resp.status();
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("Invalid credentials");
    }
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Login failed (HTTP {status}): {body}");
    }

    let body: serde_json::Value = resp
        .json()
        .await
        .context("Failed to parse login response")?;
    let requires_mfa = body["requires_mfa"].as_bool().unwrap_or(false);
    let key = body["key"]
        .as_str()
        .context("Missing 'key' in login response")?
        .to_string();

    if requires_mfa {
        return handle_mfa(client, url, username, &key).await;
    }

    Ok(key)
}

async fn handle_mfa(
    client: &reqwest::Client,
    url: &str,
    username: &str,
    _initial_key: &str,
) -> Result<String> {
    let code: String = dialoguer::Input::new()
        .with_prompt("MFA code")
        .interact_text()?;

    let resp = client
        .post(format!("{}/api/v1/auth/mfa/login-verify/", url))
        .json(&serde_json::json!({"username": username, "code": code}))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("MFA verification request failed: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("MFA verification failed (HTTP {status}): {body}");
    }

    let body: serde_json::Value = resp.json().await.context("Failed to parse MFA response")?;
    body["key"]
        .as_str()
        .map(String::from)
        .context("Missing 'key' in MFA response")
}

async fn verify_token(client: &reqwest::Client, url: &str, token: &str) -> Result<String> {
    let resp = client
        .get(format!("{}/api/v1/auth/me/", url))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                anyhow::anyhow!("Cannot connect to {url}. Is the server running?")
            } else {
                anyhow::anyhow!("Request failed: {e}")
            }
        })?;

    let status = resp.status();
    if status == reqwest::StatusCode::UNAUTHORIZED {
        anyhow::bail!("Token expired or invalid. Run `plit auth login` again.");
    }
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Token verification failed (HTTP {status}): {body}");
    }

    let body: serde_json::Value = resp
        .json()
        .await
        .context("Failed to parse user info response")?;
    body["username"]
        .as_str()
        .map(String::from)
        .context("Missing 'username' in user info response")
}

async fn status() -> Result<()> {
    let auth = match load_auth() {
        Ok(a) => a,
        Err(_) => {
            output::status("Not logged in.");
            return Ok(());
        }
    };

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/v1/auth/me/", auth.pipelit_url))
        .header("Authorization", format!("Bearer {}", auth.token))
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            output::status(&format!(
                "Logged in as {} at {}",
                auth.username, auth.pipelit_url
            ));
        }
        Ok(r) if r.status() == reqwest::StatusCode::UNAUTHORIZED => {
            output::status("Token expired or invalid. Run `plit auth login` again.");
        }
        Ok(r) => {
            output::status(&format!(
                "Unexpected response (HTTP {}). Try `plit auth login` again.",
                r.status()
            ));
        }
        Err(e) if e.is_connect() => {
            output::status(&format!(
                "Cannot reach {}. Server may be down.",
                auth.pipelit_url
            ));
        }
        Err(e) => {
            output::status(&format!("Connection error: {e}"));
        }
    }

    Ok(())
}

fn logout() -> Result<()> {
    clear_auth()?;
    output::status("Logged out.");
    Ok(())
}
