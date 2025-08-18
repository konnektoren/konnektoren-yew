use konnektoren_platform::tools::{I18nChecker, I18nReportError};
use konnektoren_yew::i18n::create_i18n_config;
use std::env;
use std::fs;

fn main() -> Result<(), I18nReportError> {
    // Read env vars or use defaults
    let src_dir = std::env::var("I18N_SRC_DIR").unwrap_or_else(|_| "src".to_string());
    let reports_dir = std::env::var("I18N_REPORTS_DIR").unwrap_or_else(|_| "reports".to_string());

    // Load i18n config
    let config = konnektoren_yew::i18n::create_i18n_config();
    let checker = I18nChecker::new(config);
    let report = checker.check_directory(&src_dir);

    // Ensure reports directory exists
    std::fs::create_dir_all(&reports_dir).map_err(|e| I18nReportError::Other(e.to_string()))?;

    // Write reports
    std::fs::write(
        format!("{}/i18n_report.txt", reports_dir),
        report.as_report()?,
    )
    .map_err(|e| I18nReportError::Other(e.to_string()))?;

    std::fs::write(
        format!("{}/missing.yml", reports_dir),
        report.missing_as_yaml()?,
    )
    .map_err(|e| I18nReportError::Other(e.to_string()))?;

    std::fs::write(
        format!("{}/missing.json", reports_dir),
        report.missing_as_json()?,
    )
    .map_err(|e| I18nReportError::Other(e.to_string()))?;

    // Print summary to stdout
    println!("{}", report.as_report()?);

    if report.has_errors {
        println!("❌ Missing translations found.");
    } else {
        println!("✅ All translations complete.");
    }

    Ok(())
}
