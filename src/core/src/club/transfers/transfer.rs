use crate::shared::CurrencyValue;

#[derive(Debug)]
pub struct TransferItem {
    pub player_id: u32,
    pub salary: CurrencyValue,
}

impl TransferItem {
    pub fn new(player_id: u32, salary: CurrencyValue) -> TransferItem {
        TransferItem { player_id, salary }
    }
}
