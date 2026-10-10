//! No router: a client-side router intercepts the record links and rewrites a real
//! fragment navigation into a history update, which does not move `:target`.

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};

use crate::components::shell::Shell;
use crate::libs::content::get_content;
use crate::libs::locale;

const TITLE: &str = "app.meta.title";
const DESCRIPTION: &str = "app.meta.description";

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let lang = RwSignal::new(locale::detect());
    provide_context(lang);

    // The server always resolves strings in the default language, and hydration reuses
    // its DOM without re-running a component body, so the shell is rebuilt under a key
    // exactly once on the client.
    let built = RwSignal::new(0u32);
    Effect::new(move |_| {
        locale::reflect(lang.get_untracked());
        locale::mark_ready();
        built.update(|n| *n += 1);
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Title text=move || locale::t(TITLE)/>
        <Meta name="description" content=move || locale::t(DESCRIPTION)/>
        <Meta property="og:title" content=move || locale::t(TITLE)/>
        <Meta property="og:description" content=move || locale::t(DESCRIPTION)/>
        <Meta property="og:type" content="profile"/>
        <Meta name="theme-color" content="#0a0907"/>

        {move || {
            built.get();
            // Translated views subscribe themselves; language changes must not
            // destroy navigation state or reset the animation register.
            untrack(|| view! { <Shell content=get_content()/> })
        }}
    }
}
