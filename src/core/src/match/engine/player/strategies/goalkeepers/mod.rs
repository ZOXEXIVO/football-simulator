mod decision;
pub mod states;
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateProcessor,
};
