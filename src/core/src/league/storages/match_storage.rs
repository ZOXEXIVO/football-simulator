use std::collections::HashMap;
use crate::r#match::MatchResult;

#[derive(Debug)]
pub struct MatchStorage {
    results: HashMap<String, MatchResult>
}

impl MatchStorage {
    pub fn new() -> Self {
        MatchStorage {
            results: HashMap::new()
        }
    }

    pub fn push(&mut self, match_result: MatchResult) {
        self.results.insert(match_result.id.clone(), match_result);
    }

    pub fn get(&self, match_id: String) -> Option<&MatchResult>{
        self.results.get(&match_id)
    }
}

