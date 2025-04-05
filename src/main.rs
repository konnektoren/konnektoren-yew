use konnektoren_yew::prelude::*;

#[cfg(all(feature = "csr", not(feature = "ssr")))]
fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    yew::Renderer::<App>::new().render();
}

#[cfg(feature = "ssr")]
fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    // Use the appropriate SSR renderer
    yew::ServerRenderer::<App>::new().render();
}

#[cfg(not(any(feature = "csr", feature = "ssr")))]
fn main() {
    // Fallback for when neither feature is enabled
    println!("Error: Either 'csr' or 'ssr' feature must be enabled.");
    std::process::exit(1);
}
