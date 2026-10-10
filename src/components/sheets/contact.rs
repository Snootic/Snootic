use super::SheetHead;
use leptos::prelude::*;

use crate::libs::locale;
use crate::structs::PortfolioContent;

#[component]
pub fn Contact(content: PortfolioContent) -> impl IntoView {
    let p = content.personal.clone();
    let social = content.social;

    view! {
        <>
            <SheetHead index=4 view_key="sheets.contact.view"/>
            <div class="p-body">
                <div class="contact-grid">
                    <section class="blk" aria-label=move || locale::t("sheets.contact.direct")>
                        <div class="blk-h">
                            <h2 class="blk-name t-head">{move || locale::t("sheets.contact.direct")}</h2>
                            <span class="blk-note t-label">{move || locale::t(p.availability)}</span>
                        </div>
                        <a class="mailto" href=format!("mailto:{}", p.email)>
                            {p.email}
                        </a>
                        <span class="contact-line">
                            <span>{move || locale::t("sheets.contact.phone")}</span>
                            <a href=p.phone_href>{p.phone}</a>
                        </span>
                        <span class="contact-line">
                            <span>{move || locale::t("sheets.contact.based_in")}</span>
                            <span>{move || locale::t(p.location)}</span>
                        </span>
                    </section>

                    <section class="blk" aria-label=move || locale::t("sheets.contact.networks")>
                        <div class="blk-h">
                            <h2 class="blk-name t-head">{move || locale::t("sheets.contact.networks")}</h2>
                            <span class="blk-note t-label">
                                {format!("{:02}", social.len())}
                            </span>
                        </div>
                        <ul class="social-list">
                            {social
                                .iter()
                                .map(|s| {
                                    let name = s.name;
                                    let handle = s.handle;
                                    match s.href {
                                        Some(href) => {
                                            view! {
                                                <li>
                                                    <a
                                                        class="social"
                                                        href=href
                                                        target="_blank"
                                                        rel="noopener"
                                                    >
                                                        <span class="social-name">{name}</span>
                                                        <span class="social-handle">{handle}</span>
                                                        <svg class="ico ico-go-l" aria-hidden="true">
                                                            <use href="#dim-arrow"/>
                                                        </svg>
                                                    </a>
                                                </li>
                                            }
                                                .into_any()
                                        }
                                        None => {
                                            view! {
                                                <li>
                                                    <span class="social social-static">
                                                        <span class="social-name">{name}</span>
                                                        <span class="social-handle">{handle}</span>
                                                    </span>
                                                </li>
                                            }
                                                .into_any()
                                        }
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </section>

                    <section
                        class="blk contact-wide"
                        aria-label=move || locale::t("sheets.contact.notes_on_set")
                    >
                        <div class="blk-h">
                            <h2 class="blk-name t-head">{move || locale::t("sheets.contact.notes")}</h2>
                            <span class="blk-note t-label">
                                {move || locale::t("sheets.contact.key")}
                            </span>
                        </div>
                        <ol class="notes">
                            <li>
                                {move || locale::t("sheets.contact.note.ring")}
                            </li>
                            <li>
                                {move || locale::t("sheets.contact.note.evidence_a")}
                                <span class="num">"REF \u{2014}"</span>
                                {move || locale::t("sheets.contact.note.evidence_b")}
                            </li>
                            <li>
                                {move || locale::t("sheets.contact.note.plates")}
                            </li>
                            <li>
                                {move || locale::t("sheets.contact.note.register")}
                                <i>"M"</i>
                                "."
                            </li>
                        </ol>
                    </section>
                </div>
            </div>
        </>
    }
}
