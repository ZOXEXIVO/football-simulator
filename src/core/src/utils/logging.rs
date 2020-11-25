use log::{info};
use crate::utils::TimeEstimation;

pub struct Logging;

impl Logging {
    pub fn wrap_call<T, F: FnOnce() -> T>(action: F, message: &str) -> T {
        let (result, estimated) = TimeEstimation::estimate(action);

        info!("{} {}ms", message, estimated);

        result
    }
}
