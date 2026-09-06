use workplace_sim::lessons::{self, QUESTS};
use workplace_sim::progress::Save;

#[test]
fn twenty_one_cards() {
    assert_eq!(lessons::street_count(), 21);
    assert_eq!(QUESTS.len(), 3);
    for q in QUESTS {
        assert_eq!(q.streets.len(), 7, "{}", q.id);
    }
}

#[test]
fn stamp_once() {
    let mut save = Save::default();
    let q = &QUESTS[0];
    save.mark_clear(q, &q.streets[0]);
    assert!(save.is_clear(q.id, q.streets[0].id));
    assert_eq!(save.streets_cleared(), 1);
    save.mark_clear(q, &q.streets[0]);
    assert_eq!(save.streets_cleared(), 1);
}

#[test]
fn save_roundtrip() {
    let mut save = Save::default();
    save.cleared.insert("overt/credit".into());
    let back = Save::decode(&save.encode());
    assert!(back.is_clear("overt", "credit"));
}

#[test]
fn find_and_next() {
    let Some((qi, si, q, s)) = lessons::find_street("covert", "victim") else {
        panic!("missing card");
    };
    assert_eq!(q.id, "covert");
    assert_eq!(s.id, "victim");
    let Some((nq, ns)) = lessons::next_street(qi, si) else {
        panic!("missing next");
    };
    assert_eq!(nq.id, "covert");
    assert_eq!(ns.id, "silent");
}
