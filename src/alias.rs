use std::sync::OnceLock;

use anyhow::Context;
use clap::ValueEnum;
use regex::{Regex, Replacer};

use crate::SupportedShells;

static PARAMETER_PATTERN: OnceLock<Regex> = OnceLock::new();
fn get_param_pattern() -> &'static Regex {
    PARAMETER_PATTERN
        .get_or_init(|| Regex::new(r"\$(\d+)").expect("Constructing PARAMETER_PATTERN"))
}

pub struct Alias {
    name: String,
    value: String,
    supported_shells: Option<Vec<SupportedShells>>,
}

impl TryFrom<&str> for Alias {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (name, value) = value
            .split_once('=')
            .context("No \"=\" in the input string")?;
        let (name, supported_shells) = name
            .split_once(':')
            .map(|(n, s)| (n, Some(s)))
            .unwrap_or((name, None));
        let supported_shells = if let Some(shells) = supported_shells {
            Some(
                shells
                    .split(',')
                    .map(|s| SupportedShells::from_str(s, true))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(anyhow::Error::msg)?,
            )
        } else {
            None
        };
        let mut value = value.to_string();
        if !value.contains('@') && get_param_pattern().find(&value).is_none() {
            value.push_str(" @");
        }
        Ok(Self {
            name: name.to_string(),
            value,
            supported_shells,
        })
    }
}

impl Alias {
    pub fn to_script(&self, lang: SupportedShells) -> Option<String> {
        if self
            .supported_shells
            .as_ref()
            .is_some_and(|l| !l.contains(&lang))
        {
            return None;
        }
        Some(match lang {
            SupportedShells::Pwsh => {
                format!(
                    "function {} {{\n    \
                        {}\n\
                    }}\n",
                    self.name,
                    get_param_pattern()
                        .replace_all(&self.value.replace('@', "@args"), "$$args[$1]")
                )
            }
            SupportedShells::Bash => {
                format!(
                    "function {} {{\n    \
                        {}\n\
                    }}\n",
                    self.name,
                    self.value.replace('@', "$@")
                )
            }
            SupportedShells::Fish => {
                format!(
                    "function {} \n    \
                        {}\n\
                    end\n",
                    self.name,
                    get_param_pattern()
                        .replace_all(&self.value.replace('@', "$argv"), FishParamReplacer {})
                )
            }
        })
    }
}

struct FishParamReplacer {}

impl Replacer for FishParamReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        if let Some(num) = caps.get(1).and_then(|s| s.as_str().parse::<usize>().ok()) {
            dst.push_str(format!("$argv[{}]", num + 1).as_str())
        }
    }
}
