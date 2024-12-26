use regex::Regex;
use std::collections::HashSet;
use std::fs;
use walkdir::WalkDir;

fn is_font_awesome_class(class: &str) -> bool {
    let fa_patterns = ["fa-", "fas ", "far ", "fal ", "fab ", "fa "];

    fa_patterns.iter().any(|pattern| class.starts_with(pattern))
}

#[test]
fn test_all_used_classes_have_styles() {
    // 1. Collect all CSS classes from SCSS files
    let scss_classes = collect_scss_classes();

    // 2. Collect all classes used in Rust files
    let rust_classes: HashSet<String> = collect_rust_classes()
        .into_iter()
        .filter(|class| !is_font_awesome_class(&class))
        .collect();

    // 3. Compare and report
    let mut has_errors = false;

    // Check for missing styles
    let missing_styles: Vec<_> = rust_classes
        .iter()
        .filter(|class| !scss_classes.contains(*class))
        .collect();

    if !missing_styles.is_empty() {
        has_errors = true;
        println!("\nClasses used in Rust files but missing in SCSS:");
        for class in missing_styles {
            println!("  - {}", class);
        }
    }

    assert!(!has_errors, "There are missing styles");
}

#[test]
fn test_no_unused_styles() {
    // 1. Collect all CSS classes from SCSS files
    let scss_classes = collect_scss_classes();

    // 2. Collect all classes used in Rust files
    let rust_classes: HashSet<String> = collect_rust_classes()
        .into_iter()
        .filter(|class| !is_font_awesome_class(&class))
        .collect();

    // 3. Compare and report
    let mut has_errors = false;

    // Check for unused styles
    let unused_styles: Vec<_> = scss_classes
        .iter()
        .filter(|class| !rust_classes.contains(*class))
        .collect();

    if !unused_styles.is_empty() {
        has_errors = true;
        println!("\nUnused SCSS classes (not found in Rust files):");
        for class in unused_styles {
            println!("  - {}", class);
        }
    }

    assert!(!has_errors, "There are unused styles");
}

struct ScssContext {
    classes: HashSet<String>,
}

fn collect_rust_classes() -> HashSet<String> {
    let mut classes = HashSet::new();
    // Matches class="...", classes="...", and class={...} patterns
    let re = Regex::new(r#"class(?:es)?=["{{]([^"}}]+)["}}]"#).unwrap();

    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap();

            for cap in re.captures_iter(&content) {
                // Split multiple classes and trim each one
                let class_str = &cap[1];
                for class in class_str.split_whitespace() {
                    // Remove any quotes or brackets
                    let cleaned_class = class
                        .trim_matches(|c| c == '"' || c == '{' || c == '}')
                        .to_string();
                    classes.insert(cleaned_class);
                }
            }
        }
    }

    classes
}

fn collect_scss_classes() -> HashSet<String> {
    let mut context = ScssContext {
        classes: HashSet::new(),
    };

    for entry in WalkDir::new("scss") {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| ext == "scss") {
            let content = fs::read_to_string(entry.path()).unwrap();
            process_scss_content(&content, &mut context);
        }
    }

    context.classes
}

fn process_scss_content(content: &str, context: &mut ScssContext) {
    let class_re = Regex::new(r"^[\s\t]*\.([a-zA-Z0-9_-]+)[\s\t]*\{").unwrap();
    let bem_re = Regex::new(r"^[\s\t]*&(?:__|--|:)?([a-zA-Z0-9_-]*)").unwrap();

    let mut brace_level = 0;
    let mut last_block = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Count braces to track nesting
        brace_level += trimmed.matches('{').count() as i32;
        brace_level -= trimmed.matches('}').count() as i32;

        // Handle closing braces - pop the last block
        if trimmed.contains('}') && !last_block.is_empty() {
            last_block.pop();
        }

        // Handle main class definitions
        if let Some(cap) = class_re.captures(trimmed) {
            let class_name = cap[1].to_string();
            last_block.push(class_name.clone());
            context.classes.insert(class_name);
            continue;
        }

        // Handle BEM notation
        if let Some(cap) = bem_re.captures(trimmed) {
            if let Some(current_block) = last_block.last() {
                let suffix = cap[1].to_string();
                if trimmed.contains("__") {
                    // Element
                    context
                        .classes
                        .insert(format!("{}__{}", current_block, suffix));
                } else if trimmed.contains("--") {
                    // Modifier
                    context
                        .classes
                        .insert(format!("{}--{}", current_block, suffix));
                } else if trimmed.contains(':') {
                    // Pseudo-class, not a BEM class
                    continue;
                } else if !suffix.is_empty() {
                    // Handle other & cases
                    context
                        .classes
                        .insert(format!("{}_{}", current_block, suffix));
                }
            }
        }
    }
}
