use super::SheetHead;
use leptos::prelude::*;

use crate::components::nav::Nav;
use crate::libs::locale;
use crate::structs::{PortfolioContent, Project};

fn card(p: Project, nav: Nav) -> impl IntoView {
    let status = locale::t(p.status());
    let tags = p.tags_line();
    view! {
        <li
            class="work-row"
            class:is-open=move || nav.open.get() == Some(p.id)
            id=format!("row-{}", p.no)
            data-status=if p.featured { "featured" } else { "archive" }
        >
            <span class="w-top">
                <span class="w-idx num" aria-hidden="true">
                    {p.no}
                </span>
                <a
                    class="w-name t-card"
                    id=format!("open-{}", p.no)
                    href=format!("#{}", p.id)
                    aria-haspopup="dialog"
                    aria-label=format!(
                        "{} \u{2014} {}",
                        locale::t(p.title),
                        locale::t("sheets.work.open_record"),
                    )
                >
                    {locale::t(p.title)}
                    <svg class="ico" aria-hidden="true">
                        <use href="#dim-arrow"/>
                    </svg>
                </a>
            </span>
            <span class="w-main">
                <span class="w-desc">{locale::t(p.brief)}</span>
                <span class="w-meta">
                    <span class="w-flag" data-status=if p.featured { "featured" } else { "archive" }>
                        {status}
                    </span>
                    <span class="w-tags">{tags}</span>
                </span>
            </span>
            <span class="w-links">
                {p.source
                    .map(|href| {
                        view! {
                            <a class="wlink" href=href target="_blank" rel="noopener">
                                {locale::t("sheets.work.source")}
                                <svg class="ico" aria-hidden="true">
                                    <use href="#dim-arrow"/>
                                </svg>
                            </a>
                        }
                    })}
                {match p.live {
                    Some(href) => {
                        view! {
                            <a class="wlink" href=href target="_blank" rel="noopener">
                                {locale::t("sheets.work.live")}
                                <svg class="ico" aria-hidden="true">
                                    <use href="#dim-arrow"/>
                                </svg>
                            </a>
                        }
                            .into_any()
                    }
                    None => {
                        view! { <span class="w-void">{locale::t("sheets.work.live_void")}</span> }
                            .into_any()
                    }
                }}
            </span>
        </li>
    }
}

#[component]
pub fn Work(content: PortfolioContent) -> impl IntoView {
    let nav = expect_context::<Nav>();
    let featured: Vec<Project> = content.featured().copied().collect();
    let archive: Vec<Project> = content.archive().copied().collect();

    let f_len = featured.len();
    let a_len = archive.len();

    view! {
        <>
            <SheetHead index=1 view_key="sheets.work.view"/>
            <div class="p-body">
                <ul class="work-list">
                    <li class="work-group">
                        <span class="wg-k">{move || locale::t("sheets.work.featured")}</span>
                        <span class="wg-n num">{move || format!("{:02} {}", f_len, locale::t("sheets.work.records"))}</span>
                    </li>
                    {move || featured.iter().copied().map(|p| card(p, nav)).collect_view()}
                    <li class="work-group is-archive">
                        <span class="wg-k">{move || locale::t("sheets.work.archive")}</span>
                        <span class="wg-n num">{move || format!("{:02} {}", a_len, locale::t("sheets.work.records"))}</span>
                    </li>
                    {move || archive.iter().copied().map(|p| card(p, nav)).collect_view()}
                </ul>
            </div>
        </>
    }
}
