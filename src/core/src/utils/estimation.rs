use std::time::Instant;

pub struct TimeEstimation;

impl TimeEstimation {
    pub fn estimate<T, F: FnOnce() -> T>(action: F) -> (T, u32) {
        let now = Instant::now();

        let result = action();

        (result, now.elapsed().as_millis() as u32)
    }
}
