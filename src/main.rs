#[cfg(all(feature = "csr", not(feature = "ssr")))]
fn main() {
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
