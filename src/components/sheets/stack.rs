use super::SheetHead;
use leptos::prelude::*;

use crate::libs::locale;
use crate::structs::PortfolioContent;

#[component]
pub fn Stack(content: PortfolioContent) -> impl IntoView {
    view! {
        <>
            <SheetHead index=2 view_key="sheets.stack.view"/>
            <div class="p-body">
                <p class="legend">
                    <span class="legend-item">
                        <span class="mark" aria-hidden="true"></span>
                        <span class="t-label">
                            {move || locale::t("sheets.stack.legend.written")}
                        </span>
                    </span>
                    <span class="legend-item">
                        <span class="mark mark-none" aria-hidden="true"></span>
                        <span class="t-label">
                            {move || locale::t("sheets.stack.legend.ref")}
                        </span>
                    </span>
                </p>
                <div class="blk-grid">
                    {move || {
                        content
                            .stack
                            .iter()
                            .map(|cell| {
                                view! {
                                    <section class="blk" aria-label={locale::t(cell.category)}>
                                        <div class="blk-h">
                                            <h2 class="blk-name t-head">{locale::t(cell.category)}</h2>
                                            <span class="blk-note t-label">{locale::t(cell.note)}</span>
                                        </div>
                                        <ul class="chip-list">
                                            {cell
                                                .chips
                                                .iter()
                                                .map(|chip| {
                                                    let e = chip.evidence;
                                                    match e.href {
                                                        Some(href) => {
                                                            view! {
                                                                <li>
                                                                    <a class="chip" href=href>
                                                                        <span class="chip-n">{chip.name}</span>
                                                                        <span class="chip-r">{locale::t(e.label)}</span>
                                                                    </a>
                                                                </li>
                                                            }
                                                                .into_any()
                                                        }
                                                        None => {
                                                            view! {
                                                                <li>
                                                                    <span class="chip chip-none">
                                                                        <span class="chip-n">{chip.name}</span>
                                                                        <span class="chip-r">{locale::t(e.label)}</span>
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
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </>
    }
}
