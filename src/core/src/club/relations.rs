use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct Relations { 
    players: RelationStore,
    staffs: RelationStore
}

#[derive(Debug)]
pub struct RelationStore {
    pub data: HashMap<RelationType, BTreeMap<u32, i8>>,
}

impl RelationStore {
    pub fn new() -> Self{
        RelationStore{
            data: HashMap::new()
        }
    }
    
    pub fn contains(&self, relation_type: RelationType, id: u32) -> bool {
        if let Some(tree) = self.data.get(&relation_type) {
            return tree.contains_key(&id);
        }

        false
    }
    
    pub fn add(&mut self, relation_type: RelationType, id: u32, level: i8) {
        let entry = self.data.entry(relation_type)
            .or_insert(BTreeMap::new());
        
        entry.insert(id, level);
    }

    pub fn remove(&mut self, relation_type: RelationType, id: u32) {
        if let Some(mut tree) = self.data.get_mut(&relation_type) {
            tree.remove(&id);
        }
    }
}


#[derive(Debug, PartialEq, Hash, Eq)]
pub enum RelationType{
    Favorite,
    Hate
}

impl Relations {
    pub fn new() -> Self {
        Relations{
            players: RelationStore::new(),
            staffs: RelationStore::new()
        }
    }

    pub fn is_favorite_player(&self, player_id: u32) -> bool {
        self.players.contains(RelationType::Favorite, player_id)
    }

    pub fn add_player_to_favorites(&mut self, player_id: u32, level: i8) {
        self.players.add(RelationType::Favorite, player_id, level);
    }
    
    pub fn is_favorite_staff(&self, staff_id: u32) -> bool {
        self.staffs.contains(RelationType::Favorite, staff_id)
    }

    pub fn add_staff_to_favorites(&mut self, staff_id: u32, level: i8) {
        self.staffs.add(RelationType::Favorite, staff_id, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relations_player_in_favorites_return_true() {
        let mut relations = Relations::new();

        const FAVORITE_PLAYER_ID: u32 = 1;
 
        relations.add_player_to_favorites(FAVORITE_PLAYER_ID, 1i8);

        assert_eq!(true, relations.is_favorite_player(FAVORITE_PLAYER_ID));
        assert_eq!(false, relations.is_favorite_staff(FAVORITE_PLAYER_ID));
    }

    #[test]
    fn relations_staff_in_favorites_return_true() {
        let mut relations = Relations::new();

        const FAVORITE_STAFF_ID: u32 = 3;

        relations.add_staff_to_favorites(FAVORITE_STAFF_ID, 1i8);

        assert_eq!(true, relations.is_favorite_staff(FAVORITE_STAFF_ID));
        assert_eq!(false, relations.is_favorite_player(FAVORITE_STAFF_ID));
    }

    #[test]
    fn relations_player_not_in_favorites_return_true() {
        let mut relations = Relations::new();

        const NOT_FAVORITE_PLAYER_ID: u32 = 2;

        assert_eq!(false, relations.is_favorite_player(NOT_FAVORITE_PLAYER_ID));
    }

    #[test]
    fn relations_staff_not_in_favorites_return_true() {
        let mut relations = Relations::new();

        const FAVORITE_STAFF_ID: u32 = 3;

        assert_eq!(false, relations.is_favorite_staff(FAVORITE_STAFF_ID));
    }
}
