use crate::r#match::{MatchPlayerLite, StateProcessingContext};
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
    pub fn all(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.teammates_for_team(self.ctx.player.team_id, None)
    }

    pub fn players_with_ball(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.teammates_for_team(self.ctx.player.team_id, Some(true))
    }

    pub fn players_without_ball(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.teammates_for_team(self.ctx.player.team_id, Some(false))
    }

    pub fn forwards(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.teammates_by_position(PlayerFieldPositionGroup::Forward, self.ctx.player.team_id)
    }

    fn teammates_by_position(
        &'b self,
        position_group: PlayerFieldPositionGroup,
        team_id: u32,
    ) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| {
                player.team_id == team_id
                    && player.tactics_position.position_group() == position_group
            })
            .map(|player| MatchPlayerLite {
                id: player.id,
                position: self.ctx.tick_context.player_position(player.id),
            })
    }

    fn teammates_for_team(
        &'b self,
        team_id: u32,
        has_ball: Option<bool>,
    ) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| {
                // opponent
                player.team_id == team_id
                    && (has_ball.is_none()
                        || (self.ctx.ball().owner_id() == Some(self.ctx.player.id)))
            })
            .map(|player| MatchPlayerLite {
                id: player.id,
                position: self.ctx.tick_context.player_position(player.id),
            })
    }

    pub fn nearby(&'b self, distance: f32) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .teammates(self.ctx.player, distance)
            .map(|(pid, _)| MatchPlayerLite {
                id: pid,
                position: self.ctx.tick_context.player_position(pid),
            })
    }

    pub fn nearby_ids(&self, distance: f32) -> impl Iterator<Item = (u32, f32)> + 'b {
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
            .any(|_| true)
    }
}
