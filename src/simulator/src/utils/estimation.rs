use std::time::{Duration, Instant};

pub fn inspect_duration<F>(action: F) -> u32
where
    F: FnOnce() -> ()
{
    let now = Instant::now();

    action();
    
    now.elapsed().as_millis() as u32
}
