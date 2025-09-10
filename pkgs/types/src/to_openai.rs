use anyhow::{Result, anyhow};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

use crate::simple::{EvalSpec, SimpleFieldType, TestSpec};
use xeval_openai::evals::*;

impl EvalSpec {
    pub fn to_openai_eval(&self) -> Result<OpenAiEval> {
        // Build item schema from declared fields
        let mut item_props: Map<String, Value> = Map::new();
        let mut required: Vec<String> = Vec::new();

        for (k, v) in &self.schema {
            item_props.insert(k.clone(), type_to_json_schema(v));
            required.push(k.clone());
        }

        // Collect extra item vars referenced by tests (e.g. {{answer}})
        let mut extra_item_keys: BTreeSet<String> = BTreeSet::new();
        for t in &self.tests {
            match t {
                TestSpec::String {
                    input,
                    eq,
                    ne,
                    like,
                    ilike,
                    ..
                } => {
                    collect_item_vars(input, &mut extra_item_keys);
                    if let Some(x) = eq {
                        collect_item_vars(x, &mut extra_item_keys);
                    }
                    if let Some(x) = ne {
                        collect_item_vars(x, &mut extra_item_keys);
                    }
                    if let Some(x) = like {
                        collect_item_vars(x, &mut extra_item_keys);
                    }
                    if let Some(x) = ilike {
                        collect_item_vars(x, &mut extra_item_keys);
                    }
                }
            }
        }

        for k in extra_item_keys {
            if !item_props.contains_key(&k) {
                item_props.insert(k.clone(), json!({"type": "string"}));
                required.push(k);
            }
        }

        let schema = json!({
            "properties": {
                "item": {
                    "properties": Value::Object(item_props.clone()),
                    "required": required,
                    "type": "object",
                },
                "sample": sample_schema(),
            },
            "required": ["item", "sample"],
            "type": "object",
        });

        let graders = self
            .tests
            .iter()
            .enumerate()
            .map(|(i, t)| string_test_to_grader(i, t))
            .collect::<Result<Vec<_>>>()?;

        let eval = OpenAiEval {
            object: OpenAiEvalObject,
            id: format!("local_eval_{}", slug_like(&self.name)),
            name: self.name.clone(),
            created_at: 0,
            metadata: None,
            data_source_config: OpenAiDataSourceConfig::Custom(OpenAiCustomDataSourceConfig {
                r#type: OpenAiCustomDataSourceConfigType,
                schema,
            }),
            testing_criteria: graders,
        };

        Ok(eval)
    }
}

fn slug_like(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn type_to_json_schema(t: &SimpleFieldType) -> Value {
    match t {
        SimpleFieldType::String => json!({"type": "string"}),
        SimpleFieldType::Number => json!({"type": "number"}),
        SimpleFieldType::Boolean => json!({"type": "boolean"}),
    }
}

fn collect_item_vars(template: &str, out: &mut BTreeSet<String>) {
    // Very light mustache: capture {{identifier}} that do not contain a dot
    // and are not prefixed with sample./response./item.
    static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\}\}").expect("regex")
    });
    for cap in RE.captures_iter(template) {
        let key = cap[1].to_string();
        if key != "sample" && key != "response" && key != "item" {
            out.insert(key);
        }
    }
}

fn translate_template_placeholders(s: &str) -> String {
    // Map tokens inside {{...}} to OpenAI expected paths
    // - {{response.text}} -> {{sample.output_text}}
    // - {{foo}} -> {{item.foo}}
    // - leave {{sample.*}} and {{item.*}} as-is
    static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z0-9_]+)?)\s*\}\}").expect("regex")
    });

    RE.replace_all(s, |caps: &regex::Captures| {
        let path = &caps[1];
        if path == "response.text" {
            "{{sample.output_text}}".to_string()
        } else if path.starts_with("sample.") || path.starts_with("item.") {
            format!("{{{{{}}}}}", path)
        } else if !path.contains('.') && path != "sample" && path != "item" && path != "response" {
            format!("{{{{item.{}}}}}", path)
        } else {
            // Fallback: keep unchanged
            format!("{{{{{}}}}}", path)
        }
    })
    .into_owned()
}

