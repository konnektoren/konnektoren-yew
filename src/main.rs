#[cfg(all(feature = "csr", not(feature = "ssr")))]
fn main() {
    use tracing_wasm::WASMLayerConfigBuilder;
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::INFO)
            .build(),
    );
    yew::Renderer::<konnektoren_yew::prelude::App>::new().render();
}

#[cfg(feature = "ssr")]
fn main() {
    yew::ServerRenderer::<konnektoren_yew::app_ssr::App>::new().render();
}

#[cfg(not(any(feature = "csr", feature = "ssr")))]
fn main() {
    println!("Error: Either 'csr' or 'ssr' feature must be enabled.");
    std::process::exit(1);
}
