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
        "kid": "pipelit-001"
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
port: 4000
listeners:
- name: llm
  policies:
    jwtAuth:
      mode: strict
      issuer: pipelit
      audiences: [agentgateway]
      jwks:
        file: \"{jwks_path}\"
      jwtValidationOptions:
        requiredClaims: []
  routes: []
",
        jwks_path = jwks_path.display()
    );
    let path = agw_dir.join("config.d/listeners/llm.yaml");
    std::fs::write(&path, &content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_mcp_listener_yaml(agw_dir: &Path) -> Result<()> {
    let content = "\
port: 3000
listeners:
- name: mcp
  routes:
  - policies:
      cors:
        allowOrigins: ['*']
        allowHeaders: [mcp-protocol-version, content-type, cache-control]
        exposeHeaders: [Mcp-Session-Id]
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
            let path = extract_path(&inputs.llm_base_url, "openai-compatible");
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

/// The agentgateway route name (`"<provider>-<model_slug>"`) that
/// `write_initial_provider` + `assemble-config.sh` produce for the initial
/// provider/model. plit init passes this to `apply-fixture --backend-route`
/// so pipelit records it on the default model node and its proxied LLM calls
/// hit the matching gateway route. Returns `None` for providers that create no
/// route. NOTE: the provider arm here must stay in sync with
/// `write_initial_provider` above.
pub fn initial_route_name(inputs: &UserInputs) -> Option<String> {
    let provider = match inputs.llm_provider.as_str() {
        "openai" => "openai".to_string(),
        "anthropic" => "anthropic".to_string(),
        "gemini" => "gemini".to_string(),
        "ollama" => "ollama".to_string(),
        "openai-compatible" => {
            if inputs.llm_base_url.is_empty() {
                return None;
            }
            provider_name_from_url(&inputs.llm_base_url)
        }
        _ => return None,
    };
    Some(format!("{}-{}", provider, model_slug(&inputs.llm_model)))
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

    // Emit backendTLS ONLY for TLS upstreams. agentgateway treats the mere
    // presence of the policy (even {}) as "speak TLS to the upstream", which
    // breaks plain-http backends (e.g. local Ollama or a LAN Qwen box) with
    // a TLS InvalidContentType error. Only an explicit http:// scheme
    // disables it; https:// (and scheme-less hosts) keep TLS on.
    let backend_tls = if host.starts_with("http://") {
        ""
    } else {
        "backendTLS: {}\n"
    };

    let content = format!(
        "\
provider:
  {provider_type}:
    model: \"\"
hostOverride: \"{hostname}\"
pathOverride: \"{path_override}\"
backendAuth:
  key:
    env: \"{env_var}\"
{backend_tls}",
        provider_type = provider_type,
        hostname = hostname,
        path_override = path_override,
        env_var = env_var,
        backend_tls = backend_tls,
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
///
/// The result is always a valid environment-variable identifier component
/// (lowercase `[a-z0-9_]`, never leading with a digit) so that IP-based and
/// local backends work — e.g. "http://192.168.0.73:8080/v1" -> "p_192_168_0_73",
/// "http://localhost:11434" -> "localhost". A bare octet like "0" used to leak
/// through and produce an invalid `0_API_KEY` env var, breaking gateway startup.
fn provider_name_from_url(url: &str) -> String {
    // Host without scheme, path, or port.
    let host = url
        .trim_end_matches('/')
        .split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("");

    // Real multi-label hostnames -> second-level domain (api.openai.com -> openai).
    // IPs and single-label hosts -> use the whole host (localhost, 192.168.0.73).
    let labels: Vec<&str> = host.split('.').filter(|s| !s.is_empty()).collect();
    let is_ip = !labels.is_empty()
        && host
            .split('.')
            .all(|l| !l.is_empty() && l.bytes().all(|b| b.is_ascii_digit()));
    let raw = if is_ip || labels.len() < 2 {
        host
    } else {
        labels[labels.len() - 2]
    };

    // Sanitize into a valid identifier component.
    let mut name: String = raw
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    if name.is_empty() {
        name = "custom".to_string();
    }
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        name = format!("p_{name}");
    }
    name
}

/// Extract host:port from a URL. Adds default port if missing (443 for https, 80 for http).
fn extract_host(url: &str) -> String {
    let is_https = url.starts_with("https://");
    let host_part = url
        .trim_end_matches('/')
        .split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or(url);
    if host_part.contains(':') {
        host_part.to_string()
    } else if is_https {
        format!("{host_part}:443")
    } else {
        format!("{host_part}:80")
    }
}

/// Extract path from a URL (e.g., "https://api.venice.ai/api/v1" -> "/api/v1/").
fn extract_path(url: &str, provider_type: &str) -> String {
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let path = after_scheme
        .find('/')
        .map(|i| &after_scheme[i..])
        .unwrap_or("/");
    let path = path.trim_end_matches('/');
    // Append endpoint suffix based on provider type
    match provider_type {
        "anthropic" => {
            if path.ends_with("/messages") {
                format!("{path}/")
            } else {
                format!("{path}/messages")
            }
        }
        _ => {
            // openai, openai-compatible, glm, gemini
            if path.ends_with("/chat/completions") {
                format!("{path}/")
            } else {
                format!("{path}/chat/completions")
            }
        }
    }
}
// cache bust 1775094575
