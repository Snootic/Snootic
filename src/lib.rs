extern crate rust_i18n;
rust_i18n::i18n!("locales", fallback = "en");

pub mod app;
pub mod components;
pub mod cube;
pub mod js;
pub mod libs;
pub mod structs;
pub mod three;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
