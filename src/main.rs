use konnektoren_yew::prelude::*;

fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    yew::Renderer::<App>::new().render();
}
