use crate::r#match::{MatchPlayer, StateProcessingContext};
use crate::PlayerFieldPositionGroup;
use std::cmp::Ordering;

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

    pub fn with_ball(&self) -> Option<&&MatchPlayer> {
        self.opponents_for_team(self.ctx.player.team_id, Some(true))
            .first()
    }

    pub fn without_ball(&self) -> Vec<&MatchPlayer> {
        self.opponents_for_team(self.ctx.player.team_id, Some(false))
    }

    pub fn nearby<'a>(&self) -> Option<&'a MatchPlayer> {
        self.nearby_with_distance(300.0)
    }

    pub fn nearby_with_distance<'a>(&self, distance: f32) -> Option<&'a MatchPlayer> {
        let nearest_player = self
            .ctx
            .tick_context
            .object_positions
            .player_distances
            .distances
            .iter()
            .filter(|item| item.distance <= distance)
            .filter(|item| {
                item.distance <= distance
                    && item.player_from_id == self.ctx.player.id
                    && item.player_from_team != item.player_to_team
            })
            .min_by(|a, b| {
                a.distance
                    .partial_cmp(&b.distance)
                    .unwrap_or(Ordering::Equal)
            })
            .map(|item| (item.player_to_id, item.distance));

        if let Some((nearest_player_id, nearest_player_distance)) = nearest_player {
            return self.ctx.context.players.get(nearest_player_id);
        }

        None
    }

    pub fn exists_with_distance(&self, distance: f32) -> bool {
        self.ctx
            .tick_context
            .object_positions
            .player_distances
            .distances
            .iter()
            .filter(|item| item.distance <= distance)
            .any(|item| {
                item.distance <= distance
                    && item.player_from_id == self.ctx.player.id
                    && item.player_from_team != item.player_to_team
            })
    }

    pub fn nearbies(&self, player: &MatchPlayer) -> Option<Vec<(u32, f32)>> {
        let mut opponents: Vec<_> = self
            .ctx
            .tick_context
            .object_positions
            .player_distances
            .distances
            .iter()
            .filter(|item| {
                (item.player_from_id == player.id && item.player_from_team != item.player_to_team)
                    || (item.player_to_id == player.id
                        && item.player_from_team != item.player_to_team)
            })
            .map(|item| {
                if item.player_from_id == player.id {
                    (item.player_to_id, item.distance)
                } else {
                    (item.player_from_id, item.distance)
                }
            })
            .collect();

        opponents.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        if opponents.is_empty() {
            None
        } else {
            Some(opponents)
        }
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
