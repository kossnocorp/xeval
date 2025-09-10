use anyhow::{anyhow, Context, Result};
use globwalk::GlobWalkerBuilder;
use std::path::{Path, PathBuf};
use xeval_types::EvalSpec;

pub fn parse_eval_spec_from_str(yaml: &str) -> Result<EvalSpec> {
    let spec: EvalSpec = serde_yaml::from_str(yaml)?;
    Ok(spec)
}

pub fn parse_eval_spec_from_path(path: impl AsRef<Path>) -> Result<EvalSpec> {
    let text = std::fs::read_to_string(path)?;
    parse_eval_spec_from_str(&text)
}

pub fn find_eval_specs(glob: &str, root: impl AsRef<Path>) -> Result<Vec<(PathBuf, EvalSpec)>> {
    let root = root.as_ref();
    // Normalize common leading"./" and backslashes in patterns
    let normalized = normalize_glob(glob);
    let walker = GlobWalkerBuilder::from_patterns(root, &[normalized.as_str()])
        .case_insensitive(true)
        .build()
        .with_context(|| format!("Invalid glob pattern: {glob}"))?;
    let mut out = Vec::new();
    for entry in walker.filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
        let path = entry.path().to_path_buf();
        let spec = parse_eval_spec_from_path(&path)
            .with_context(|| format!("Failed to parse eval spec: {}", path.display()))?;
        out.push((path, spec));
    }
    Ok(out)
}

fn normalize_glob(glob: &str) -> String {
    let mut s = glob.replace('\\', "/");
    while let Some(stripped) = s.strip_prefix("./") {
        s = stripped.to_string();
    }
    if s.is_empty() {
        return glob.to_string();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn parse_math_yaml() {
        // Workspace file path (relative): tests/workspace/evals/math.yaml
        let yaml = include_str!("../../../tests/workspace/evals/math.yaml");
        let spec = parse_eval_spec_from_str(yaml).expect("parse");
        assert_yaml_snapshot!("math_yaml_spec", &spec);
    }
}
