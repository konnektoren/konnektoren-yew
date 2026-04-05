use konnektoren_platform::tools::{I18nChecker, I18nReportError};
use konnektoren_yew::i18n::create_i18n_config;

#[test]
fn test_i18n_completeness() -> Result<(), I18nReportError> {
    // Initialize logging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    // Load configuration
    let config = create_i18n_config();

    // Create checker and run check, excluding test-only keys
    let checker = I18nChecker::new(config).exclude_tests();
    let report = checker.check_directory("src");

    // Print detailed report
    println!("{}", report.as_report()?);

    // Print per-language coverage
    let mut lang_codes: Vec<&String> = report.language_stats.keys().collect();
    lang_codes.sort();
    for lang in &lang_codes {
        let stats = &report.language_stats[*lang];
        println!(
            "  {}: {:.1}% ({}/{} keys)",
            lang,
            stats.coverage_percentage,
            stats.total_keys - stats.missing_keys,
            stats.total_keys,
        );
    }

    // Show missing keys per language on failure
    if report.has_errors {
        for lang in &lang_codes {
            if let Some(missing) = report.missing_translations.get(*lang) {
                if !missing.is_empty() {
                    let mut sorted_missing = missing.clone();
                    sorted_missing.sort();
                    println!("\nMissing keys for '{}':", lang);
                    for key in &sorted_missing {
                        println!("  - {}", key);
                    }
                }
            }
        }
    }

    // Assert 100% coverage for every language
    for lang in &lang_codes {
        let stats = &report.language_stats[*lang];
        assert_eq!(
            stats.missing_keys,
            0,
            "Language '{}' is missing {}/{} translations ({:.1}% coverage)",
            lang,
            stats.missing_keys,
            stats.total_keys,
            stats.coverage_percentage,
        );
    }

    Ok(())
}
