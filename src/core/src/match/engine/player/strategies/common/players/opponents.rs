use crate::r#match::{MatchPlayer, StateProcessingContext};
use crate::PlayerFieldPositionGroup;

pub struct PlayerOpponentsOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl<'b> PlayerOpponentsOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        PlayerOpponentsOperationsImpl { ctx }
    }
}

impl<'b> PlayerOpponentsOperationsImpl<'b> {
    pub fn all(&'b self) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.opponents_for_team(self.ctx.player.team_id, None)
    }

    pub fn with_ball(&'b self) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.opponents_for_team(self.ctx.player.team_id, Some(true))
    }

    pub fn without_ball(&'b self) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.opponents_for_team(self.ctx.player.team_id, Some(false))
    }

    pub fn nearby(&'b self, distance: f32) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .opponents(self.ctx.player, distance)
            .map(|(pid, _)| self.ctx.context.players.get(pid).unwrap())
    }

    pub fn nearby_raw(&'b self, distance: f32) -> impl Iterator<Item = (u32, f32)> + 'b {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .opponents(self.ctx.player, distance)
    }

    pub fn exists(&self, distance: f32) -> bool {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .opponents(self.ctx.player, distance)
            .any(|_| true)
    }

    pub fn goalkeeper(&'b self) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.opponents_by_position(
            PlayerFieldPositionGroup::Goalkeeper,
            self.ctx.player.team_id,
        )
    }

    fn opponents_by_position(
        &'b self,
        position_group: PlayerFieldPositionGroup,
        team_id: u32,
    ) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| {
                player.team_id != team_id
                    && player.tactics_position.position_group() == position_group
            })
    }

    fn opponents_for_team(
        &'b self,
        team_id: u32,
        has_ball: Option<bool>,
    ) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| {
                player.team_id != team_id
                    && (has_ball.is_none() || player.has_ball == has_ball.unwrap())
            })
    }
}
