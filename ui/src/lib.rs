mod nav;
mod persist;

use leptos::prelude::*;
use nav::{href, Page};
use persist::{clear, load, persist};
use workplace_sim::lessons::{self, Quest, QUESTS};
use workplace_sim::progress::Save;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let save = RwSignal::new(load());
    let page = nav::track();
    provide_context(save);
    view! {
        <style>{include_str!("../style.css")}</style>
        <main class="wrap">
            {move || match page.get() {
                Page::Title => view! { <Title/> }.into_any(),
                Page::Map => view! { <MapPage/> }.into_any(),
                Page::Play { qid, sid } => view! { <Play qid=qid sid=sid/> }.into_any(),
                Page::Win => view! { <Win/> }.into_any(),
                Page::Missing => view! { <Missing/> }.into_any(),
            }}
        </main>
    }
}

fn use_save() -> RwSignal<Save> {
    expect_context::<RwSignal<Save>>()
}

fn pct(save: &Save) -> u32 {
    let tot = save.total_streets().max(1);
    ((save.streets_cleared() * 100) / tot) as u32
}

fn reset_progress(save: RwSignal<Save>) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(w) = web_sys::window() {
            if !w
                .confirm_with_message("Clear all progress? This cannot be undone.")
                .unwrap_or(false)
            {
                return;
            }
        }
    }
    clear();
    save.set(Save::default());
}

#[component]
fn Hud() -> impl IntoView {
    let save = use_save();
    view! {
        <div class="hud-bar">
            <div class="brand"><a href=href("/")>"TOXIC COMPANY"</a></div>
            <div class="hud">
                {move || {
                    let s = save.get();
                    format!("{}%  ·  {} / {}", pct(&s), s.streets_cleared(), s.total_streets())
                }}
            </div>
            <div class="hud-actions">
                <a href=href("/") class="btn gold">"TITLE"</a>
                <a href=href("/map") class="btn gold">"CONTENTS"</a>
            </div>
        </div>
        <div class="bar">
            <i style=move || format!("width:{}%", pct(&save.get()))></i>
        </div>
    }
}

#[component]
fn Title() -> impl IntoView {
    let save = use_save();
    let next = move || {
        save.get()
            .next_open()
            .map(|(q, s)| href(&format!("/play/{}/{}", q.id, s.id)))
            .unwrap_or_else(|| href("/win"))
    };
    view! {
        <Hud/>
        <div class="title-hero">
            <div class="ribbon">"WHS COURSE"</div>
            <h1 class="big">"TOXIC COMPANY"</h1>
            <p>"Overt narcissist. Covert narcissist. Corporate psychopath."</p>
        </div>
        <div class="card">
            <p>"Start learning about overt narcissists, covert narcissists, and corporate psychopaths at work."</p>
            <div class="row">
                <a href=next class="btn gold">"START"</a>
                <a href=href("/map") class="btn">"CONTENTS"</a>
                {move || (pct(&save.get()) > 0).then(|| view! {
                    <button class="ghost" type="button" on:click=move |_| reset_progress(save)>"Clear progress"</button>
                })}
            </div>
        </div>
        <p class="foot">"Educational. Not legal advice. If you are unsafe, leave first."</p>
    }
}

#[component]
fn MapPage() -> impl IntoView {
    let save = use_save();
    view! {
        <Hud/>
        <h1>"CONTENTS"</h1>
        <p class="dim">"Three types. Seven cards each."</p>
        <div class="mods-grid">
            {QUESTS.iter().map(|q| view! { <ModuleCard q=*q save/> }).collect_view()}
        </div>
        <p class="foot"><a href=href("/")>"TITLE"</a></p>
    }
}

#[component]
fn ModuleCard(q: Quest, save: RwSignal<Save>) -> impl IntoView {
    let first = q
        .streets
        .first()
        .map(|s| href(&format!("/play/{}/{}", q.id, s.id)))
        .unwrap_or_else(|| href("/map"));
    let mood = match q.id {
        "overt" => "overt",
        "covert" => "covert",
        "corp" => "corp",
        _ => "",
    };
    view! {
        <div class=move || {
            let done = save.get().quest_cleared(&q);
            format!("person-card type-{mood}{}", if done { " done" } else { "" })
        }>
            <span class=format!("type-pill {mood}")>{q.tag}</span>
            <h2>{q.name}</h2>
            <p class="dim">{q.goal}</p>
            <div class="dots">
                {q.streets.iter().map(move |s| {
                    let h = href(&format!("/play/{}/{}", q.id, s.id));
                    let qid = q.id;
                    let sid = s.id;
                    view! {
                        <a href=h class=move || {
                            if save.get().is_clear(qid, sid) { "dot on" } else { "dot" }
                        } title=s.title></a>
                    }
                }).collect_view()}
            </div>
            <div class="pc-foot">
                <span class="dim">{move || {
                    let n = q.streets.iter().filter(|s| save.get().is_clear(q.id, s.id)).count();
                    format!("{n} / {}", q.streets.len())
                }}</span>
                <a href=first class="btn gold">"OPEN"</a>
            </div>
        </div>
    }
}

