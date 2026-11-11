use glob::Pattern;
use serde::{Deserialize, Serialize};
use smart_filee_file_engine::is_illegal_name;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomRule {
    pub pattern: String,
    pub folder: String,
}

impl CustomRule {
    pub fn new(pattern: impl Into<String>, folder: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into().trim().to_lowercase(),
            folder: folder.into().trim().to_string(),
        }
    }
}

/// Validate and parse raw user rules. Invalid or dangerous entries are dropped.
pub fn parse_rules(raw: &[(String, String)]) -> Vec<CustomRule> {
    let mut rules = Vec::new();
    for (pat, folder) in raw {
        let p = pat.trim().to_lowercase();
        let f = folder.trim();

        if p.is_empty() || f.is_empty() || is_illegal_name(f) {
            continue;
        }

        rules.push(CustomRule {
            pattern: p,
            folder: f.to_string(),
        });
    }
    rules
}

/// Match a file name against custom rules in priority order.
/// If rule pattern has `*`, `?`, or `[`, uses glob pattern matching; otherwise uses substring search.
pub fn rule_folder(file_name: &str, rules: &[CustomRule]) -> Option<String> {
    let lower_name = file_name.to_lowercase();

    for rule in rules {
        if rule
            .pattern
            .chars()
            .any(|c| c == '*' || c == '?' || c == '[')
        {
            if let Ok(glob_pat) = Pattern::new(&rule.pattern) {
                if glob_pat.matches(&lower_name) {
                    return Some(rule.folder.clone());
                }
            }
        } else if lower_name.contains(&rule.pattern) {
            return Some(rule.folder.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules() {
        let raw = vec![
            ("*.psd".to_string(), "Design".to_string()),
            ("invoice".to_string(), "Money".to_string()),
            ("".to_string(), "Invalid".to_string()),
            ("bad".to_string(), "invalid/folder".to_string()),
            ("trailing".to_string(), "space.".to_string()),
        ];
        let rules = parse_rules(&raw);
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].pattern, "*.psd");
        assert_eq!(rules[0].folder, "Design");
        assert_eq!(rules[1].pattern, "invoice");
        assert_eq!(rules[1].folder, "Money");
    }

    #[test]
    fn test_rule_folder_matching() {
        let rules = vec![
            CustomRule::new("*.psd", "Design"),
            CustomRule::new("invoice", "Finances"),
        ];

        assert_eq!(
            rule_folder("banner.PSD", &rules),
            Some("Design".to_string())
        );
        assert_eq!(
            rule_folder("May_Invoice_2026.pdf", &rules),
            Some("Finances".to_string())
        );
        assert_eq!(rule_folder("unrelated.txt", &rules), None);
    }
}
