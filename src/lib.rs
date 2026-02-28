pub mod app;
pub mod components;
pub mod libs;
pub mod structs;
pub mod three;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
