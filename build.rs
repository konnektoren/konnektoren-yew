use anyhow::{Context, Result};
use vergen::*;

#[cfg(feature = "sbom")]
fn generate_sbom() -> Result<()> {
    // Try to generate SBOM, but don't fail the build if it doesn't work
    let sbom_result = generate_sbom_content().unwrap_or_else(|e| {
        eprintln!("Warning: SBOM generation skipped: {}", e);
        "{}".to_string() // Empty JSON object as fallback
    });

    // Always set the environment variable, even if empty
    println!("cargo:rustc-env=CARGO_SBOM={}", sbom_result);
    Ok(())
}

#[cfg(feature = "sbom")]
fn generate_sbom_content() -> Result<String> {
    use std::fs;
    use std::process::Command;

    // Generate SBOM
    let output = Command::new("cargo")
        .args([
            "cyclonedx",
            "--format",
            "json",
            "--output-file",
            "konnektoren-yew.cdx.json",
            "--no-build-deps",
            "--spec-version",
            "1.3",
            "--top-level",
            "-q",
        ])
        .output()
        .context("Failed to execute cargo cyclonedx")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "SBOM generation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Read the generated SBOM file
    let sbom = fs::read_to_string("konnektoren-yew.cdx.json")
        .context("Failed to read generated SBOM file")?;

    // Parse and minimize the SBOM
    let mut value: serde_json::Value =
        serde_json::from_str(&sbom).context("Failed to parse SBOM JSON")?;

    if let Some(obj) = value.as_object_mut() {
        // Keep only necessary fields
        let keep_fields = ["components", "metadata"];
        obj.retain(|k, _| keep_fields.contains(&k.as_str()));

        // Minimize component information
        if let Some(components) = obj.get_mut("components") {
            if let Some(components_array) = components.as_array_mut() {
                for component in components_array {
                    if let Some(component_obj) = component.as_object_mut() {
                        component_obj
                            .retain(|k, _| ["name", "version", "licenses"].contains(&k.as_str()));
                    }
                }
            }
        }
    }

    // Serialize the minimized SBOM
    serde_json::to_string(&value).context("Failed to serialize minimized SBOM")
}

#[cfg(not(feature = "sbom"))]
fn generate_sbom() -> Result<()> {
    // Set empty SBOM when feature is disabled
    println!("cargo:rustc-env=CARGO_SBOM={{}}");
    Ok(())
}

fn main() -> Result<()> {
    // Generate SBOM if feature is enabled
    generate_sbom()?;

    // Generate version information
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let rustc = RustcBuilder::all_rustc()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
