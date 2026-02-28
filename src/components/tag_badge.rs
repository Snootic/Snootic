use leptos::prelude::*;

#[component]
pub fn TagBadge(tag: &'static str) -> impl IntoView {
    view! {
        <span class="px-2 py-0.5 rounded font-mono text-[11px] text-ink-3 bg-page/60 border border-edge">
            {tag}
        </span>
    }
}
