use anyhow::Result;
use std::process::Command;
use vergen::*;

fn main() -> Result<()> {
    #[cfg(feature = "sbom")]
    {
        // Generate minimal SBOM with only necessary information
        let output = Command::new("cargo")
            .args([
                "cyclonedx",
                "--format",
                "json",
                "--output-file",
                "konnektoren-yew.cdx.json",
                "--no-build-deps", // Exclude build dependencies
                "--spec-version",
                "1.3",         // Use minimal spec version
                "--top-level", // Only top-level dependencies
                "-q",          // Quiet mode
            ])
            .output()
            .expect("Failed to generate SBOM");

        if !output.status.success() {
            eprintln!(
                "SBOM generation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Read and minify SBOM
        if let Ok(sbom) = std::fs::read_to_string("konnektoren-yew.cdx.json") {
            // Parse and extract only needed fields
            if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&sbom) {
                if let Some(obj) = value.as_object_mut() {
                    // Keep only necessary fields
                    let keep_fields = ["components", "metadata"];
                    obj.retain(|k, _| keep_fields.contains(&k.as_str()));

                    // For components, keep only name, version, and licenses
                    if let Some(components) = obj.get_mut("components") {
                        if let Some(components_array) = components.as_array_mut() {
                            for component in components_array {
                                if let Some(component_obj) = component.as_object_mut() {
                                    component_obj.retain(|k, _| {
                                        ["name", "version", "licenses"].contains(&k.as_str())
                                    });
                                }
                            }
                        }
                    }
                }

                // Write minified SBOM
                if let Ok(minimal_sbom) = serde_json::to_string(&value) {
                    println!("cargo:rustc-env=CARGO_SBOM={}", minimal_sbom);
                }
            }
        }
    }

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
