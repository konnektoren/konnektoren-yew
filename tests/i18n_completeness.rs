use konnektoren_platform::i18n::Language;
use konnektoren_yew::i18n::create_i18n_config;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use walkdir::WalkDir;

#[test]
fn test_i18n_completeness() {
    // Initialize logging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    // Load configuration with both platform and local translations
    let config = create_i18n_config();

    // 1. Collect all i18n keys from source files
    let source_keys = collect_source_keys();
    log::info!("Found {} keys in source files", source_keys.len());

    // 2. Get all translation keys from config
    let mut translation_keys: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_keys = HashSet::new();
    let empty_set = HashSet::new();

    for lang in Language::builtin() {
        if let Some(translations) = config.translations.get(lang.code()) {
            if let Some(obj) = translations.as_object() {
                let keys: HashSet<_> = obj.keys().cloned().collect();
                all_keys.extend(keys.clone());
                translation_keys.insert(lang.code().to_string(), keys);
            }
        }
    }

    // 3. Compare and report
    let mut has_errors = false;

    // Check for missing translations
    log::info!("\nChecking translations completeness:");
    for lang in Language::builtin() {
        let lang_code = lang.code();
        let keys = translation_keys.get(lang_code).unwrap_or(&empty_set);

        let missing: Vec<_> = source_keys
            .iter()
            .filter(|key| !keys.contains(*key))
            .collect();

        if !missing.is_empty() {
            has_errors = true;
            log::error!(
                "Missing translations in {} ({}):",
                lang.native_name(),
                lang_code
            );
            for key in missing {
                log::error!("  - {}", key);
                // Show default translation if available
                if let Some(default_trans) = config.translations.get("en") {
                    if let Some(trans) = default_trans.get(key) {
                        log::info!("    Default (EN): {}", trans);
                    }
                }
            }
        } else {
            log::info!(
                "{} ({}) - All translations present",
                lang.native_name(),
                lang_code
            );
        }

        // Show translation coverage
        let coverage = if !source_keys.is_empty() {
            (keys.len() as f64 / source_keys.len() as f64 * 100.0).round()
        } else {
            100.0
        };
        log::info!(
            "{} coverage: {:.1}% ({}/{} keys)",
            lang.native_name(),
            coverage,
            keys.len(),
            source_keys.len()
        );
    }

    // Check for unused translations
    let unused: Vec<_> = all_keys
        .iter()
        .filter(|key| !source_keys.contains(*key))
        .collect();

    if !unused.is_empty() {
        log::warn!("\nUnused translations:");
        for key in unused {
            log::warn!("  - {}", key);
            // Show translations for unused keys
            for lang in Language::builtin() {
                let trans = config.t_with_lang(key, &lang);
                log::info!("    {} ({}): {}", lang.native_name(), lang.code(), trans);
            }
        }
    }

    // Print summary
    log::info!("\nTranslation Summary:");
    log::info!("Total keys in source: {}", source_keys.len());
    log::info!("Total unique translations: {}", all_keys.len());
    log::info!("Supported languages: {}", Language::builtin().len());

    assert!(!has_errors, "There are missing translations");
}

fn collect_source_keys() -> HashSet<String> {
    let mut keys = HashSet::new();
    let re = Regex::new(r#"(?:i18n\.t|t_with_lang)\("([^"]+)"\)"#).unwrap();

    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for cap in re.captures_iter(&content) {
                    keys.insert(cap[1].to_string());
                }
            }
        }
    }

    keys
}
