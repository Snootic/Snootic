use std::time::Duration;

use leptos::leptos_dom::helpers::{
    request_animation_frame_with_handle, set_timeout_with_handle, AnimationFrameRequestHandle,
    TimeoutHandle,
};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

use crate::components::nav::{Nav, TURN_MS};
use crate::cube::{self, Cube, Edge, Mat3, Triad};
use crate::libs::browser::{now_ms, reduced_motion};
use crate::libs::locale;
use crate::structs::{sheet_of_face, Direction, Face};

type Drag = StoredValue<Option<(i32, f64, f64, f64, f64)>>;

const ORBIT_PER_PX: f64 = 0.0075;
const REST_MS: u64 = 2600;
/// A drag shorter than this is a tap, and the face patches own taps.
const TAP_SLOP: f64 = 3.0;

#[derive(Debug, Clone, PartialEq)]
struct CxFace {
    visible: bool,
    style: String,
    active: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct CxView {
    edges: [Edge; 12],
    hatch: String,
    target: Option<String>,
    section: Option<String>,
    triad: [Triad; 3],
    faces: [CxFace; 6],
}

impl CxView {
    fn resolve(cube: Cube, preview: Option<Face>, record_open: bool) -> CxView {
        let patches = cube.patches();
        let active = cube.active();
        let faces = Face::ALL.map(|face| {
            let patch = patches.iter().find(|p| p.face == face);
            CxFace {
                visible: patch.is_some(),
                style: patch
                    .map(|p| {
                        format!(
                            "left:{:.2}%;top:{:.2}%",
                            p.cx / cube::BOX * 100.0,
                            p.cy / cube::BOX * 100.0
                        )
                    })
                    .unwrap_or_default(),
                active: face == active,
            }
        });
        CxView {
            edges: cube.edges_toward(preview),
            hatch: cube.clip_path(active),
            target: preview
                .filter(|f| cube.is_front(*f) && *f != active)
                .map(|f| cube.clip_path(f)),
            section: record_open.then(|| cube.section()),
            triad: cube.triad(),
            faces,
        }
    }
}

fn ease(k: f32) -> f32 {
    1.0 - (1.0 - k).powi(3)
}

fn turn_frame(
    nav: Nav,
    from: Mat3,
    to: Mat3,
    started: f64,
    pending: StoredValue<Option<AnimationFrameRequestHandle>>,
) {
    let handle = request_animation_frame_with_handle(move || {
        if pending.is_disposed() {
            return;
        }
        pending.set_value(None);
        let k = ((now_ms() - started) / TURN_MS).clamp(0.0, 1.0) as f32;
        nav.orient.set(cube::slerp(from, to, ease(k)));
        if k < 1.0 {
            turn_frame(nav, from, to, started, pending);
        }
    });
    pending.set_value(handle.ok());
}

fn begin_drag(ev: web_sys::PointerEvent, nav: Nav, drag: Drag) {
    if ev.button() != 0 || drag.get_value().is_some() {
        return;
    }
    ev.prevent_default();
    if let Some(node) = ev
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    {
        let _ = node.set_pointer_capture(ev.pointer_id());
    }
    drag.set_value(Some((
        ev.pointer_id(),
        ev.client_x() as f64,
        ev.client_y() as f64,
        0.0,
        0.0,
    )));
    nav.preview.set(None);
    nav.wake();
}

#[component]
pub fn Instrument(nav: Nav) -> impl IntoView {
    let rest_timer = StoredValue::new(None::<TimeoutHandle>);
    let turn_frame_handle = StoredValue::new(None::<AnimationFrameRequestHandle>);
    on_cleanup(move || {
        if let Some(handle) = rest_timer.get_value() {
            handle.clear();
        }
        if let Some(handle) = turn_frame_handle.get_value() {
            handle.cancel();
        }
    });

    Effect::new(move |_| {
        let _ = nav.orient.get();
        if let Some(handle) = rest_timer.get_value() {
            handle.clear();
        }
        nav.rest.set(false);
        let handle = set_timeout_with_handle(
            move || {
                if !rest_timer.is_disposed() {
                    rest_timer.set_value(None);
                    nav.rest.set(true);
                }
            },
            Duration::from_millis(REST_MS),
        );
        rest_timer.set_value(handle.ok());
    });

    Effect::new(move |_| {
        let goal = nav.goal.get();
        if let Some(handle) = turn_frame_handle.get_value() {
            handle.cancel();
            turn_frame_handle.set_value(None);
        }
        let from = nav.orient.get_untracked();
        if cube::same(from, goal) {
            return;
        }
        if reduced_motion() {
            nav.orient.set(goal);
            return;
        }
        turn_frame(nav, from, goal, now_ms(), turn_frame_handle);
    });

    let view = Memo::new(move |_| {
        CxView::resolve(
            Cube::new(nav.orient.get()),
            nav.preview.get(),
            nav.open.get().is_some(),
        )
    });

    let drag = StoredValue::new(None::<(i32, f64, f64, f64, f64)>);

    let clack = RwSignal::new(false);

    let down = move |ev: web_sys::PointerEvent| begin_drag(ev, nav, drag);

    let mv = move |ev: web_sys::PointerEvent| {
        let Some((id, lx, ly, ax, ay)) = drag.get_value() else {
            return;
        };
        if id != ev.pointer_id() {
            return;
        }
        let (x, y) = (ev.client_x() as f64, ev.client_y() as f64);
        let (dx, dy) = (x - lx, y - ly);
        drag.set_value(Some((id, x, y, ax + dx, ay + dy)));
        let m = cube::mul(
            cube::mul(
                cube::rot_y((dx * ORBIT_PER_PX) as f32),
                cube::rot_x((-dy * ORBIT_PER_PX) as f32),
            ),
            nav.orient.get_untracked(),
        );
        nav.orient.set(m);
        nav.goal.set(m);
        nav.wake();
    };

    let up = move |ev: web_sys::PointerEvent| {
        let Some((id, _, _, ax, ay)) = drag.get_value() else {
            return;
        };
        if id != ev.pointer_id() {
            return;
        }
        drag.set_value(None);
        if ax.abs() < TAP_SLOP && ay.abs() < TAP_SLOP {
            clack.set(true);
            return;
        }
        let dir = if ax.abs() >= ay.abs() {
            if ax > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if ay > 0.0 {
            Direction::Down
        } else {
            Direction::Up
        };
        let face = Cube::new(nav.orient.get_untracked()).active();
        nav.face_to(face, dir);
    };

    let cancel = move |_: web_sys::PointerEvent| {
        drag.set_value(None);
        nav.aim(Cube::new(nav.orient.get_untracked()).active());
    };

    view! {
        <div class="dock" id="dock">
            <div
                class="cube"
                id="cube"
                role="group"
                tabindex="0"
                data-state=move || if nav.rest.get() { "rest" } else { "live" }
                data-clack=move || if clack.get() { "1" } else { "0" }
                on:animationend=move |ev: leptos::web_sys::AnimationEvent| {
                    let on_self = ev
                        .target()
                        .and_then(|t| t.dyn_into::<leptos::web_sys::Element>().ok())
                        .map(|e| e.id() == "cube")
                        .unwrap_or(false);
                    if on_self {
                        clack.set(false);
                    }
                }
                aria-label=move || locale::t("instrument.cube.label")
                on:pointerdown=down
                on:pointermove=mv
                on:pointerup=up
                on:pointercancel=cancel
            >
                <span class="brk brk-tl" aria-hidden="true"></span>
                <span class="brk brk-tr" aria-hidden="true"></span>
                <span class="brk brk-bl" aria-hidden="true"></span>
                <span class="brk brk-br" aria-hidden="true"></span>

                <span
                    class="cface cface-hatch js-only"
                    aria-hidden="true"
                    style=move || format!("clip-path:{}", view.get().hatch)
                ></span>
                <span
                    class="cface cface-target js-only"
                    aria-hidden="true"
                    hidden=move || view.get().target.is_none()
                    style=move || {
                        view.get().target.map(|t| format!("clip-path:{t}")).unwrap_or_default()
                    }
                ></span>

                <svg class="cube-svg" viewBox="0 0 156 156" aria-hidden="true" focusable="false">
                    {(0..12)
                        .map(|i| {
                            view! {
                                <path
                                    d=move || {
                                        let e = view.get().edges[i];
                                        cube::line_d(e.from, e.to)
                                    }
                                    class=move || {
                                        let e = view.get().edges[i];
                                        if e.target {
                                            "e-edge e-tgt"
                                        } else if e.hidden {
                                            "e-hid"
                                        } else if e.active {
                                            "e-edge e-act"
                                        } else {
                                            "e-edge"
                                        }
                                    }
                                />
                            }
                        })
                        .collect_view()}
                    <path
                        class="e-into"
                        hidden=move || view.get().section.is_none()
                        d=move || view.get().section.clone().unwrap_or_default()
                    />
                </svg>

                <div class="cube-patches js-only">
                    {Face::ALL
                        .iter()
                        .enumerate()
                        .map(|(i, face)| {
                            let face = *face;
                            let sheet = sheet_of_face(face);
                            view! {
                                <button
                                    class=move || {
                                        if sheet.is_none() { "cf cf-void" } else { "cf" }
                                    }
                                    type="button"
                                    data-face=face.key()
                                    hidden=move || !view.get().faces[i].visible
                                    style=move || view.get().faces[i].style.clone()
                                    aria-current=move || {
                                        if nav.booted.get() && view.get().faces[i].active {
                                            "true"
                                        } else {
                                            "false"
                                        }
                                    }
                                    aria-label=move || match sheet {
                                        Some(s) => {
                                            format!(
                                                "{} {} {} — {}",
                                                locale::t("instrument.patch.sheet"),
                                                s.no,
                                                locale::t(s.title),
                                                locale::t("instrument.patch.go"),
                                            )
                                        }
                                        None => locale::t("instrument.patch.void"),
                                    }
                                    on:click=move |_| {
                                        clack.set(true);
                                        nav.face_to(face, Direction::Left)
                                    }
                                >
                                    <span class="cf-no num">{face.no()}</span>
                                    <span class="cf-name" hidden=move || !view.get().faces[i].active>
                                        {move || match sheet {
                                            Some(s) => locale::t(s.title),
                                            None => "\u{2014}".to_string(),
                                        }}
                                    </span>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            <p class="dock-hint" aria-hidden="true">
                <span class="dock-dot"></span>
                {move || locale::t("instrument.cube.hint")}
            </p>
            <div class="triad-box">
                <svg class="triad" viewBox="0 0 100 100" aria-hidden="true" focusable="false">
                    {(0..3)
                        .map(|i| {
                            let parts = move || {
                                let t = view.get().triad[i];
                                let (tx, ty) = (50.0 + 30.0 * t.x, 50.0 + 30.0 * t.y);
                                let (bx, by) = (50.0 + 22.0 * t.x, 50.0 + 22.0 * t.y);
                                let (px, py) = (-t.y * 4.0, t.x * 4.0);
                                (t.collapsed, tx, ty, bx, by, px, py)
                            };
                            view! {
                                <path
                                    class="ax"
                                    hidden=move || parts().0
                                    d=move || {
                                        let (_, tx, ty, ..) = parts();
                                        format!("M50 50L{tx:.1} {ty:.1}")
                                    }
                                />
                                <path
                                    class="ah"
                                    fill="currentColor"
                                    hidden=move || parts().0
                                    d=move || {
                                        let (_, tx, ty, bx, by, px, py) = parts();
                                        format!(
                                            "M{tx:.1} {ty:.1}L{:.1} {:.1}L{:.1} {:.1}Z",
                                            bx + px,
                                            by + py,
                                            bx - px,
                                            by - py,
                                        )
                                    }
                                />
                            }
                        })
                        .collect_view()}
                    <circle
                        cx="50"
                        cy="50"
                        r="6"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1"
                        hidden=move || !view.get().triad.iter().any(|a| a.collapsed && a.toward)
                    />
                    <path
                        d="M45 45L55 55M55 45L45 55"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1"
                        hidden=move || {
                            !view.get().triad.iter().any(|a| a.collapsed && !a.toward)
                        }
                    />
                </svg>
                {(0..3)
                    .map(|i| {
                        view! {
                            <span
                                class="ax-lab"
                                style=move || {
                                    let t = view.get().triad[i];
                                    format!(
                                        "left:{:.0}%;top:{:.0}%",
                                        50.0 + 38.0 * t.x,
                                        50.0 + 38.0 * t.y
                                    )
                                }
                            >
                                {move || locale::t(view.get().triad[i].label)}
                            </span>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
