use leptos::__reexports::send_wrapper::SendWrapper;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/three/panorama.js")]
extern "C" {
    #[wasm_bindgen(js_name = mountPano)]
    pub fn mount_pano(canvas: &web_sys::HtmlCanvasElement);

    #[wasm_bindgen(js_name = disposePano)]
    pub fn dispose_pano(canvas: &web_sys::HtmlCanvasElement);
}

#[component]
pub fn Panorama() -> impl IntoView {
    let canvas = NodeRef::<leptos::html::Canvas>::new();
    Effect::new(move |_| {
        let Some(canvas) = canvas.get() else { return };
        mount_pano(&canvas);
        let canvas: &web_sys::HtmlCanvasElement = &canvas;
        let canvas = SendWrapper::new(canvas.clone());
        on_cleanup(move || dispose_pano(&canvas));
    });
    view! {
        <div class="pano" aria-hidden="true">
            <canvas node_ref=canvas></canvas>
        </div>
    }
}

#[component]
pub fn ImportMap() -> impl IntoView {
    view! {
                                        <script type="importmap">
                                            {r#"{
                                                "imports": {
                                                    "three": "https://unpkg.com/three@0.183.1/build/three.module.js"
                                                }
                                            }"#}
                                        </script>
    }
}
