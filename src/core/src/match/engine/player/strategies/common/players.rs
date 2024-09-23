use crate::PlayerFieldPositionGroup;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerDistanceFromStartPosition, PlayerSide,
    StateProcessingContext,
};

pub struct PlayerOperationsImpl<'p> {
    ctx: &'p StateProcessingContext<'p>,
}

impl<'p> PlayerOperationsImpl<'p> {
    pub fn new(ctx: &'p StateProcessingContext<'p>) -> Self {
        PlayerOperationsImpl { ctx }
    }
}

impl<'p> PlayerOperationsImpl<'p> {
    pub fn on_own_side(&self) -> bool {
        let field_half_width = self.ctx.context.field_size.width / 2;

        if let Some(side) = self.ctx.player.side {
            return side == PlayerSide::Left && self.ctx.player.position.x < field_half_width as f32;
        }

        false
    }

    pub fn distance_from_start_position(&self) -> f32 {
        self.ctx
            .player
            .start_position
            .distance_to(&self.ctx.player.position)
    }

    pub fn position_to_distance(&self) -> PlayerDistanceFromStartPosition {
        MatchPlayerLogic::distance_to_start_position(&self.ctx.player)
    }

    pub fn is_tired(&self) -> bool {
        self.ctx.player.player_attributes.condition_percentage() > 50
    }

    pub fn distances(&self) -> (usize, usize) {
        self.ctx
            .tick_context
            .objects_positions
            .player_distances
            .players_within_distance_count(self.ctx.player, 10.0)
    }

    pub fn is_team_loosing(&self) -> bool {
        if self.ctx.player.team_id == self.ctx.context.result.score.home_team.team_id {
            self.ctx.context.result.score.home_team < self.ctx.context.result.score.away_team
        } else {
            self.ctx.context.result.score.away_team < self.ctx.context.result.score.home_team
        }
    }

    pub fn calculate_pass_power(&self, teammate: &MatchPlayer) -> f64 {
        let distance = self.ctx.player.position.distance_to(&teammate.position);
        let pass_skill = self.ctx.player.skills.technical.passing;
        (distance / pass_skill as f32 * 10.0) as f64
    }

    pub fn is_under_pressure(&self, ctx: &StateProcessingContext) -> bool {
        let (_, opponents_count) = self.distances();
        opponents_count > 1
    }

    pub fn opponents(&self) -> Vec<&MatchPlayer> {
        self.ctx.context.players.get_by_not_team(self.ctx.player.team_id)
    }

    pub fn defenders(&self) -> Vec<&MatchPlayer> {
        self.ctx.context.players.get_by_position(PlayerFieldPositionGroup::Defender)
    }
}

pub struct MatchPlayerLogic;

impl MatchPlayerLogic {
    pub fn find_leader(
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
    ) -> u32 {
        let mut leader_id = 0;
        let mut highest_leadership = 0.0;

        for player_position in &objects_positions.players_positions.items {
            let player = context.players.get(player_position.player_id).unwrap();
            let leadership_skill = player.skills.mental.leadership;

            if leadership_skill > highest_leadership {
                highest_leadership = leadership_skill;
                leader_id = player_position.player_id;
            }
        }

        leader_id
    }

    pub fn distance_to_start_position(player: &MatchPlayer) -> PlayerDistanceFromStartPosition {
        let start_position_distance = player.position.distance_to(&player.start_position);

        if start_position_distance < 50.0 {
            PlayerDistanceFromStartPosition::Small
        } else if start_position_distance < 100.0 {
            PlayerDistanceFromStartPosition::Medium
        } else {
            PlayerDistanceFromStartPosition::Big
        }
    }
}
