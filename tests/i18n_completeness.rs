use konnektoren_platform::tools::I18nChecker;
use konnektoren_yew::i18n::create_i18n_config;

#[test]
fn test_i18n_completeness() {
    // Initialize logging
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    // Load configuration
    let config = create_i18n_config();

    // Create checker and run check
    let checker = I18nChecker::new(config);
    let report = checker.check_directory("src");

    // Print detailed report
    report.print_report();

    // Assert no missing translations
    assert!(!report.has_errors, "There are missing translations");
}
