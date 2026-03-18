use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Subcommand;
use pipelit_client::apis::auth_api;
use pipelit_client::apis::configuration::Configuration;
use pipelit_client::apis::Error as ApiError;
use pipelit_client::models::{MfaLoginVerifyRequest, TokenRequest};
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

/// Build a pipelit-client Configuration from stored auth.
/// Used by `plit api` subcommands and anything that needs authenticated API access.
pub fn pipelit_config() -> Result<Configuration> {
    let auth = load_auth().context("Not logged in. Run `plit auth login` first.")?;
    Ok(Configuration {
        base_path: auth.pipelit_url,
        bearer_access_token: Some(auth.token),
        ..Configuration::default()
    })
}

fn anon_config(url: &str) -> Configuration {
    Configuration {
        base_path: url.trim_end_matches('/').to_string(),
        ..Configuration::default()
    }
}

fn authed_config(url: &str, token: &str) -> Configuration {
    Configuration {
        base_path: url.trim_end_matches('/').to_string(),
        bearer_access_token: Some(token.to_string()),
        ..Configuration::default()
    }
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
    let (key, username) = if let Some(token) = token_arg {
        let username = verify_token(url, &token).await?;
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

        let key = authenticate(url, &username, &password).await?;
        (key, username)
    };

    verify_token(url, &key).await?;

    save_auth(&AuthConfig {
        token: key,
        username: username.clone(),
        pipelit_url: url.to_string(),
    })?;

    output::status(&format!("Logged in as {username} at {url}"));
    Ok(())
}

async fn authenticate(url: &str, username: &str, password: &str) -> Result<String> {
    let config = anon_config(url);
    let req = TokenRequest::new(username.to_string(), password.to_string());

    let resp = auth_api::obtain_token_api_v1_auth_token_post(&config, req).await;

    match resp {
        Ok(token_resp) => {
            if token_resp.requires_mfa.unwrap_or(false) {
                return handle_mfa(url, username, &token_resp.key).await;
            }
            Ok(token_resp.key)
        }
        Err(ApiError::Reqwest(e)) if e.is_connect() => {
            anyhow::bail!("Cannot connect to {url}. Is the server running?")
        }
        Err(ApiError::ResponseError(e)) if e.status == reqwest::StatusCode::UNAUTHORIZED => {
            anyhow::bail!("Invalid credentials")
        }
        Err(ApiError::ResponseError(e)) => {
            anyhow::bail!("Login failed (HTTP {}): {}", e.status, e.content)
        }
        Err(e) => anyhow::bail!("Login failed: {e}"),
    }
}

async fn handle_mfa(url: &str, username: &str, _initial_key: &str) -> Result<String> {
    let code: String = dialoguer::Input::new()
        .with_prompt("MFA code")
        .interact_text()?;

    let config = anon_config(url);
    let req = MfaLoginVerifyRequest::new(username.to_string(), code);

    let resp =
        auth_api::mfa_login_verify_api_v1_auth_mfa_login_verify_post(&config, req).await;

    match resp {
        Ok(token_resp) => Ok(token_resp.key),
        Err(ApiError::ResponseError(e)) => {
            anyhow::bail!("MFA verification failed (HTTP {}): {}", e.status, e.content)
        }
        Err(e) => anyhow::bail!("MFA verification failed: {e}"),
    }
}

async fn verify_token(url: &str, token: &str) -> Result<String> {
    let config = authed_config(url, token);

    let resp = auth_api::me_api_v1_auth_me_get(&config).await;

    match resp {
        Ok(me) => Ok(me.username),
        Err(ApiError::Reqwest(e)) if e.is_connect() => {
            anyhow::bail!("Cannot connect to {url}. Is the server running?")
        }
        Err(ApiError::ResponseError(e)) if e.status == reqwest::StatusCode::UNAUTHORIZED => {
            anyhow::bail!("Token expired or invalid. Run `plit auth login` again.")
        }
        Err(ApiError::ResponseError(e)) => {
            anyhow::bail!(
                "Token verification failed (HTTP {}): {}",
                e.status,
                e.content
            )
        }
        Err(e) => anyhow::bail!("Token verification failed: {e}"),
    }
}

async fn status() -> Result<()> {
    let auth = match load_auth() {
        Ok(a) => a,
        Err(_) => {
            output::status("Not logged in.");
            return Ok(());
        }
    };

    let config = authed_config(&auth.pipelit_url, &auth.token);
    match auth_api::me_api_v1_auth_me_get(&config).await {
        Ok(_) => {
            output::status(&format!(
                "Logged in as {} at {}",
                auth.username, auth.pipelit_url
            ));
        }
        Err(ApiError::ResponseError(e)) if e.status == reqwest::StatusCode::UNAUTHORIZED => {
            output::status("Token expired or invalid. Run `plit auth login` again.");
        }
        Err(ApiError::Reqwest(e)) if e.is_connect() => {
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
