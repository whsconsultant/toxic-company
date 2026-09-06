//! Toxic Company: title → contents → flip → stamp.
//! Curriculum is shared with the Leptos WASM UI.

pub mod lessons;
pub mod progress;

pub use lessons::{Quest, Street, QUESTS};
pub use progress::Save;

pub const DEFAULT_BIND: &str = "0.0.0.0:7420";
