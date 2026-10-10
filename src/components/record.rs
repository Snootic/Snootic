use crate::libs::browser::document_opt;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

use crate::components::nav::Nav;
use crate::libs::locale;
use crate::structs::Project;

fn host(url: &str) -> &str {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or(url)
}

fn trap_focus(ev: web_sys::KeyboardEvent) {
    if ev.key() != "Tab" {
        return;
    }
    let Some(doc) = ev
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    else {
        return;
    };
    let scope = doc
        .query_selector(".plate-zoom")
        .ok()
        .flatten()
        .unwrap_or(doc);
    let Ok(nodes) = scope.query_selector_all("a[href], button:not([disabled]), [tabindex=\"0\"]")
    else {
        return;
    };
    let Some(first) = nodes
        .item(0)
        .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };
    let Some(last) = nodes
        .item(nodes.length() - 1)
        .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };
    let active = document_opt().and_then(|d| d.active_element());
    let boundary: &web_sys::Element = if ev.shift_key() { &first } else { &last };
    if active.as_ref() == Some(boundary) || active.as_ref() == Some(&scope) {
        ev.prevent_default();
        let _ = if ev.shift_key() {
            last.focus()
        } else {
            first.focus()
        };
    }
}

#[component]
fn Record(p: Project, total: usize) -> impl IntoView {
    let nav = expect_context::<Nav>();
    let selected = RwSignal::new(0usize);
    let zoom = RwSignal::new(false);
    let zoom_ref = NodeRef::<leptos::html::Div>::new();
    let plate_ref = NodeRef::<leptos::html::Button>::new();
    let doc_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if nav.open.get() != Some(p.id) || nav.closing.get() {
            zoom.set(false);
        } else if let Some(doc) = doc_ref.get() {
            let _ = doc.focus();
        }
    });

    Effect::new(move |was_zoomed: Option<bool>| {
        let zoomed = zoom.get();
        if zoomed {
            if let Some(el) = zoom_ref.get() {
                let _ = el.focus();
            }
        } else if was_zoomed == Some(true) && nav.open.get_untracked() == Some(p.id) {
            if let Some(button) = plate_ref.get() {
                let _ = button.focus();
            }
        }
        zoomed
    });
    let plates = p.plates;
    let entry = format!("{} / {:02}", p.no, total);
    let is_open = move || nav.open.get() == Some(p.id);

    let slot = move || {
        let i = selected.get().min(plates.len().saturating_sub(1));
        match plates.get(i) {
            Some(plate) => {
                let state = match plate.image {
                    Some(_) => locale::t("record.plate.captured"),
                    None => locale::t("record.plate.not_captured"),
                };
                format!(
                    "{} {} \u{2014} 1600\u{00d7}1000 \u{2014} {} \u{2014} {}",
                    locale::t("record.plate.noun"),
                    plate.no,
                    locale::t(plate.label),
                    state
                )
            }
            None => locale::t("record.plate.none"),
        }
    };

    view! {
        <div class="popup" id=p.id class:is-open=is_open
            class:is-closing=move || is_open() && nav.closing.get()
        >
            <a class="popup-scrim" href="#p-work" tabindex="-1" aria-hidden="true"
                on:click=move |ev| {
                    ev.prevent_default();
                    nav.close_record();
                }
            ></a>
            <div
                class="popup-doc"
                node_ref=doc_ref
                on:keydown=trap_focus
                on:animationend=move |ev: leptos::web_sys::AnimationEvent| {
                    if is_open() && ev.animation_name() == "rec-close" {
                        nav.finish_record_close();
                    }
                }
                on:animationcancel=move |ev: leptos::web_sys::AnimationEvent| {
                    if is_open() && ev.animation_name() == "rec-close" {
                        nav.finish_record_close();
                    }
                }
                role="dialog"
                aria-modal="true"
                aria-labelledby=format!("{}-h", p.id)
                aria-describedby=format!("{}-d", p.id)
                tabindex="-1"
            >
                <header class="popup-head">
                    <span class="popup-idx num" aria-hidden="true">
                        {p.no}
                    </span>
                    <div>
                        <h2 class="popup-title t-head" id=format!("{}-h", p.id)>
                            {move || locale::t(p.title)}
                        </h2>
                        // the stack has its own section below; listing it here said the same thing twice
                        <p class="popup-flag" data-status=if p.featured { "featured" } else { "archive" }>
                            {move || locale::t(p.status())}
                        </p>
                    </div>
                    <a class="popup-close" href="#p-work"
                        on:click=move |ev| {
                            ev.prevent_default();
                            nav.close_record();
                        }
                    >
                        {move || locale::t("record.button.close")}
                        <span aria-hidden="true">"\u{00d7}"</span>
                    </a>
                </header>

                <div class="popup-body">
                    <div class="popup-col">
                        <p class="popup-k">{move || locale::t("record.heading.brief")}</p>
                        <p class="popup-long t-body" id=format!("{}-d", p.id)>
                            {move || locale::t(p.brief)}
                        </p>
                        <p class="popup-k">{move || locale::t("record.heading.detail")}</p>
                        {move || p.paragraphs()
                            .into_iter()
                            .map(|para| view! { <p class="popup-long t-body">{para}</p> })
                            .collect_view()}
                        <p class="popup-k">
                            {move || locale::t("record.heading.stack")}
                            <span class="popup-n num">{format!("{:02}", p.tags.len())}</span>
                        </p>
                        <ul class="tag-row">
                            {move || p.tags
                                .iter()
                                .map(|t| view! { <li class="tag">{locale::t(t)}</li> })
                                .collect_view()}
                        </ul>
                    </div>

                    <div class="popup-col">
                        <p class="popup-k">
                            {move || locale::t("record.heading.plates")}
                            <span class="popup-n num">{format!("{:02}", plates.len())}</span>
                        </p>
                        {if plates.is_empty() {
                            view! {
                                <p class="plate-empty">
                                    {move || locale::t("record.plate.none_yet")}
                                </p>
                            }
                                .into_any()
                        } else {
                            view! {
                                {move || {
                                    if !zoom.get() {
                                        return ().into_any();
                                    }
                                    let i = selected.get().min(plates.len().saturating_sub(1));
                                    match plates.get(i).and_then(|pl| pl.image) {
                                        Some(src) => view! {
                                            <div
                                                class="plate-zoom"
                                                node_ref=zoom_ref
                                                tabindex="-1"
                                                role="dialog"
                                                aria-modal="true"
                                                aria-label=move || locale::t("record.plate.expand")
                                                on:keydown=move |ev: leptos::web_sys::KeyboardEvent| {
                                                    /* the zoom is on top, so it answers Escape first;
                                                       otherwise the sheet's handler closes the record behind it */
                                                    if ev.key() == "Escape" {
                                                        ev.prevent_default();
                                                        ev.stop_propagation();
                                                        zoom.set(false);
                                                    }
                                                }
                                            >
                                                <div
                                                    class="plate-zoom-scrim"
                                                    aria-hidden="true"
                                                    on:click=move |_| zoom.set(false)
                                                ></div>
                                                <figure class="plate-zoom-fig">
                                                    <img src=src alt=slot()/>
                                                    <figcaption class="plate-zoom-cap">{slot()}</figcaption>
                                                </figure>
                                                <button
                                                    class="plate-zoom-x js-only"
                                                    type="button"
                                                    on:click=move |_| zoom.set(false)
                                                >
                                                    {move || locale::t("record.plate.close")}
                                                </button>
                                            </div>
                                        }
                                            .into_any(),
                                        None => ().into_any(),
                                    }
                                }}
                                <div class="plates">
                                    <figure
                                        class="plate-main"
                                        data-captured=move || {
                                            let i = selected.get().min(plates.len().saturating_sub(1));
                                            match plates.get(i).and_then(|pl| pl.image) {
                                                Some(_) => "true",
                                                None => "false",
                                            }
                                        }
                                    >
                                        {move || {
                                            let i = selected.get().min(plates.len().saturating_sub(1));
                                            match plates.get(i).and_then(|pl| pl.image) {
                                                Some(src) => view! {
                                                    <button
                                                        class="plate-open"
                                                        node_ref=plate_ref
                                                        type="button"
                                                        aria-label=move || locale::t("record.plate.expand")
                                                        on:click=move |_| zoom.set(true)
                                                    >
                                                        <img class="plate-img" src=src alt=slot() loading="lazy"/>
                                                    </button>
                                                }
                                                    .into_any(),
                                                None => ().into_any(),
                                            }
                                        }}
                                        <figcaption
                                            class="plate-slot"
                                            aria-live="polite"
                                            data-plate-slot
                                        >
                                            {slot}
                                        </figcaption>
                                    </figure>
                                    <ul
                                        class="plate-strip"
                                        aria-label=move || locale::t("record.plate.select")
                                    >
                                        {plates
                                            .iter()
                                            .enumerate()
                                            .map(|(i, plate)| {
                                                let count = plates.len();
                                                view! {
                                                    <li>
                                                        <button
                                                            class="plate-th"
                                                            type="button"
                                                            aria-pressed=move || if selected.get() == i { "true" } else { "false" }
                                                            on:click=move |_| selected.set(i)
                                                            on:keydown=move |ev: leptos::web_sys::KeyboardEvent| {
                                                                let to = match ev.key().as_str() {
                                                                    "ArrowRight" => (i + 1) % count,
                                                                    "ArrowLeft" => (i + count - 1) % count,
                                                                    "Home" => 0,
                                                                    "End" => count - 1,
                                                                    _ => return,
                                                                };
                                                                ev.prevent_default();
                                                                ev.stop_propagation();
                                                                selected.set(to);
                                                                if let Some(next) = ev.current_target()
                                                                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                                                                    .and_then(|n| n.closest(".plate-strip").ok().flatten())
                                                                    .and_then(|n| n.query_selector_all(".plate-th").ok())
                                                                    .and_then(|nodes| nodes.item(to as u32))
                                                                    .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                                                                {
                                                                    let _ = next.focus();
                                                                }
                                                            }
                                                        >
                                                            <span class="plate-no num">{plate.no}</span>
                                                            <span class="plate-lbl">{move || locale::t(plate.label)}</span>
                                                        </button>
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                </div>
                            }
                                .into_any()
                        }}

                        <p class="popup-k">{move || locale::t("record.heading.record")}</p>
                        <dl class="popup-facts">
                            <div class="fact">
                                <dt>{move || locale::t("record.fact.entry")}</dt>
                                <dd class="num">{entry}</dd>
                            </div>
                            <div class="fact">
                                <dt>{move || locale::t("record.fact.status")}</dt>
                                <dd>{move || locale::t(p.status())}</dd>
                            </div>
                            <div class="fact">
                                <dt>{move || locale::t("record.fact.kind")}</dt>
                                <dd>{move || locale::t(p.kind)}</dd>
                            </div>
                            <div class="fact">
                                <dt>{move || locale::t("record.fact.repository")}</dt>
                                <dd>{move || match p.source {
                                    Some(href) => host(href).to_string(),
                                    None => locale::t("record.fact.none"),
                                }}</dd>
                            </div>
                            <div class="fact">
                                <dt>{move || locale::t("record.fact.live")}</dt>
                                <dd>{move || match p.live {
                                    Some(href) => host(href).to_string(),
                                    None => locale::t("record.fact.none"),
                                }}</dd>
                            </div>
                        </dl>

                        <div class="popup-links">
                            {p.source
                                .map(|href| {
                                    view! {
                                        <a
                                            class="plink"
                                            href=href
                                            target="_blank"
                                            rel="noopener"
                                        >
                                            <span>{move || locale::t("record.fact.repository")}</span>
                                            <svg class="ico" aria-hidden="true">
                                                <use href="#dim-arrow"/>
                                            </svg>
                                        </a>
                                    }
                                })}
                            {p.live
                                .map(|href| {
                                    view! {
                                        <a
                                            class="plink plink-alt"
                                            href=href
                                            target="_blank"
                                            rel="noopener"
                                        >
                                            <span>{move || locale::t("record.fact.live")}</span>
                                            <svg class="ico" aria-hidden="true">
                                                <use href="#dim-arrow"/>
                                            </svg>
                                        </a>
                                    }
                                })}
                        </div>
                    </div>
                </div>

                <p class="popup-foot">
                    <span>{move || locale::t("record.foot.return")}</span>
                    <span class="num">
                        {p.plates.len()}
                        {move || locale::t("record.plate.plural")}
                    </span>
                </p>
                <span class="crt" aria-hidden="true"></span>
            </div>
        </div>
    }
}

#[component]
pub fn Records(content: crate::structs::PortfolioContent) -> impl IntoView {
    let total = content.projects.len();
    view! {
        <>
            {content
                .projects
                .iter()
                .map(|p| view! { <Record p=*p total=total/> })
                .collect_view()}
        </>
    }
}
