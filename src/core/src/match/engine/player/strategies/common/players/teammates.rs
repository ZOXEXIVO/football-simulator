use crate::r#match::{MatchPlayer, StateProcessingContext};
use crate::PlayerFieldPositionGroup;

pub struct PlayerTeammatesOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl<'b> PlayerTeammatesOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        PlayerTeammatesOperationsImpl { ctx }
    }
}

impl<'b> PlayerTeammatesOperationsImpl<'b> {
    pub fn all(&'b self) -> impl Iterator<Item = &'b MatchPlayer> {
        self.teammates_for_team(self.ctx.player.team_id, None)
    }

    pub fn players_with_ball(&self) -> impl Iterator<Item = &MatchPlayer> {
        self.teammates_for_team(self.ctx.player.team_id, Some(true))
    }

    pub fn players_without_ball(&self) -> impl Iterator<Item = &MatchPlayer> {
        self.teammates_for_team(self.ctx.player.team_id, Some(false))
    }

    pub fn forwards(&self) -> impl Iterator<Item = &MatchPlayer> {
        self.teammates_by_position(PlayerFieldPositionGroup::Forward, self.ctx.player.team_id)
    }

    fn teammates_by_position(
        &self,
        position_group: PlayerFieldPositionGroup,
        team_id: u32,
    ) -> impl Iterator<Item = &MatchPlayer> {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| {
                player.team_id == team_id
                    && player.tactics_position.position_group() == position_group
            })
    }

    fn teammates_for_team(&self, team_id: u32, has_ball: Option<bool>) -> impl Iterator<Item = &'b MatchPlayer> {
        let teammates = self
            .ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| player.team_id == team_id && (has_ball.is_none() || player.has_ball == has_ball.unwrap()));

        teammates
    }

    pub fn nearby(&'b self, distance: f32) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .teammates(self.ctx.player, distance)
            .map(|(pid, _)| {
                self.ctx.context.players.get(pid).unwrap()
            })
    }

    pub fn nearby_raw(&'b self, distance: f32) -> impl Iterator<Item = (u32, f32)> + 'b {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .teammates(self.ctx.player, distance)
    }

    pub fn exists(&self, distance: f32) -> bool {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .teammates(self.ctx.player, distance)
            .any(|_| { true })
    }
}
