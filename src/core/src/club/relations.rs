use std::collections::HashMap;

#[derive(Debug)]
pub struct Relations {
    players: RelationStore,
    _staffs: RelationStore,
}

impl Relations {
    pub fn new() -> Self {
        Relations {
            players: RelationStore::new(),
            _staffs: RelationStore::new(),
        }
    }

    pub fn get_player(&self, id: u32) -> Option<f32> {
        if let Some(&tree) = self.players.relation_data.get(&id) {
            return Some(tree);
        }

        None
    }

    pub fn update(&mut self, id: u32, increment_level: f32) {
        let entry = self
            .players
            .relation_data
            .entry(id)
            .or_insert(increment_level);

        *entry += increment_level;
    }

    pub fn is_favorite_player(&self, player_id: u32) -> bool {
        match self.players.relation_data.get(&player_id) {
            Some(&relation_level) => relation_level > 10.0,
            None => false,
        }
    }
}

#[derive(Debug)]
pub struct RelationStore {
    pub relation_data: HashMap<u32, f32>,
}

impl RelationStore {
    pub fn new() -> Self {
        RelationStore {
            relation_data: HashMap::new(),
        }
    }
}
