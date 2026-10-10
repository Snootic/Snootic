use leptos::web_sys;

pub fn window_opt() -> Option<web_sys::Window> {
    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    {
        web_sys::window()
    }
    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    {
        None
    }
}

pub fn document_opt() -> Option<web_sys::Document> {
    window_opt()?.document()
}

pub fn el(id: &str) -> Option<web_sys::Element> {
    document_opt()?.get_element_by_id(id)
}

pub fn now_ms() -> f64 {
    window_opt()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

pub fn reduced_motion() -> bool {
    window_opt()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches())
}
