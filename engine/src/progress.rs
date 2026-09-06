//! Which cards are stamped.

use crate::lessons::{self, Quest, Street, QUESTS};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default)]
pub struct Save {
    pub cleared: BTreeSet<String>,
}

impl Save {
    fn key(qid: &str, sid: &str) -> String {
        format!("{qid}/{sid}")
    }

    pub fn is_clear(&self, qid: &str, sid: &str) -> bool {
        self.cleared.contains(&Self::key(qid, sid))
    }

    pub fn quest_cleared(&self, q: &Quest) -> bool {
        q.streets.iter().all(|s| self.is_clear(q.id, s.id))
    }

    pub fn streets_cleared(&self) -> usize {
        self.cleared.len()
    }

    pub fn total_streets(&self) -> usize {
        lessons::street_count()
    }

    pub fn mark_clear(&mut self, q: &Quest, s: &Street) {
        self.cleared.insert(Self::key(q.id, s.id));
    }

    pub fn encode(&self) -> String {
        self.cleared.iter().cloned().collect::<Vec<_>>().join(",")
    }

    pub fn decode(raw: &str) -> Self {
        let mut s = Save::default();
        for item in raw.split(',') {
            if !item.is_empty() {
                s.cleared.insert(item.to_string());
            }
        }
        s
    }

    pub fn next_open(&self) -> Option<(&'static Quest, &'static Street)> {
        for q in QUESTS {
            for s in q.streets {
                if !self.is_clear(q.id, s.id) {
                    return Some((q, s));
                }
            }
        }
        None
    }
}
