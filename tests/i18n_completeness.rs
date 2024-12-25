use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[test]
fn test_i18n_completeness() {
    // 1. Collect all i18n keys from source files
    let source_keys = collect_source_keys();

    // 2. Collect all translation keys from i18n files
    let translation_files = collect_translation_files();

    // 3. Compare and report
    let mut has_errors = false;

    // Check for missing translations in each language file
    for (lang, keys) in &translation_files {
        let missing: Vec<_> = source_keys
            .iter()
            .filter(|key| !keys.contains(*key))
            .collect();

        if !missing.is_empty() {
            has_errors = true;
            println!("Missing translations in {}.json:", lang);
            for key in missing {
                println!("  - {}", key);
            }
        }
    }

    // Check for unused translations
    let all_translation_keys: HashSet<_> = translation_files
        .values()
        .flat_map(|keys| keys.iter().cloned())
        .collect();

    let unused: Vec<_> = all_translation_keys
        .iter()
        .filter(|key| !source_keys.contains(*key))
        .collect();

    if !unused.is_empty() {
        println!("\nUnused translations:");
        for key in unused {
            println!("  - {}", key);
        }
    }

    assert!(!has_errors, "There are missing translations");
}

fn collect_source_keys() -> HashSet<String> {
    let mut keys = HashSet::new();
    let re = Regex::new(r#"i18n\.t\("([^"]+)"\)"#).unwrap();

    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap();
            for cap in re.captures_iter(&content) {
                keys.insert(cap[1].to_string());
            }
        }
    }

    keys
}

fn collect_translation_files() -> HashMap<String, HashSet<String>> {
    let mut translations = HashMap::new();
    let i18n_dir = Path::new("src/assets/i18n");

    for entry in fs::read_dir(i18n_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "json") {
            let content = fs::read_to_string(&path).unwrap();
            let json: Value = serde_json::from_str(&content).unwrap();

            let lang = path.file_stem().unwrap().to_str().unwrap().to_string();
            let keys = collect_keys_from_json(&json);
            translations.insert(lang, keys);
        }
    }

    translations
}

fn collect_keys_from_json(json: &Value) -> HashSet<String> {
    let mut keys = HashSet::new();

    fn collect_recursive(json: &Value, prefix: &str, keys: &mut HashSet<String>) {
        match json {
            Value::Object(map) => {
                for (key, value) in map {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    collect_recursive(value, &new_prefix, keys);
                }
            }
            Value::String(_) => {
                keys.insert(prefix.to_string());
            }
            _ => {}
        }
    }

    collect_recursive(json, "", &mut keys);
    keys
}
