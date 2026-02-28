use leptos::prelude::*;

#[component]
pub fn SectionHeader(number: u8, title: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-4 mb-16">
            <span class="font-mono text-sm text-brand">{number}</span>
            <div class="h-px w-14 bg-edge" />
            <h2 class="text-3xl font-bold text-ink">{title}</h2>
        </div>
    }
}