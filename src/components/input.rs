use leptos::__reexports::send_wrapper::SendWrapper;
use leptos::prelude::*;
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use leptos::web_sys;

use crate::components::nav::Nav;
use crate::libs::browser::{now_ms, window_opt};
use crate::structs::{sheet_index, Direction};

const WHEEL_STEP: f64 = 60.0;

const WHEEL_LOCK_MS: f64 = 420.0;

const SWIPE_STEP: f64 = 90.0;

fn listen<E: JsCast + leptos::wasm_bindgen::convert::FromWasmAbi + 'static>(
    target: &web_sys::EventTarget,
    event: &str,
    cb: impl FnMut(E) + 'static,
) {
    let closure = Closure::<dyn FnMut(E)>::new(cb);
    let _ = target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref());
    let target = SendWrapper::new(target.clone());
    let event = event.to_owned();
    let closure = SendWrapper::new(closure);
    on_cleanup(move || {
        let _ =
            target.remove_event_listener_with_callback(&event, closure.as_ref().unchecked_ref());
    });
}

pub fn mount(nav: Nav, frame_ref: NodeRef<leptos::html::Div>) {
    if let Some(win) = window_opt() {
        listen(&win, "hashchange", move |_: web_sys::Event| {
            let hash = window_opt()
                .and_then(|w| w.location().hash().ok())
                .unwrap_or_default();
            let id = hash.trim_start_matches('#').to_string();
            if let Some(i) = sheet_index(&id) {
                nav.dismiss_record();
                nav.go(i, Direction::Left);
            } else if let Some(project) = nav.content().project(&id) {
                nav.open_record(project.id);
            } else {
                nav.dismiss_record();
                if id.is_empty() {
                    nav.go(0, Direction::Left);
                }
            }
        });

        listen(&win, "resize", move |_: web_sys::Event| {
            nav.plot.update(|t| *t += 1);
        });

        keyboard(nav, &win);
    }

    Effect::new(move |ran: Option<bool>| {
        if ran == Some(true) {
            return true;
        }
        let Some(frame) = frame_ref.get() else {
            return false;
        };
        wheel(nav, &frame);
        swipe(nav, &frame);
        true
    });
}

fn keyboard(nav: Nav, target: &web_sys::EventTarget) {
    listen(target, "keydown", move |ev: web_sys::KeyboardEvent| {
        if ev.default_prevented() || ev.ctrl_key() || ev.meta_key() || ev.alt_key() {
            return;
        }
        let key = ev.key();
        if key == "Escape" {
            nav.escape();
            ev.prevent_default();
            return;
        }
        if nav.open.get_untracked().is_some() {
            return;
        }
        let target = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        let within = |sel: &str| {
            target
                .as_ref()
                .and_then(|n| n.closest(sel).ok().flatten())
                .is_some()
        };
        if within("input, select, textarea, [contenteditable]:not([contenteditable=\"false\"])") {
            return;
        }
        if (key == "Enter" || key == " ") && within("a, button, summary") {
            return;
        }
        match key.as_str() {
            "ArrowLeft" => nav.preview_dir(Direction::Left),
            "ArrowRight" => nav.preview_dir(Direction::Right),
            "ArrowUp" => nav.preview_dir(Direction::Up),
            "ArrowDown" => nav.preview_dir(Direction::Down),
            "Enter" | " " => nav.commit(),
            "m" | "M" => nav.toggle_motion(),
            "1" | "2" | "3" | "4" | "5" => {
                if let Ok(n) = key.parse::<usize>() {
                    nav.go(n - 1, Direction::Left);
                }
            }
            _ => return,
        }
        ev.prevent_default();
    });
}

fn wheel(nav: Nav, target: &web_sys::EventTarget) {
    let acc = StoredValue::new(0.0f64);
    let lock = StoredValue::new(0.0f64);
    listen(target, "wheel", move |ev: web_sys::WheelEvent| {
        let scrolls = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|n| n.closest(".p-body, .popup-body").ok().flatten())
            .is_some_and(|n| n.scroll_height() - n.client_height() > 4);
        if nav.open.get_untracked().is_some() || scrolls {
            return;
        }
        let d = if ev.delta_y().abs() >= ev.delta_x().abs() {
            ev.delta_y()
        } else {
            ev.delta_x()
        };
        if d == 0.0 {
            return;
        }
        ev.prevent_default();
        let t = now_ms();
        if t < lock.get_value() {
            acc.set_value(0.0);
            return;
        }
        let total = acc.get_value() + d;
        if total.abs() >= WHEEL_STEP {
            nav.step(total > 0.0);
            acc.set_value(0.0);
            lock.set_value(t + WHEEL_LOCK_MS);
        } else {
            acc.set_value(total);
        }
    });
}

fn swipe(nav: Nav, target: &web_sys::EventTarget) {
    let drag = StoredValue::new(None::<(i32, f64, f64, f64, f64)>);
    listen(target, "pointerdown", move |ev: web_sys::PointerEvent| {
        if ev.pointer_type() != "touch" || nav.open.get_untracked().is_some() {
            return;
        }
        drag.set_value(Some((
            ev.pointer_id(),
            ev.client_x() as f64,
            ev.client_y() as f64,
            0.0,
            0.0,
        )));
    });
    let Some(win) = window_opt() else {
        return;
    };
    listen(&win, "pointermove", move |ev: web_sys::PointerEvent| {
        let Some((id, lx, ly, ax, ay)) = drag.get_value() else {
            return;
        };
        if id != ev.pointer_id() {
            return;
        }
        let (x, y) = (ev.client_x() as f64, ev.client_y() as f64);
        let (mut ax, ay) = (ax + (x - lx), ay + (y - ly));
        if ax.abs() > ay.abs() * 1.4 {
            while ax.abs() >= SWIPE_STEP {
                nav.step(ax < 0.0);
                ax -= ax.signum() * SWIPE_STEP;
            }
        }
        drag.set_value(Some((id, x, y, ax, ay)));
    });
    let end = move |ev: web_sys::PointerEvent| {
        if drag
            .get_value()
            .is_some_and(|(id, ..)| id == ev.pointer_id())
        {
            drag.set_value(None);
        }
    };
    listen(&win, "pointerup", end);
    listen(&win, "pointercancel", end);
}
