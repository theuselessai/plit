//! agentgateway bootstrap for `plit init`.
//!
//! Creates the agentgateway config directory structure, generates an ES256
//! keypair, writes config fragments, and shell scripts. The agentgateway
//! binary itself is NOT downloaded here — it is bundled in the Docker image
//! or downloaded separately for bare-metal installs.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use tokio::process::Command;

use super::config;
use super::prompts::UserInputs;
use crate::output;

/// Result of agentgateway bootstrap — returned to caller for .env writing.
pub struct AgentgatewaySetup {
    /// Absolute path to the agentgateway directory
    pub agw_dir: PathBuf,
    /// PEM-encoded ES256 private key (for JWT signing)
    pub jwt_private_key: String,
}

/// Top-level directory for agentgateway config.
pub fn agentgateway_dir() -> Result<PathBuf> {
    let base = dirs::config_dir().context("Could not determine config directory")?;
    Ok(base.join("agentgateway"))
}

/// Bootstrap agentgateway config structure + ES256 keypair.
pub async fn bootstrap(inputs: &UserInputs) -> Result<AgentgatewaySetup> {
    let agw_dir = agentgateway_dir()?;

    // Create directory structure
    let dirs_to_create = [
        "bin",
        "config.d/jwt",
        "config.d/listeners",
        "config.d/backends",
        "config.d/rules",
        "config.d/mcp_servers",
        "keys",
    ];
    for d in &dirs_to_create {
        let path = agw_dir.join(d);
        std::fs::create_dir_all(&path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    output::status("  * Created directory structure");

    // Generate ES256 keypair via Python (cryptography is in the venv)
    let (private_pem, jwks_json) = generate_es256_keypair().await?;
    output::status("  * Generated ES256 keypair");

    // Write JWKS public key
    let jwks_path = agw_dir.join("config.d/jwt/jwks.json");
    std::fs::write(&jwks_path, &jwks_json)
        .with_context(|| format!("Failed to write {}", jwks_path.display()))?;

    // Write config fragments
    write_base_yaml(&agw_dir)?;
    write_llm_listener_yaml(&agw_dir)?;
    write_mcp_listener_yaml(&agw_dir)?;
    write_rules(&agw_dir)?;
    output::status("  * Wrote config fragments");

    // Write shell scripts
    write_assemble_config_sh(&agw_dir)?;
    write_start_sh(&agw_dir)?;
    write_decrypt_keys_py(&agw_dir)?;
    output::status("  * Wrote shell scripts");

    // Write initial provider + model from LLM choice (if applicable)
    write_initial_provider(inputs, &agw_dir)?;

    Ok(AgentgatewaySetup {
        agw_dir,
        jwt_private_key: private_pem,
    })
}

/// Generate an ES256 (P-256) keypair using Python's cryptography library.
/// Returns (private_key_pem, jwks_json).
async fn generate_es256_keypair() -> Result<(String, String)> {
    let venv_dir = config::venv_dir()?;
    let python = venv_dir.join("bin").join("python");

    let script = r#"
import json
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.backends import default_backend
import base64

# Generate P-256 key
private_key = ec.generate_private_key(ec.SECP256R1(), default_backend())
public_key = private_key.public_key()

# PEM private key
private_pem = private_key.private_bytes(
    encoding=serialization.Encoding.PEM,
    format=serialization.PrivateFormat.PKCS8,
    encryption_algorithm=serialization.NoEncryption()
).decode()

# JWKS public key
public_numbers = public_key.public_numbers()
x_bytes = public_numbers.x.to_bytes(32, 'big')
y_bytes = public_numbers.y.to_bytes(32, 'big')

def b64url(b):
    return base64.urlsafe_b64encode(b).rstrip(b'=').decode()

jwks = {
    "keys": [{
        "kty": "EC",
        "crv": "P-256",
        "x": b64url(x_bytes),
        "y": b64url(y_bytes),
        "use": "sig",
        "alg": "ES256",
        "kid": "pipelit-1"
    }]
}

print(json.dumps({"private_pem": private_pem, "jwks": jwks}))
"#;

    let output = Command::new(&python)
        .args(["-c", script])
        .output()
        .await
        .context("Failed to run Python for ES256 keygen")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("ES256 keygen failed: {}", stderr);
    }

    let stdout = String::from_utf8(output.stdout).context("Invalid UTF-8 from keygen")?;
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).context("Failed to parse keygen output")?;

    let private_pem = parsed["private_pem"]
        .as_str()
        .context("Missing private_pem")?
        .to_string();
    let jwks = serde_json::to_string_pretty(&parsed["jwks"]).context("Failed to format JWKS")?;

    Ok((private_pem, jwks))
}

