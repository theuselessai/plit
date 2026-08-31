use anyhow::{Context, Result};
use clap::Subcommand;
use pipelit_client::apis::{configuration::Configuration, workflows_api};
use pipelit_client::models::ValidateDslIn;

use super::auth::pipelit_config;

#[derive(Subcommand)]
pub enum ApiCommands {
    #[command(subcommand)]
    Workflow(WorkflowCommands),

    NodeTypes,
}

#[derive(Subcommand)]
pub enum WorkflowCommands {
    List {
        #[arg(long, default_value = "50")]
        limit: i32,
    },

    Get {
        slug: String,
    },

    Validate {
        #[arg(long)]
        yaml: String,
    },

    Delete {
        slug: String,

        #[arg(long)]
        force: bool,
    },
}

pub async fn run(cmd: ApiCommands, json_output: bool) -> Result<()> {
    let config = pipelit_config()?;

    match cmd {
        ApiCommands::Workflow(wf_cmd) => run_workflow(wf_cmd, &config, json_output).await,
        ApiCommands::NodeTypes => run_node_types(&config, json_output).await,
    }
}

async fn run_workflow(
    cmd: WorkflowCommands,
    config: &Configuration,
    json_output: bool,
) -> Result<()> {
    match cmd {
        WorkflowCommands::List { limit } => {
            let result =
                workflows_api::list_workflows_api_v1_workflows_get(config, Some(limit), Some(0))
                    .await
                    .context("Failed to list workflows")?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                if let Some(arr) = result.as_array() {
                    println!("{:<8} {:<30} {:<20}", "ID", "NAME", "SLUG");
                    println!("{}", "-".repeat(60));
                    for wf in arr {
                        println!(
                            "{:<8} {:<30} {:<20}",
                            wf["id"].as_i64().unwrap_or(0),
                            truncate(wf["name"].as_str().unwrap_or("-"), 28),
                            wf["slug"].as_str().unwrap_or("-"),
                        );
                    }
                    println!("\nTotal: {} workflows", arr.len());
                }
            }
            Ok(())
        }

        WorkflowCommands::Get { slug } => {
            let result =
                workflows_api::get_workflow_detail_api_v1_workflows_slug_get(config, &slug)
                    .await
                    .context("Failed to get workflow")?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("Workflow: {}", result.name);
                println!("Slug: {}", result.slug);
                println!("ID: {}", result.id);
                println!("Description: {}", result.description);
                if let Some(Some(tags)) = &result.tags {
                    println!("Tags: {}", tags.join(", "));
                }
                if let Some(nodes) = &result.nodes {
                    println!("\nNodes: {}", nodes.len());
                    for node in nodes {
                        println!("  - {} ({:?})", node.node_id, node.component_type);
                    }
                }
                if let Some(edges) = &result.edges {
                    println!("\nEdges: {}", edges.len());
                }
            }
            Ok(())
        }

        WorkflowCommands::Validate { yaml } => {
            let yaml_content = std::fs::read_to_string(&yaml)
                .with_context(|| format!("Failed to read {}", yaml))?;

            let req = ValidateDslIn::new(yaml_content);

            let result = workflows_api::validate_dsl_endpoint_api_v1_workflows_validate_dsl_post(
                config, req,
            )
            .await
            .context("Failed to validate DSL")?;

            if json_output {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                let valid = result["valid"].as_bool().unwrap_or(false);
                if valid {
                    println!("Valid DSL");
                    println!("  Nodes: {}", result["node_count"].as_i64().unwrap_or(0));
                    println!("  Edges: {}", result["edge_count"].as_i64().unwrap_or(0));
                } else {
                    println!("Invalid DSL");
                    if let Some(errors) = result["errors"].as_array() {
                        for err in errors {
                            println!("  - {}", err.as_str().unwrap_or("?"));
                        }
                    }
                }
            }
            Ok(())
        }

        WorkflowCommands::Delete { slug, force } => {
            if !force {
                eprint!("Delete workflow '{}'? [y/N] ", slug);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled");
                    return Ok(());
                }
            }

            workflows_api::delete_workflow_api_v1_workflows_slug_delete(config, &slug)
                .await
                .context("Failed to delete workflow")?;

            println!("Deleted workflow: {}", slug);
            Ok(())
        }
    }
}

async fn run_node_types(config: &Configuration, json_output: bool) -> Result<()> {
    let result = workflows_api::list_node_types_api_v1_workflows_node_types_get(config)
        .await
        .context("Failed to list node types")?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        if let Some(obj) = result.as_object() {
            println!("{:<25} {:<15} DESCRIPTION", "TYPE", "CATEGORY");
            println!("{}", "-".repeat(80));
            for (type_name, spec) in obj {
                println!(
                    "{:<25} {:<15} {}",
                    type_name,
                    spec["category"].as_str().unwrap_or("-"),
                    truncate(spec["description"].as_str().unwrap_or("-"), 40),
                );
            }
            println!("\nTotal: {} node types", obj.len());
        }
    }
    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
