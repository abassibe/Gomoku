use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::*;

use crate::goban::fscore::Fscore;
use crate::goban::Goban;

//Transposition table states, hash map of already encountered board states
lazy_static! {
    static ref TT_STATES: RwLock<HashMap<Goban, Fscore>> = RwLock::new(HashMap::new());
}

/// Returns `Option<Fscore>` if move already was computed before
pub fn tt_lookup_state(key: &Goban) -> Option<Fscore> {
    if TT_STATES.read().unwrap().contains_key(key) {
        tt_get_fscore(key)
    } else {
        None
    }
}

pub fn tt_insert_new_state(new_goban: Goban, new_fscore: Fscore) {
    TT_STATES.write().unwrap().insert(new_goban, new_fscore);
}

fn tt_get_fscore(state: &Goban) -> Option<Fscore> {
    let lock = TT_STATES.read().unwrap();
    lock.get(state).copied()
}