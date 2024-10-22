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
    pub fn is_control_ball(&self) -> bool {
        if let Some(owner_id) = self.ctx.ball().owner_id() {
            if let Some(owner) = self.ctx.context.players.get(owner_id) {
                return self.ctx.player.team_id == owner.team_id;
            }
        }

        false
    }

    // Teamates

    pub fn all(&self) -> Vec<&MatchPlayer> {
        self.teammates_for_team(self.ctx.player.team_id, None)
    }

    pub fn players_with_ball(&self) -> Vec<&MatchPlayer> {
        self.teammates_for_team(self.ctx.player.team_id, Some(true))
    }

    pub fn players_without_ball(&self) -> Vec<&MatchPlayer> {
        self.teammates_for_team(self.ctx.player.team_id, Some(false))
    }

    pub fn forwards(&self) -> Vec<&MatchPlayer> {
        self.teammates_by_position(PlayerFieldPositionGroup::Forward, self.ctx.player.team_id)
    }

    fn teammates_by_position(
        &self,
        position_group: PlayerFieldPositionGroup,
        team_id: u32,
    ) -> Vec<&MatchPlayer> {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(|player| {
                player.team_id == team_id
                    && player.tactics_position.position_group() == position_group
            })
            .collect()
    }

    fn teammates_for_team(&self, team_id: u32, has_ball: Option<bool>) -> Vec<&MatchPlayer> {
        let teammates = self
            .ctx
            .context
            .players
            .players
            .values()
            .filter(|player| player.team_id == team_id);

        if has_ball.is_some() {
            let ball_val = has_ball.unwrap();
            return teammates
                .filter(|player| player.has_ball == ball_val)
                .collect();
        }

        teammates.collect()
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
