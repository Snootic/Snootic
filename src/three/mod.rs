use wasm_bindgen::prelude::*;
use leptos::web_sys;

#[wasm_bindgen(module = "/src/three/scene.js")]
extern "C" {
    pub fn HeroScene(frontCanvas: web_sys::HtmlCanvasElement, backCanvas: web_sys::HtmlCanvasElement, personal: JsValue);
}


#[wasm_bindgen(module = "/src/three/sceneCard.js")]
extern "C" {
    pub fn createCard(name: &str, title: &str, available: bool, R: f32, BAR_H: f32);
    pub fn createCardTexture(card: web_sys::HtmlCanvasElement);
    pub fn updateCardTexture(card: web_sys::HtmlCanvasElement, BAR_H: f32, elapsed: f32);
}