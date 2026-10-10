use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js/plot.js")]
extern "C" {
    #[wasm_bindgen(js_name = plotSheet)]
    pub fn plot_sheet(panel: &web_sys::Element);

    #[wasm_bindgen(js_name = clearPlot)]
    pub fn clear_plot(panel: &web_sys::Element);
}
