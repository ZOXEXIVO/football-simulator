use crate::r#match::ball::events::BallUpdateEvent;
use crate::r#match::player::events::PlayerUpdateEvent;

pub enum Event {
    BallEvent(BallUpdateEvent),
    PlayerEvent(PlayerUpdateEvent),
}

pub struct EventCollection {
    pub events: Vec<Event>,
}

impl EventCollection {
    pub fn new() -> Self {
        EventCollection { events: Vec::with_capacity(100) }
    }

    pub fn add(&mut self, event: Event) {
        self.events.push(event)
    }

    pub fn add_range(&mut self, events: impl Iterator<Item = Event>) {
        self.events.append(events)
    }
}

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch(&mut self, events: Vec<Event>) {
        for event in events {
            match event {
                Event::BallEvent(ball_event) => self.handle_ball_event(ball_event),
                Event::PlayerEvent(player_event) => self.handle_player_event(player_event),
            }
        }
    }
}
