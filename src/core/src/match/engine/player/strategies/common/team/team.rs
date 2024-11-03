use crate::r#match::StateProcessingContext;
use crate::Tactics;
use nalgebra::Vector3;

pub struct TeamOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl<'b> TeamOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        TeamOperationsImpl { ctx }
    }
}

impl<'b> TeamOperationsImpl<'b> {
    pub fn tactics(&self) -> Option<Tactics> {
        None
    }

    pub fn is_control_ball(&self) -> bool {
        self.ctx.ball().owner_id() == Some(self.ctx.player.id)
    }

    pub fn is_leading(&self) -> bool {
        let team_score = self.get_home_team_score();
        let opponent_score = self.get_away_score();

        team_score > opponent_score
    }

    pub fn is_loosing(&self) -> bool {
        if self.ctx.player.team_id == self.ctx.context.score.home_team.team_id {
            self.ctx.context.score.home_team < self.ctx.context.score.away_team
        } else {
            self.ctx.context.score.away_team < self.ctx.context.score.home_team
        }
    }

    fn get_home_team_score(&self) -> u8 {
        if self.ctx.player.team_id == self.ctx.context.score.home_team.team_id {
            self.ctx.context.score.home_team.get()
        } else {
            self.ctx.context.score.away_team.get()
        }
    }

    fn get_away_score(&self) -> u8 {
        if self.ctx.player.team_id == self.ctx.context.score.home_team.team_id {
            self.ctx.context.score.away_team.get()
        } else {
            self.ctx.context.score.home_team.get()
        }
    }
}
