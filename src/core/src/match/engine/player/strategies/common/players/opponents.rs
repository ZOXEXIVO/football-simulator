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
    pub fn all(&self) -> Vec<&MatchPlayer> {
        self.opponents_for_team(self.ctx.player.team_id, None)
    }

    pub fn with_ball(&self) -> Vec<&MatchPlayer> {
        self.opponents_for_team(self.ctx.player.team_id, Some(true))
    }

    pub fn without_ball(&self) -> Vec<&MatchPlayer> {
        self.opponents_for_team(self.ctx.player.team_id, Some(false))
    }

    pub fn nearby(&'b self, distance: f32) -> impl Iterator<Item = &MatchPlayer> + 'b {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .opponents(self.ctx.player, distance)
            .map(|(pid, _)| {
                self.ctx.context.players.get(pid).unwrap()
            })
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
            .any(|_| { true })
    }

    pub fn goalkeeper(&self) -> Vec<&MatchPlayer> {
        self.opponents_by_position(
            PlayerFieldPositionGroup::Goalkeeper,
            self.ctx.player.team_id,
        )
    }

    fn opponents_by_position(
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
                player.team_id != team_id
                    && player.tactics_position.position_group() == position_group
            })
            .collect()
    }

    fn opponents_for_team(&self, team_id: u32, has_ball: Option<bool>) -> Vec<&MatchPlayer> {
        let opponents = self
            .ctx
            .context
            .players
            .players
            .values()
            .filter(|player| player.team_id != team_id);

        if has_ball.is_some() {
            let ball_val = has_ball.unwrap();
            return opponents
                .filter(|player| player.has_ball == ball_val)
                .collect();
        }

        opponents.collect()
    }
}
