use std::time::Instant;

pub struct TimeEstimation;

impl TimeEstimation {
    pub fn estimate<F>(action: F) -> u32
    where
        F: FnOnce() -> (),
    {
        let now = Instant::now();

        action();

        now.elapsed().as_millis() as u32
    }
}
