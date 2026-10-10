use super::SheetHead;
use leptos::prelude::*;

use crate::libs::locale;
use crate::structs::{PortfolioContent, Term};

fn role(no: usize, t: Term) -> impl IntoView {
    let number = format!("{:02}", no + 1);
    let stack = t.stack.join(" \u{00b7} ");
    view! {
        <li class="track-item">
            <div class="track-rail">
                <span class="track-num num">{number}</span>
                <span class="track-dot" aria-hidden="true"></span>
            </div>
            <div class="track-main">
                <h2 class="track-role t-head">{locale::t(t.title)}</h2>
                <p class="track-when num">{locale::t(t.period)}</p>
                <p class="track-co t-read">
                    {t.org}
                    {t.location.map(|loc| format!(" \u{2014} {}", locale::t(loc)))}
                </p>
                <ul class="track-points">
                    {t.points
                        .iter()
                        .map(|key| locale::t(key))
                        .map(|point| view! { <li>{point}</li> })
                        .collect_view()}
                </ul>
                <p class="track-tech">{stack}</p>
            </div>
        </li>
    }
}

#[component]
pub fn Track(content: PortfolioContent) -> impl IntoView {
    let academy = content.academy;
    let roles = content.roles;
    let courses = content.courses;
    let languages = content.languages;

    view! {
        <>
            <SheetHead index=3 view_key="sheets.track.view" numeric_meta=true/>
            <div class="p-body">
                <ol class="track-list">
                    {move || roles.iter().enumerate().map(|(i, t)| role(i, *t)).collect_view()}
                </ol>

                <div class="blk-grid foot-grid">
                    <section class="blk" aria-label=move || locale::t("sheets.track.academy")>
                        <div class="blk-h">
                            <h2 class="blk-name t-head">{academy.institution}</h2>
                            <span class="blk-note t-label num">{move || locale::t(academy.period)}</span>
                        </div>
                        <p class="track-co t-read">
                            {move || locale::t(academy.programme)} " \u{00b7} " {move || locale::t(academy.location)}
                        </p>
                        <ul class="track-points">
                            {move || academy.points
                                .iter()
                                .map(|key| locale::t(key))
                                .map(|point| view! { <li>{point}</li> })
                                .collect_view()}
                        </ul>
                    </section>

                    <section
                        class="blk"
                        aria-label=move || locale::t("sheets.track.courses_languages")
                    >
                        <div class="blk-h">
                            <h2 class="blk-name t-head">{move || locale::t("sheets.track.courses")}</h2>
                            <span class="blk-note t-label">
                                {format!("{:02}", content.courses.len())}
                            </span>
                        </div>
                        <ul class="course-list">
                            {move || courses
                                .iter()
                                .map(|c| {
                                    view! {
                                        <li class="course">
                                            <span class="course-n">{locale::t(c.name)}</span>
                                            <span class="course-i">{locale::t(c.issuer)}</span>
                                            <span class="course-y num">{c.year}</span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                        <p class="lang-line">
                            {move || languages
                                .iter()
                                .map(|key| locale::t(key))
                                .collect::<Vec<_>>()
                                .join(" \u{00b7} ")}
                        </p>
                    </section>
                </div>
            </div>
        </>
    }
}
