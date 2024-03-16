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
        if let Some(entry) = self.players.relation_data.get_mut(&id) {
            *entry += increment_level;
        } else {
            self.players.relation_data.insert(id, increment_level);
        }
    }

    pub fn is_favorite_player(&self, player_id: u32) -> bool {
        match self.players.relation_data.get(&player_id) {
            Some(&relation_level) => relation_level >= 10.0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relations_new() {
        let relations = Relations::new();
        assert!(relations.players.relation_data.is_empty());
    }

    #[test]
    fn test_relations_update() {
        let mut relations = Relations::new();
        relations.update(1, 10.0);
        assert_eq!(relations.players.relation_data.get(&1), Some(&10.0));

        // Increment existing relation
        relations.update(1, 5.0);
        assert_eq!(relations.players.relation_data.get(&1), Some(&15.0));
    }

    #[test]
    fn test_relations_get_player() {
        let mut relations = Relations::new();
        relations.update(1, 10.0);
        assert_eq!(relations.get_player(1), Some(10.0));
        assert_eq!(relations.get_player(2), None);
    }

    #[test]
    fn test_relations_is_favorite_player() {
        let mut relations = Relations::new();
        relations.update(1, 10.0);

        assert!(relations.is_favorite_player(1));
        assert!(!relations.is_favorite_player(2));
    }

    #[test]
    fn test_relation_store_new() {
        let relation_store = RelationStore::new();
        assert!(relation_store.relation_data.is_empty());
    }
}