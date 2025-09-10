use crate::prelude::*;
use serde_json::Value;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};

#[derive(clap::Args)]
pub struct EvalArgs {
    /// Watch for changes.
    #[arg(short, long, default_value_t = false)]
    watch: bool,
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Global(#[from] GlobalError),

    #[error(transparent)]
    OpenAiEvals(#[from] OpenAiEvalsError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub struct EvalCmd {}

impl EvalCmd {
    pub async fn run<'a>(cli: &'a Cli, args: &'a EvalArgs) -> Result<(), EvalError> {
        // Determine base path from --config (file or dir); fallback to CWD
        let base_path = match &cli.config {
            Some(p) if p.is_file() => p
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from(".")),
            Some(p) if p.is_dir() => p.clone(),
            _ => std::env::current_dir().context("Failed to get current directory")?,
        };
        let project = Project {
            path: base_path.clone(),
        };
        // Load config (honor --config strictly)
        let config = Config::find(&cli.config)?;
        let evals_glob = config.evals.clone();

        // Load local YAML specs
        let specs = xeval_files::find_eval_specs(&evals_glob, &project.path)?;
        if specs.is_empty() {
            UiMessage::warn(&format!(
                "No eval YAML files found for glob '{}' in {}",
                evals_glob,
                project.path.display()
            ));
            return Ok(());
        } else {
            UiMessage::info(&format!(
                "Found {} eval file(s) for glob '{}'",
                specs.len(),
                evals_glob
            ));
        }

        // Auth and fetch remote evals after we know we have local work
        let mut global = Global::resolve()?;
        let spinner = UiTheme::start_spinner("Logging in with OpenAI API...");
        let auth = Auth::ensure(&mut global, AuthState::Existing).await?;

        let state = OpenAiLocalProjectState::new(&project)?;

        let spinner = UiTheme::start_spinner("Syncing OpenAI evals");
        let remote_evals = OpenAiLocalEvals::sync(&auth, &state, false, None).await?;
        spinner.finish_and_clear();

        // Index remote evals by xeval_name metadata (prefer the latest by created_at)
        let mut by_name: HashMap<String, OpenAiEval> = HashMap::new();
        for eval in remote_evals {
            if let Some(md) = &eval.metadata {
                if let Some(name) = md.get("xeval_name") {
                    match by_name.get(name) {
                        Some(existing) => {
                            if eval.created_at > existing.created_at {
                                by_name.insert(name.clone(), eval);
                            }
                        }
                        None => {
                            by_name.insert(name.clone(), eval);
                        }
                    }
                }
            }
        }

        for (_path, spec) in specs {
            let mut local = spec.to_openai_eval()?;
            let local_hash = content_hash(&local);
            let mut md: BTreeMap<String, String> = local.metadata.take().unwrap_or_default();
            md.insert("xeval_name".into(), spec.name.clone());
            md.insert("xeval_hash".into(), local_hash.clone());
            local.metadata = Some(md.clone());

            if let Some(remote) = by_name.get(&spec.name) {
                let remote_hash = content_hash(remote);
                if remote_hash == local_hash {
                    let needs_metadata_update = remote
                        .metadata
                        .as_ref()
                        .and_then(|m| m.get("xeval_hash").cloned())
                        .unwrap_or_default()
                        != local_hash;

                    if needs_metadata_update {
                        let _ = auth
                            .openai
                            .update_eval_metadata(
                                project_header.as_deref(),
                                &remote.id,
                                Some(&spec.name),
                                Some(&md),
                            )
                            .await?;
                    }
                    UiMessage::info(&format!("Up-to-date: {}", spec.name));
                } else {
                    // Create a new eval (OpenAI API does not allow changing core fields)
                    let spinner =
                        UiTheme::start_spinner(&format!("Updating OpenAI eval: {}", spec.name));
                    let upsert = OpenAiEvalUpsert {
                        name: spec.name.clone(),
                        metadata: Some(md),
                        data_source_config: map_upsert_config(&local.data_source_config),
                        testing_criteria: local.testing_criteria.clone(),
                    };
                    let _created = auth
                        .openai
                        .create_eval(project_header.as_deref(), &upsert)
                        .await?;
                    spinner.finish_and_clear();
                    UiMessage::success(&format!("Updated eval: {}", spec.name));
                }
            } else {
                // Missing: create
                let upsert = OpenAiEvalUpsert {
                    name: spec.name.clone(),
                    metadata: Some(md),
                    data_source_config: map_upsert_config(&local.data_source_config),
                    testing_criteria: local.testing_criteria.clone(),
                };
                let _created = auth
                    .openai
                    .create_eval(project_header.as_deref(), &upsert)
                    .await?;
                UiMessage::success(&format!("Created eval: {}", spec.name));
            }
        }

        Ok(())
    }
}

fn content_hash(eval: &OpenAiEval) -> String {
    // Hash only content fields that matter for identity
    let v = json!({
        "name": eval.name,
        "data_source_config": eval.data_source_config,
        "testing_criteria": eval.testing_criteria,
    });
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_vec(&v).unwrap());
    format!("{:x}", hasher.finalize())
}

fn map_upsert_config(cfg: &OpenAiDataSourceConfig) -> OpenAiDataSourceConfigUpsert {
    match cfg {
        OpenAiDataSourceConfig::Custom(c) => {
            let item_schema = match c
                .schema
                .get("properties")
                .and_then(|p| p.get("item"))
                .cloned()
            {
                Some(s) => s,
                None => c.schema.clone(),
            };

            OpenAiDataSourceConfigUpsert::Custom(OpenAiCustomDataSourceConfigUpsert {
                r#type: OpenAiCustomDataSourceConfigType,
                item_schema,
                include_sample_schema: Some(true),
            })
        }

        OpenAiDataSourceConfig::Logs(c) => {
            OpenAiDataSourceConfigUpsert::Logs(OpenAiLogsDataSourceConfigUpsert {
                r#type: OpenAiLogsDataSourceConfigType,
                metadata: c.metadata.clone(),
            })
        }

        OpenAiDataSourceConfig::StoredCompletions(c) => {
            OpenAiDataSourceConfigUpsert::StoredCompletions(
                OpenAiStoredCompletionsDataSourceConfigUpsert {
                    r#type: OpenAiStoredCompletionsDataSourceConfigType,
                    metadata: c.metadata.clone(),
                },
            )
        }
    }
}
