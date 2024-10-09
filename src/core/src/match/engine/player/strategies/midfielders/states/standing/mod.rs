    use std::sync::LazyLock;
    use nalgebra::Vector3;
    use rand::Rng;
    use crate::common::loader::DefaultNeuralNetworkLoader;
    use crate::common::NeuralNetwork;
    use crate::r#match::{ConditionContext, PlayerSide, StateChangeResult, StateProcessingContext, StateProcessingHandler};
    use crate::r#match::midfielders::states::MidfielderState;

    static MIDFIELDER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
        LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

    const POSSESSION_DISTANCE_THRESHOLD: f32 = 100.0; // Adjust based on simulation scale
    const PASSING_DISTANCE_THRESHOLD: f32 = 30.0; // Adjust as needed
    const PRESSING_DISTANCE_THRESHOLD: f32 = 50.0; // Adjust as needed
    const STAMINA_THRESHOLD: u32 = 20; // Minimum stamina percentage before resting

    #[derive(Default)]
    pub struct MidfielderStandingState {}

    impl StateProcessingHandler for MidfielderStandingState {
        fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
            if ctx.ball().distance() < 10.0 {

                // Transition to Tackling state to attempt to regain possession
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Tackling,
                ));
            }

            // 1. Check if the midfielder has the ball
            if ctx.player.has_ball {
                // Decide whether to hold possession or distribute the ball
                return if self.should_hold_possession(ctx) {
                    Some(StateChangeResult::with_midfielder_state(MidfielderState::HoldingPossession))
                } else {
                    Some(StateChangeResult::with_midfielder_state(MidfielderState::Distributing))
                }
            }

            if ctx.player.player_attributes.condition_percentage() < STAMINA_THRESHOLD {
                // Transition to Resting state
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Resting,
                ));
            }

            // 2. Check if the ball is close and the midfielder should attempt to gain possession
            if ctx.ball().distance() < POSSESSION_DISTANCE_THRESHOLD {
                // Transition to Tackling state to try and win the ball
                return Some(StateChangeResult::with_midfielder_state(MidfielderState::Tackling));
            }

            // 3. Check if an opponent is nearby and pressing is needed
            if self.is_opponent_nearby(ctx) {
                // Transition to Pressing state to apply pressure
                return Some(StateChangeResult::with_midfielder_state(MidfielderState::Pressing));
            }

            // 4. Check if a teammate is making a run and needs support
            if self.should_support_attack(ctx) {
                return Some(StateChangeResult::with_midfielder_state(MidfielderState::SupportingAttack));
            }

            None
        }

        fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
            // Implement neural network logic if necessary
            None
        }

        fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
            let mut rng = rand::thread_rng();

            // Determine if the player should move based on their concentration and positioning
            let should_move = self.calculate_movement_probability(&mut rng, &ctx);

            if should_move {
                // Get player's agility and stamina to adjust movement
                let agility_factor = ctx.player.skills.physical.agility / 20.0; // normalize between 0 and 1
                let stamina_factor = ctx.player.skills.physical.stamina / 20.0; // normalize between 0 and 1

                // Generate random movement direction
                let random_x: f32 = rng.gen_range(-0.5..0.5) * agility_factor * stamina_factor;
                let random_y: f32 = rng.gen_range(-0.5..0.5) * agility_factor * stamina_factor;

                // Random movement vector based on skills
                let random_movement = Vector3::new(random_x, random_y, 0.0);

                // Return random movement
                Some(random_movement)
            } else {
                // Player remains stationary
                Some(Vector3::new(0.0, 0.0, 0.0))
            }
        }

        fn process_conditions(&self, _ctx: ConditionContext) {
            // No additional conditions
        }
    }

    impl MidfielderStandingState {
        /// Checks if the midfielder should hold possession based on game context.
        fn should_hold_possession(&self, ctx: &StateProcessingContext) -> bool {
            // For simplicity, let's assume the midfielder holds possession if there are no immediate passing options
            !self.has_passing_options(ctx)
        }

        fn calculate_movement_probability(&self, rng: &mut impl Rng, ctx: &StateProcessingContext) -> bool {
            let positioning_probability = (ctx.player.skills.mental.positioning as f64) / 20.0;
            let concentration_probability = (ctx.player.skills.mental.concentration as f64) / 20.0;

            let positioning_roll = rng.gen_range(0.0..1.0);
            let concentration_roll = rng.gen_range(0.0..1.0);

            positioning_roll < positioning_probability && concentration_roll < concentration_probability
        }

        /// Determines if the midfielder has passing options.
        fn has_passing_options(&self, ctx: &StateProcessingContext) -> bool {
            const PASSING_DISTANCE_THRESHOLD: f32 = 30.0; // Distance within which a teammate is considered available for a pass

            ctx.context.players.raw_players().iter()
                .filter(|p| p.team_id == ctx.player.team_id && p.id != ctx.player.id)
                .any(|teammate| {
                    let distance = (ctx.player.position - teammate.position).magnitude();
                    distance < PASSING_DISTANCE_THRESHOLD
                })
        }

        const PRESSING_DISTANCE_THRESHOLD: f32 = 10.0;

        /// Checks if an opponent player is nearby within the pressing threshold.
        fn is_opponent_nearby(&self, ctx: &StateProcessingContext) -> bool {
            let (_, opponents_count) = ctx.tick_context.object_positions.player_distances.
                players_within_distance_count(ctx.player, PRESSING_DISTANCE_THRESHOLD);

            opponents_count > 0
        }

        /// Determines if the midfielder should support an attacking play.
        fn should_support_attack(&self, ctx: &StateProcessingContext) -> bool {
            // For simplicity, assume the midfielder supports the attack if the ball is in the attacking third
            let field_length = ctx.context.field_size.width as f32;
            let attacking_third_start = if ctx.player.side == Some(PlayerSide::Left) {
                field_length * (2.0 / 3.0)
            } else {
                field_length / 3.0
            };

            let ball_position_x = ctx.tick_context.object_positions.ball_position.x;

            if ctx.player.side == Some(PlayerSide::Left){
                ball_position_x > attacking_third_start
            } else {
                ball_position_x < attacking_third_start
            }
        }
    }