// --- Config fragment writers ---

fn write_base_yaml(agw_dir: &Path) -> Result<()> {
    let content = "\
config:
  adminAddr: \"0.0.0.0:15000\"
";
    let path = agw_dir.join("config.d/base.yaml");
    std::fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_llm_listener_yaml(agw_dir: &Path) -> Result<()> {
    // The JWKS path must be absolute. In Docker it will be under /root/.config/agentgateway/
    // For bare-metal it will be under ~/.config/agentgateway/
    let jwks_path = agw_dir.join("config.d/jwt/jwks.json");
    let content = format!(
        "\
listeners:
  - name: llm
    protocol: HTTP
    address: \"0.0.0.0:4000\"
    routes: []
    authentication:
      jwt:
        localJwks:
          filename: \"{jwks_path}\"
",
        jwks_path = jwks_path.display()
    );
    let path = agw_dir.join("config.d/listeners/llm.yaml");
    std::fs::write(&path, &content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_mcp_listener_yaml(agw_dir: &Path) -> Result<()> {
    let content = "\
listeners:
  - name: mcp
    protocol: HTTP
    address: \"0.0.0.0:3000\"
    routes:
      - name: mcp-route
        matches:
          - path:
              pathPrefix: /
        backends: []
";
    let path = agw_dir.join("config.d/listeners/mcp.yaml");
    std::fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_rules(agw_dir: &Path) -> Result<()> {
    let admin = "- 'jwt.role == \"admin\"'\n";
    let user = "- 'jwt.role == \"user\"'\n";

    let admin_path = agw_dir.join("config.d/rules/admin.yaml");
    std::fs::write(&admin_path, admin)
        .with_context(|| format!("Failed to write {}", admin_path.display()))?;

    let user_path = agw_dir.join("config.d/rules/user.yaml");
    std::fs::write(&user_path, user)
        .with_context(|| format!("Failed to write {}", user_path.display()))
}

// --- Shell script writers ---

fn write_assemble_config_sh(agw_dir: &Path) -> Result<()> {
    let content = include_str!("agentgateway_scripts/assemble-config.sh");
    let path = agw_dir.join("assemble-config.sh");
    std::fs::write(&path, content)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

fn write_start_sh(agw_dir: &Path) -> Result<()> {
    let content = include_str!("agentgateway_scripts/start.sh");
    let path = agw_dir.join("start.sh");
    std::fs::write(&path, content)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

fn write_decrypt_keys_py(agw_dir: &Path) -> Result<()> {
    let content = include_str!("agentgateway_scripts/decrypt_keys.py");
    let path = agw_dir.join("decrypt_keys.py");
    std::fs::write(&path, content)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

// --- Initial provider/model from LLM choice ---

fn write_initial_provider(inputs: &UserInputs, agw_dir: &Path) -> Result<()> {
    // Map LLM provider choice to agentgateway provider config
    let (provider_name, provider_type, host, path_override) = match inputs.llm_provider.as_str() {
        "openai" => ("openai", "openAI", "https://api.openai.com", "/v1/"),
        "anthropic" => (
            "anthropic",
            "anthropic",
            "https://api.anthropic.com",
            "/v1/",
        ),
        "gemini" => (
            "gemini",
            "gemini",
            "https://generativelanguage.googleapis.com",
            "/",
        ),
        "ollama" => {
            let base = if inputs.llm_base_url.is_empty() {
                "http://localhost:11434"
            } else {
                &inputs.llm_base_url
            };
            // Ollama doesn't use API keys, skip encrypted key
            write_provider_yaml(agw_dir, "ollama", "openAI", base, "/v1/")?;
            write_model_yaml(agw_dir, "ollama", &inputs.llm_model)?;
            output::status(&format!(
                "  * Created initial provider: ollama/{}",
                model_slug(&inputs.llm_model)
            ));
            return Ok(());
        }
        "openai-compatible" => {
            if inputs.llm_base_url.is_empty() {
                bail!("Base URL is required for openai-compatible providers");
            }
            // Derive a provider name from the base URL hostname
            let name = provider_name_from_url(&inputs.llm_base_url);
            let path = extract_path(&inputs.llm_base_url);
            write_provider_yaml(agw_dir, &name, "openAI", &inputs.llm_base_url, &path)?;
            write_model_yaml(agw_dir, &name, &inputs.llm_model)?;
            write_encrypted_key(agw_dir, &name, &inputs.llm_api_key)?;
            output::status(&format!(
                "  * Created initial provider: {}/{}",
                name,
                model_slug(&inputs.llm_model)
            ));
            return Ok(());
        }
        _ => return Ok(()), // Unknown provider, skip
    };

    write_provider_yaml(agw_dir, provider_name, provider_type, host, path_override)?;
    write_model_yaml(agw_dir, provider_name, &inputs.llm_model)?;
    if !inputs.llm_api_key.is_empty() {
        write_encrypted_key(agw_dir, provider_name, &inputs.llm_api_key)?;
    }
    output::status(&format!(
        "  * Created initial provider: {}/{}",
        provider_name,
        model_slug(&inputs.llm_model)
    ));

    Ok(())
}

fn write_provider_yaml(
    agw_dir: &Path,
    name: &str,
    provider_type: &str,
    host: &str,
    path_override: &str,
) -> Result<()> {
    let provider_dir = agw_dir.join("config.d/backends").join(name);
    std::fs::create_dir_all(&provider_dir)?;

    // Build env var name from provider name
    let env_var = format!("{}_API_KEY", name.to_uppercase().replace('-', "_"));

    // Extract just the hostname for hostOverride and SNI
    let hostname = extract_host(host);

    let content = format!(
        "\
provider:
  {provider_type}:
    model: \"\"
hostOverride: \"{hostname}\"
pathOverride: \"{path_override}\"
backendAuth:
  apiKey:
    envKey: \"{env_var}\"
backendTLS:
  sni: \"{hostname}\"
",
        provider_type = provider_type,
        hostname = hostname,
        path_override = path_override,
        env_var = env_var,
    );

    let path = provider_dir.join("_provider.yaml");
    std::fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_model_yaml(agw_dir: &Path, provider_name: &str, model_name: &str) -> Result<()> {
    let slug = model_slug(model_name);
    let provider_dir = agw_dir.join("config.d/backends").join(provider_name);
    std::fs::create_dir_all(&provider_dir)?;

    let content = format!("model: \"{}\"\n", model_name);
    let path = provider_dir.join(format!("{}.yaml", slug));
    std::fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_encrypted_key(agw_dir: &Path, provider_name: &str, api_key: &str) -> Result<()> {
    if api_key.is_empty() {
        return Ok(());
    }
    // Write the key as plaintext for now — Pipelit's provider API will
    // encrypt it with Fernet when providers are managed via the UI.
    // For init bootstrap, plaintext is fine since start.sh handles both.
    let path = agw_dir.join("keys").join(format!("{}.key", provider_name));
    std::fs::write(&path, api_key)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

/// Derive a model slug from a model name (e.g., "zai-org-glm-4.7" -> "glm-4.7").
/// Keeps the full name but replaces characters unsafe for filenames.
fn model_slug(model_name: &str) -> String {
    model_name.replace(['/', ':', ' '], "-")
}

/// Derive a provider name from a base URL (e.g., "https://api.venice.ai/api/v1" -> "venice").
fn provider_name_from_url(url: &str) -> String {
    url.trim_end_matches('/')
        .split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or("custom")
        .split('.')
        .rev()
        .nth(1) // second-level domain
        .unwrap_or("custom")
        .to_lowercase()
        .replace('-', "_")
}

/// Extract hostname from a URL for SNI.
fn extract_host(url: &str) -> String {
    url.trim_end_matches('/')
        .split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or(url)
        .to_string()
}

/// Extract path from a URL (e.g., "https://api.venice.ai/api/v1" -> "/api/v1/").
fn extract_path(url: &str) -> String {
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let path = after_scheme
        .find('/')
        .map(|i| &after_scheme[i..])
        .unwrap_or("/");
    let path = path.trim_end_matches('/');
    format!("{}/", path)
}
