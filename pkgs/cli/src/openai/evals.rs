use crate::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct OpenAiLocalEvalsFile {
    /// When this cache file was updated (unix seconds)
    updated_at: i64,
    /// The cached evals, expected to be ordered by `updated_at` desc.
    evals: Vec<OpenAiEval>,
}

pub struct OpenAiLocalEvals;

impl OpenAiLocalEvals {
    pub async fn sync(
        auth: &Auth,
        state: &OpenAiLocalProjectState<'_>,
        force: bool,
        project: Option<String>,
    ) -> Result<Vec<OpenAiEval>> {
        let mut cache: OpenAiLocalEvalsFile = state.evals.read_json_or_default()?;

        let now = now_unix();
        let is_stale = now - cache.updated_at > 5 * 60; // 5 minutes

        if !force && !is_stale {
            return Ok(cache.evals);
        }

        let newest_page = auth
            .openai
            .list_evals_page(&OpenAiListEvalsPageRequest {
                project: project.clone(),
                after: None,
                limit: Some(1),
                order: Some("desc".to_string()),
                order_by: Some("updated_at".to_string()),
            })
            .await
            .context("Failed to fetch latest eval page")?;

        let remote_latest = newest_page.data.get(0).cloned();

        let local_latest = cache.evals.get(0).cloned();
        let local_hash = local_latest.as_ref().map(hash_eval).unwrap_or(0);
        let remote_hash = remote_latest.as_ref().map(hash_eval).unwrap_or(0);

        if local_hash == remote_hash {
            return Ok(cache.evals);
        }

        let evals = auth
            .openai
            .list_all_evals(&OpenAiListAllEvalsRequest {
                project,
                order: Some("desc".to_string()),
                order_by: Some("updated_at".to_string()),
            })
            .await
            .context("Failed to fetch all evals")?;

        cache.updated_at = now_unix();
        cache.evals = evals.clone();

        state.evals.write_json(&cache)?;

        Ok(evals)
    }
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn hash_eval(eval: &OpenAiEval) -> u64 {
    let mut h = DefaultHasher::new();
    eval.hash(&mut h);
    h.finish()
}
