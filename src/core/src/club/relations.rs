use std::collections::{HashSet, BTreeMap};

#[derive(Debug)]
pub struct Relations { 
    players: BTreeMap<u32, RelationType>,
    staffs: BTreeMap<u32, RelationType>
}

#[derive(Debug, PartialEq)]
pub enum RelationType{
    Favorite,
    Hate
}

impl Relations {
    pub fn new() -> Self {
        Relations{
            players: BTreeMap::new(),
            staffs: BTreeMap::new()
        }
    }

    pub fn is_favorite_player(&self, player_id: u32) -> bool {
        self.contains_player_relation_type(player_id, RelationType::Favorite)
    }

    pub fn is_favorite_staff(&self, staff_id: u32) -> bool {
        self.contains_staff_relation_type(staff_id, RelationType::Favorite)
    }
    
    fn contains_player_relation_type(&self, player_id: u32, relation_type: RelationType) -> bool {
        if let Some(rel_type) = self.players.get(&player_id){
            return *rel_type == relation_type
        }

        false
    }
    
    fn contains_staff_relation_type(&self, staff_id: u32, relation_type: RelationType) -> bool {
        if let Some(rel_type) = self.staffs.get(&staff_id){
            return *rel_type == relation_type
        }
        
        false
    }
    
    pub fn add_player_relation_type(&mut self, player_id: u32, relation_type: RelationType) {
        self.players.insert(player_id, relation_type);
    }

    pub fn add_staff_relation_type(&mut self, staff_id: u32, relation_type: RelationType) {
        self.staffs.insert(staff_id, relation_type);
    }
}