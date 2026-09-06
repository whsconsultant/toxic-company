use workplace_sim::progress::Save;
use web_sys::window;

const KEY: &str = "toxic-company";

pub fn load() -> Save {
    let Some(w) = window() else {
        return Save::default();
    };
    let Ok(Some(store)) = w.local_storage() else {
        return Save::default();
    };
    match store.get_item(KEY) {
        Ok(Some(raw)) => Save::decode(&raw),
        _ => Save::default(),
    }
}

pub fn persist(save: &Save) {
    let Some(w) = window() else {
        return;
    };
    if let Ok(Some(store)) = w.local_storage() {
        let _ = store.set_item(KEY, &save.encode());
    }
}

pub fn clear() {
    let Some(w) = window() else {
        return;
    };
    if let Ok(Some(store)) = w.local_storage() {
        let _ = store.remove_item(KEY);
    }
}
