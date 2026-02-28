use crate::components::*;
use crate::libs::content::get_content;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        <Title text="Snootic | Portfolio"/>

        <Router>
            <Routes fallback=move || "Not found.">
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=WildcardSegment("any") view=NotFound/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let content = get_content();

    view! {
        <>
            <navbar::Navbar personal=content.personal.clone() />
            <main>
                <hero::Hero personal=content.personal.clone() />
                <about::About personal=content.personal.clone() stats=content.stats />
                <skills::Skills skills=content.skills />
                <experience::Experience experience=content.experience />
                <projects::Projects projects=content.projects />
                <contact::Contact personal=content.personal.clone() />
            </main>
            <footer::Footer personal=content.personal.clone() social=content.social.clone() />
        </>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
