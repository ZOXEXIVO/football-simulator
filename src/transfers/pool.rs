use std::collections::HashMap;
use std::sync::Mutex;

pub struct TransferPool<T> {
    pool: Mutex<HashMap<u32, Vec<T>>>,
}

impl<T> TransferPool<T> {
    pub fn new() -> Self {
        TransferPool {
            pool: Mutex::new(HashMap::new()),
        }
    }

    pub fn push_transfer(&mut self, item: T, club_id: u32) {
        let mut inner_map = self.pool.lock().unwrap();

        let mut entry = inner_map.entry(club_id).or_insert(Vec::new());

        entry.push(item);
    }

    pub fn pull_transfers(&mut self, club_id: u32) -> Option<Vec<T>> {
        let mut inner_map = self.pool.lock().unwrap();

        if !inner_map.contains_key(&club_id) {
            return None;
        }

        inner_map.remove(&club_id)
    }
}