#[component]
fn Play(qid: String, sid: String) -> impl IntoView {
    let save = use_save();
    let qid = RwSignal::new(qid);
    let sid = RwSignal::new(sid);
    let flipped = RwSignal::new(false);
    let show_stamp = RwSignal::new(false);

    Effect::new(move |_| {
        qid.get();
        sid.get();
        flipped.set(false);
        show_stamp.set(false);
    });

    let stamp = move |_| {
        let Some((_, _, q, s)) = lessons::find_street(&qid.get(), &sid.get()) else {
            return;
        };
        if !flipped.get() && !save.get().is_clear(q.id, s.id) {
            flipped.set(true);
            return;
        }
        save.update(|sv| {
            sv.mark_clear(q, s);
            persist(sv);
        });
        show_stamp.set(true);
    };

    view! {
        <Hud/>
        {move || show_stamp.get().then(|| view! { <StampFx/> })}
        {move || match lessons::find_street(&qid.get(), &sid.get()) {
            None => view! { <Missing/> }.into_any(),
            Some((qi, si, q, s)) => {
                let prev = if si > 0 {
                    Some(href(&format!("/play/{}/{}", q.id, q.streets[si - 1].id)))
                } else if qi > 0 {
                    let pq = &QUESTS[qi - 1];
                    pq.streets.last().map(|ps| href(&format!("/play/{}/{}", pq.id, ps.id)))
                } else {
                    None
                };
                let next = lessons::next_street(qi, si)
                    .map(|(nq, ns)| href(&format!("/play/{}/{}", nq.id, ns.id)))
                    .unwrap_or_else(|| href("/win"));
                let done = save.get().is_clear(q.id, s.id);
                view! {
                    <p class="tag">{q.name}"  ·  "{si + 1}" / "{q.streets.len()}</p>
                    <button
                        class=move || if flipped.get() { "lesson-card flipped" } else { "lesson-card" }
                        type="button"
                        on:click=move |_| flipped.update(|v| *v = !*v)
                    >
                        <div class="lc-inner">
                            <div class="lc-face front">
                                <p class="when">{q.name}</p>
                                <p class="face-word">{s.title}</p>
                                <p class="story">{s.scene}</p>
                                <p class="flip-hint">"Click — flip"</p>
                            </div>
                            <div class="lc-face back">
                                <div class="takeaway">
                                    <div class="lab">"WHY IT MATTERS"</div>
                                    <div class="name">{s.title}</div>
                                    <p>{s.teach}</p>
                                    <p class="dim">{s.why}</p>
                                </div>
                                <p class="flip-hint">"Click — front"</p>
                            </div>
                        </div>
                    </button>
                    {move || show_stamp.get().then(|| view! { <div class="flash">"STAMPED"</div> })}
                    <div class="navrow">
                        {if let Some(p) = prev {
                            view! { <a href=p class="btn">"BACK"</a> }.into_any()
                        } else {
                            view! { <a href=href("/map") class="btn">"CONTENTS"</a> }.into_any()
                        }}
                        <div class="row" style="margin:0">
                            <button type="button" on:click=move |_| flipped.update(|v| *v = !*v)>"FLIP"</button>
                            {if done {
                                view! { <a href=next class="btn gold">"NEXT"</a> }.into_any()
                            } else {
                                view! {
                                    <button class="gold" type="button" on:click=stamp>
                                        {move || if flipped.get() { "STAMP" } else { "FLIP FIRST" }}
                                    </button>
                                }.into_any()
                            }}
                        </div>
                    </div>
                }.into_any()
            }
        }}
    }
}

#[component]
fn Win() -> impl IntoView {
    let save = use_save();
    view! {
        <Hud/>
        <div class="card cert">
            <div class="ribbon">"CLEARED"</div>
            <h1 class="big">"TOXIC COMPANY"</h1>
            <p>"Overt. Covert. Corporate psychopath. Named. Not a diagnosis."</p>
            <p class="dim">{move || format!("{} / {} cards.", save.get().streets_cleared(), save.get().total_streets())}</p>
            <div class="row" style="justify-content:center">
                <a href=href("/map") class="btn gold">"CONTENTS"</a>
                <a href=href("/") class="btn">"TITLE"</a>
                <button class="ghost" type="button" on:click=move |_| reset_progress(save)>"Clear progress"</button>
            </div>
        </div>
    }
}

#[component]
fn StampFx() -> impl IntoView {
    let colors = ["#f8d030", "#fcecc8", "#50d8f8", "#f878a8", "#00a844", "#d82800"];
    view! {
        <div class="fanfare" aria-hidden="true">
            <span class="shock"></span>
            <span class="shock s2"></span>
            {(0..14).map(|i| {
                let ang = (i as f32) / 14.0 * std::f32::consts::TAU;
                let dist = 80.0 + (i % 4) as f32 * 16.0;
                let dx = format!("{:.0}px", ang.cos() * dist);
                let dy = format!("{:.0}px", ang.sin() * dist);
                view! {
                    <span class="star" style=format!("--dx:{dx}; --dy:{dy};")></span>
                }
            }).collect_view()}
            {(0..28).map(|i| {
                let left = format!("{}%", (i * 13 + 4) % 100);
                let delay = format!("{:.2}s", (i % 10) as f32 * 0.05);
                let bg = colors[i % colors.len()];
                view! {
                    <span class="confetti" style=format!(
                        "left:{left}; animation-delay:{delay}; background:{bg};"
                    )></span>
                }
            }).collect_view()}
        </div>
        <div class="stamp-banner">"STAMP"</div>
    }
}

#[component]
fn Missing() -> impl IntoView {
    view! {
        <Hud/>
        <h1>"No such card"</h1>
        <p><a href=href("/map")>"CONTENTS"</a></p>
    }
}
