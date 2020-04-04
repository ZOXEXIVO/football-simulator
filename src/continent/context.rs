use crate::people::Player;
use crate::transfers::TransferPool;

pub struct ContinentContext {
    pub transfer_pool: TransferPool<Player>,
}

impl ContinentContext {
    pub fn new() -> Self {
        ContinentContext {          
            transfer_pool: TransferPool::new(),
        }
    }
}
