use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvalSpec {
    pub name: String,
    #[serde(default, alias = "input")]
    pub schema: BTreeMap<String, SimpleFieldType>,
    pub tests: Vec<TestSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SimpleFieldType {
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TestSpec {
    String {
        #[serde(default)]
        name: Option<String>,
        input: String,
        #[serde(default)]
        eq: Option<String>,
        #[serde(default)]
        ne: Option<String>,
        #[serde(default)]
        like: Option<String>,
        #[serde(default)]
        ilike: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn parse_eval_spec_yaml_variants() {
        let yaml = r#"
name: math
input:
  a: number
  b: number
tests:
  - type: string
    input: "{{response.text}}"
    eq: "{{answer}}"
"#;
        let spec: EvalSpec = serde_yaml::from_str(yaml).expect("parse yaml");
        assert_yaml_snapshot!("spec_from_input_alias", &spec);

        let yaml2 = r#"
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
        let spec2: EvalSpec = serde_yaml::from_str(yaml2).expect("parse yaml2");
        assert_yaml_snapshot!("spec_from_schema", &spec2);
    }
}
