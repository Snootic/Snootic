use super::SheetHead;
use leptos::prelude::*;

use crate::libs::content::CTAS;
use crate::libs::locale;
use crate::structs::PortfolioContent;

#[component]
pub fn Identity(content: PortfolioContent) -> impl IntoView {
    let p = content.personal.clone();
    let stats = content.stats();

    view! {
        <>
            <SheetHead index=0 view_key="sheets.identity.view" numeric_meta=true/>
            <div class="p-body id-body">
                <div class="id-main">
                    <p class="callout">
                        <span class="balloon num" aria-hidden="true">
                            "01"
                        </span>
                        <span class="leader" aria-hidden="true"></span>
                        <span class="callout-note">{move || locale::t("sheets.identity.name_role")}</span>
                    </p>
                    <h1 class="macro t-display">{p.name}</h1>
                    <div class="dim" id="dim" aria-hidden="true">
                        <span class="dim-line"></span>
                        <svg class="ico ico-head ico-head-l">
                            <use href="#arrow-head"/>
                        </svg>
                        <svg class="ico ico-head ico-head-r">
                            <use href="#arrow-head"/>
                        </svg>
                        <span class="dim-val num" id="dim-val">
                            "\u{2014}"
                        </span>
                    </div>
                    <p class="id-role">{move || locale::t(p.title)}</p>
                    <p class="id-tagline t-body">{move || locale::t(p.tagline)}</p>
                    <p class="id-avail">{move || locale::t(p.availability)}</p>
                    <ul class="id-stats">
                        {stats
                            .into_iter()
                            .map(|s| {
                                let label = s.label;
                                view! {
                                    <li>
                                        <span class="id-stat-n num">{s.value}</span>
                                        <span class="t-label">{move || locale::t(label)}</span>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                    <div class="cta-row">
                        <a class="cta cta-go" href=CTAS[0].href>
                            {move || locale::t(CTAS[0].label)}
                        </a>
                        <a class="cta cta-cv" href=p.resume target="_blank" rel="noopener">
                            {move || locale::t("sheets.identity.download_cv")}
                            <svg class="ico" aria-hidden="true">
                                <use href="#dim-arrow"/>
                            </svg>
                        </a>
                    </div>
                </div>

                <figure class="id-fig">
                    <div class="portrait">
                        <img src=p.portrait alt=move || locale::t(p.portrait_alt) loading="lazy" decoding="async"/>
                    </div>
                    <figcaption class="t-label">
                        {move || locale::t("sheets.identity.figure")}
                    </figcaption>
                </figure>

                <section class="bio" aria-labelledby="bio-h">
                    <h2 class="callout" id="bio-h">
                        <span class="balloon num" aria-hidden="true">
                            "02"
                        </span>
                        <span class="leader" aria-hidden="true"></span>
                        <span class="callout-note">{move || locale::t("sheets.identity.brief")}</span>
                    </h2>
                    {move || p.brief
                        .iter()
                        .map(|key| locale::t(key))
                        .map(|para| view! { <p class="t-body">{para}</p> })
                        .collect_view()}
                </section>
            </div>
        </>
    }
}
