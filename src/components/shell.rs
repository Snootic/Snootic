use leptos::prelude::*;

use crate::components::instrument::Instrument;
use crate::components::nav::{measure_scroll, plot_active, Nav};
use crate::components::record::Records;
use crate::components::sheets::{
    contact::Contact, identity::Identity, stack::Stack, track::Track, work::Work,
};
use crate::libs::browser::{document_opt, el};
use crate::libs::locale::{self, Locale};
use crate::structs::{PortfolioContent, SHEETS};
use crate::three::Panorama;

fn measure_dim() {
    let Some(doc) = document_opt() else {
        return;
    };
    let Ok(Some(h1)) = doc.query_selector("#p-id .macro") else {
        return;
    };
    let width = h1.get_bounding_client_rect().width().round();
    if let Some(dim) = el("dim") {
        let _ = dim.set_attribute("style", &format!("--dim-w:{width}px"));
    }
    if let Some(val) = el("dim-val") {
        val.set_text_content(Some(&format!("{} px", width as i64)));
    }
}

#[component]
pub fn Shell(content: PortfolioContent) -> impl IntoView {
    let nav = Nav::new(content.clone());
    provide_context(nav);

    let lang = locale::locale();
    let frame_ref = NodeRef::<leptos::html::Div>::new();

    let personal = content.personal.clone();
    let brand_role = personal.title;
    let c = content.clone();
    let readout = Memo::new(move |_| {
        let cube = crate::cube::Cube::new(nav.orient.get());
        let (active, neighbours) = (cube.active(), cube.neighbours());
        let max = PortfolioContent::LINE_MAX;
        let candidates = [
            c.readout(active, neighbours, &nav.state_line(false), false),
            c.readout(active, neighbours, &nav.state_line(true), false),
            c.readout(active, neighbours, &nav.state_line(true), true),
        ];
        candidates
            .iter()
            .find(|line| line.chars().count() <= max)
            .unwrap_or(&candidates[2])
            .clone()
    });

    let sheets: &'static [_] = &SHEETS;

    Effect::new(move |ran: Option<bool>| {
        if ran == Some(true) {
            return true;
        }
        nav.boot_from_hash();
        true
    });

    Effect::new(move |_| {
        let _ = nav.plot.get();
        plot_active(nav);
        measure_scroll(nav);
        measure_dim();
    });

    crate::components::input::mount(nav, frame_ref);

    view! {
        <a class="skip" href="#stage">
            {move || locale::t("app.skip.content")}
        </a>
        <p class="sr-only" id="live" role="status" aria-live="polite"></p>

        <Panorama/>

        <svg class="sprite" aria-hidden="true" focusable="false">
            <defs>
                <symbol id="dim-arrow" viewBox="0 0 28 8">
                    <path d="M1 4h26" stroke="currentColor" stroke-width="1"/>
                    <path d="M7 1L1.5 4 7 7z" fill="currentColor"/>
                    <path d="M21 1l5.5 3L21 7z" fill="currentColor"/>
                </symbol>
                <symbol id="arrow-head" viewBox="0 0 6 7">
                    <path d="M6 0L0 3.5 6 7z" fill="currentColor"/>
                </symbol>
                <pattern
                    id="dwg45"
                    width="6"
                    height="6"
                    patternUnits="userSpaceOnUse"
                    patternTransform="rotate(45)"
                >
                    <line
                        x1="0"
                        y1="0"
                        x2="0"
                        y2="6"
                        stroke="rgba(240,235,229,.10)"
                        stroke-width="0.5"
                    />
                </pattern>
            </defs>
        </svg>

        <Instrument nav=nav/>

        <div class="frame" id="frame" node_ref=frame_ref>
            <header class="head">
                <a class="brand" href="#p-id">
                    <span class="brand-mark" aria-hidden="true">
                        {personal.initials}
                    </span>
                    <span class="brand-name">
                        <span>{personal.name}</span>
                        <span class="brand-sep" aria-hidden="true">
                            "/"
                        </span>
                        <span class="brand-role">{move || locale::t(brand_role)}</span>
                    </span>
                </a>
                <div class="head-status">
                    <button
                        class="mtog js-only"
                        id="mtog"
                        type="button"
                        aria-pressed=move || {
                            if nav.motion.get() == crate::components::nav::Motion::Plot {
                                "true"
                            } else {
                                "false"
                            }
                        }
                        aria-label=move || locale::t("app.register.motion_aria")
                        on:click=move |_| nav.toggle_motion()
                    >
                        <span class="t-label">
                            {move || locale::t("app.register.motion")}
                        </span>
                        <span class="mtog-val" data-mtval>
                            {move || nav.motion.get().label()}
                        </span>
                        <span class="mtog-alt" aria-hidden="true">
                            {move || match nav.motion.get() {
                                crate::components::nav::Motion::Plot => "/ Mech",
                                crate::components::nav::Motion::Mech => "/ Plot",
                            }}
                        </span>
                    </button>
                    <button
                        class="mtog js-only"
                        id="ltog"
                        type="button"
                        data-alt=move || {
                            if lang.get() == Locale::En { "false" } else { "true" }
                        }
                        aria-label=move || {
                            let next = lang.get().next();
                            format!(
                                "{}. {} {}.",
                                locale::t("app.register.language"),
                                locale::t("app.register.switch_to"),
                                next.name(),
                            )
                        }
                        on:click=move |_| locale::set_locale(lang, lang.get_untracked().next())
                    >
                        <span class="t-label">{move || locale::t("app.register.language")}</span>
                        <span class="mtog-val">{move || lang.get().label()}</span>
                        <span class="mtog-alt" aria-hidden="true">
                            {move || format!(" / {}", lang.get().alt_label())}
                        </span>
                    </button>
                    <a
                        class="back"
                        aria-label=move || locale::t("app.back.cv")
                        href=personal.resume
                        target="_blank"
                        rel="noopener"
                    >
                        "\u{2039} "
                        <span>{move || locale::t("app.back.cv")}</span>
                    </a>
                </div>
            </header>

            <main class="stage" id="stage" tabindex="-1">
                {sheets
                    .iter()
                    .enumerate()
                    .map(|(i, sheet)| {
                        let body = match i {
                            0 => view! { <Identity content=content.clone()/> }.into_any(),
                            1 => view! { <Work content=content.clone()/> }.into_any(),
                            2 => view! { <Stack content=content.clone()/> }.into_any(),
                            3 => view! { <Track content=content.clone()/> }.into_any(),
                            _ => view! { <Contact content=content.clone()/> }.into_any(),
                        };
                        view! {
                            <section
                                class="panel"
                                id=sheet.id
                                class:is-on=move || nav.booted.get() && nav.sheet.get() == i
                                data-sheet=sheet.no
                                data-face=sheet.face.key()
                                data-in=move || nav.reveal.get()[i].key()
                                aria-label=move || locale::t(sheet.title)
                            >
                                <svg class="dwg" aria-hidden="true"></svg>
                                {body}
                                <p class="cont" hidden=move || !nav.scrolls.get()[i]>
                                    {move || locale::t("app.panel.continued")}
                                </p>
                                <span class="pen" aria-hidden="true">
                                    <span class="pen-head"></span>
                                    <span class="pen-trail"></span>
                                </span>
                            </section>
                        }
                    })
                    .collect_view()}
            </main>

            <div class="hintrow">
                <nav class="stations" aria-label=move || locale::t("app.stations.aria")>
                    {sheets
                        .iter()
                        .enumerate()
                        .map(|(i, sheet)| {
                            view! {
                                <a
                                    class="st"
                                    href=format!("#{}", sheet.id)
                                    data-sheet=sheet.id
                                    aria-current=move || {
                                        if nav.booted.get() && nav.sheet.get() == i {
                                            "true"
                                        } else {
                                            "false"
                                        }
                                    }
                                >
                                    <span class="st-no num">{sheet.no}</span>
                                    <span class="st-nm">{move || locale::t(sheet.title)}</span>
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>
                <p class="hint">
                    <span id="ro-line">{move || readout.get()}</span>
                    <span class="cube-noscript">
                        {move || locale::t("app.hint.noscript")}
                    </span>
                </p>
                <details class="keys" id="keys">
                    <summary>
                        {move || locale::t("app.controls.summary")}
                    </summary>
                    <div class="keys-body">
                        <ol class="keys-list">
                            <li>
                                <b>{move || locale::t("app.controls.item1.title")}</b>
                                {move || locale::t("app.controls.item1.body")}
                            </li>
                            <li>
                                <b>{move || locale::t("app.controls.item2.title")}</b>
                                " \u{2014} " <i>"\u{2190}\u{2191}\u{2192}\u{2193}"</i>
                                {move || locale::t("app.controls.item2.preview")}
                                <i>"Enter"</i>
                                {move || locale::t("app.controls.item2.commits")}
                                <i>"1"</i> "\u{2013}" <i>"5"</i>
                                {move || locale::t("app.controls.item2.jump")}
                                <i>"Esc"</i>
                                {move || locale::t("app.controls.item2.closes")}
                                <i>"M"</i>
                                {move || locale::t("app.controls.item2.flips")}
                                <i>"PLOT"</i>
                                {move || locale::t("app.controls.item2.plot")}
                                <i>"MECH"</i>
                                {move || locale::t("app.controls.item2.mech")}
                            </li>
                            <li>
                                <b>{move || locale::t("app.controls.item3.title")}</b>
                                {move || locale::t("app.controls.item3.body")}
                                <i>"Esc"</i>
                                {move || locale::t("app.controls.item3.tail")}
                            </li>
                            <li>
                                <b>{move || locale::t("app.controls.item4.title")}</b>
                                {move || locale::t("app.controls.item4.body")}
                                <i>"\u{2190}"</i>
                                {move || locale::t("app.controls.item4.and")}
                                <i>"\u{2192}"</i>
                                {move || locale::t("app.controls.item4.move")}
                            </li>
                            <li>
                                <b>{move || locale::t("app.controls.item5.title")}</b>
                                {move || locale::t("app.controls.item5.body")}
                            </li>
                        </ol>
                    </div>
                </details>
            </div>

            <nav class="nojs" aria-label=move || locale::t("app.register.nojs_aria")>
                <span class="t-label">
                    {move || locale::t("app.register.sheets")}
                </span>
                {sheets
                    .iter()
                    .map(|sheet| {
                        view! {
                            <a class="fr" href=format!("#{}", sheet.id)>
                                <span class="fr-no num">{sheet.no}</span>
                                " "
                                {move || locale::t(sheet.title)}
                            </a>
                        }
                    })
                    .collect_view()}
                {content
                    .projects
                    .iter()
                    .map(|p| {
                        view! {
                            <a class="fr fr-rec" href=format!("#{}", p.id)>
                                <span class="fr-no num">{format!("R{}", p.no)}</span>
                                " "
                                {move || locale::t(p.title)}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>
        </div>

        <Records content=content.clone()/>
    }
}
