//! Hash paths so the app can live on GitHub Pages (no server rewrite).

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

#[derive(Clone, PartialEq)]
pub enum Page {
    Title,
    Map,
    Play { qid: String, sid: String },
    Win,
    Missing,
}

pub fn href(path: &str) -> String {
    let p = path.trim();
    if p.starts_with('#') {
        p.to_string()
    } else if p.starts_with('/') {
        format!("#{p}")
    } else {
        format!("#/{p}")
    }
}

pub fn current() -> Page {
    parse(&hash())
}

fn hash() -> String {
    window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
}

pub fn parse(hash: &str) -> Page {
    let h = hash.trim_start_matches('#').trim_start_matches('/');
    if h.is_empty() {
        return Page::Title;
    }
    let parts: Vec<&str> = h.split('/').filter(|s| !s.is_empty()).collect();
    match parts.as_slice() {
        ["map"] | ["contents"] => Page::Map,
        ["win"] => Page::Win,
        ["play", q, s] => Page::Play {
            qid: (*q).to_string(),
            sid: (*s).to_string(),
        },
        _ => Page::Missing,
    }
}

pub fn track() -> RwSignal<Page> {
    let page = RwSignal::new(current());
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(w) = window() {
            let p = page;
            let cb = Closure::wrap(Box::new(move || {
                p.set(current());
            }) as Box<dyn FnMut()>);
            let _ = w.add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }
    page
}