fn string_test_to_grader(index: usize, t: &TestSpec) -> Result<OpenAiGrader> {
    match t {
        TestSpec::String {
            name,
            input,
            eq,
            ne,
            like,
            ilike,
        } => {
            let name = name
                .clone()
                .unwrap_or_else(|| "String check grader".to_string());
            let input = translate_template_placeholders(input);
            let (operation, reference) = match (eq, ne, like, ilike) {
                (Some(r), None, None, None) => (OpenAiGraderStringCheckOperation::Eq, r.clone()),
                (None, Some(r), None, None) => (OpenAiGraderStringCheckOperation::Ne, r.clone()),
                (None, None, Some(r), None) => (OpenAiGraderStringCheckOperation::Like, r.clone()),
                (None, None, None, Some(r)) => (OpenAiGraderStringCheckOperation::Ilike, r.clone()),
                _ => {
                    return Err(anyhow!(
                        "string test requires exactly one of eq/ne/like/ilike"
                    ));
                }
            };

            Ok(OpenAiGrader::StringCheck(OpenAiGraderStringCheck {
                r#type: OpenAiGraderStringCheckType,
                name,
                operation,
                input,
                reference: translate_template_placeholders(&reference),
            }))
        }
    }
}

fn sample_schema() -> Value {
    // Mirrors the cached .xeval/openai/evals.json "sample" schema
    json!({
        "properties": {
            "choices": {
                "items": {
                    "properties": {
                        "finish_reason": {"type": "string"},
                        "message": {
                            "properties": {
                                "content": {"type": ["string", "array", "null"]},
                                "function_call": {
                                    "properties": {
                                        "arguments": {"type": "string"},
                                        "name": {"type": "string"}
                                    },
                                    "required": ["name", "arguments"],
                                    "type": ["object", "null"]
                                },
                                "refusal": {"type": ["boolean", "null"]},
                                "role": {"enum": ["assistant"], "type": "string"},
                                "tool_calls": {
                                    "items": {
                                        "properties": {
                                            "function": {
                                                "properties": {
                                                    "arguments": {"type": "string"},
                                                    "name": {"type": "string"}
                                                },
                                                "required": ["name", "arguments"],
                                                "type": "object"
                                            },
                                            "id": {"type": "string"},
                                            "type": {"enum": ["function"], "type": "string"}
                                        },
                                        "required": ["type", "function", "id"],
                                        "type": "object"
                                    },
                                    "type": ["array", "null"]
                                }
                            },
                            "required": ["role"],
                            "type": "object"
                        }
                    },
                    "required": ["index", "message", "finish_reason"],
                    "type": "object"
                },
                "type": "array"
            },
            "input_tools": {"items": {"type": "object"}, "type": "array"},
            "model": {"type": "string"},
            "output_audio": {"type": ["object", "null"]},
            "output_json": {"type": "object"},
            "output_reasoning_summary": {"type": ["string", "null"]},
            "output_text": {"type": "string"},
            "output_tools": {"items": {"type": "object"}, "type": "array"}
        },
        "required": ["model", "choices"],
        "type": "object"
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use serde_json::to_string_pretty;

    #[test]
    fn converts_simple_math_eval_to_openai_eval() {
        let yaml = r#"
name: math
schema:
  a: number
  b: number
  answer: number
tests:
  - type: string
    input: "{{response.text}}"
    eq: "{{answer}}"
"#;
        let spec: EvalSpec = serde_yaml::from_str(yaml).unwrap();
        let eval = spec.to_openai_eval().unwrap();
        let json = to_string_pretty(&eval).unwrap();
        assert_snapshot!("openai_eval_math", json);
    }
}
