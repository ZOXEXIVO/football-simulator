use crate::r#match::{
    MatchPlayer,
    PlayerOpponentsOperationsImpl, PlayerTeammatesOperationsImpl,
    StateProcessingContext,
};
use crate::PlayerFieldPositionGroup;

pub struct PlayersOperationsImpl<'p> {
    ctx: &'p StateProcessingContext<'p>,
}

impl<'p> PlayersOperationsImpl<'p> {
    pub fn new(ctx: &'p StateProcessingContext<'p>) -> Self {
        PlayersOperationsImpl { ctx }
    }
}

impl<'p> PlayersOperationsImpl<'p> {
    // Teammates
    pub fn teammates(&'p self) -> PlayerTeammatesOperationsImpl<'p> {
        PlayerTeammatesOperationsImpl::new(self.ctx)
    }

    // Opponents
    pub fn opponents(&'p self) -> PlayerOpponentsOperationsImpl<'_> {
        PlayerOpponentsOperationsImpl::new(self.ctx)
    }

    // Other

    pub fn defenders(&self) -> Vec<&MatchPlayer> {
        self.get_by_position(PlayerFieldPositionGroup::Defender)
    }

    pub fn forwards(&self) -> Vec<&MatchPlayer> {
        self.get_by_position(PlayerFieldPositionGroup::Forward)
    }

    pub fn raw_players(&self) -> Vec<&MatchPlayer> {
        self.ctx.context.players.players.values().collect()
    }

    pub fn get_by_position(&self, position_group: PlayerFieldPositionGroup) -> Vec<&MatchPlayer> {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(|player| player.tactics_position.position_group() == position_group)
            .collect()
    }
}
