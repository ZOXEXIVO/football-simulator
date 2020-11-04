use crate::shared::CurrencyValue;

#[derive(Debug)]
pub struct TransferItem {
    pub player_id: u32,
    pub amount: CurrencyValue,
}

impl TransferItem {
    pub fn new(player_id: u32, amount: CurrencyValue) -> TransferItem {
        TransferItem { player_id, amount }
    }
}
