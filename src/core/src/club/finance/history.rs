use crate::club::ClubFinancialBalance;
use std::collections::LinkedList;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct ClubFinancialBalanceHistory{
    history: LinkedList<(NaiveDate, ClubFinancialBalance)>
}

impl ClubFinancialBalanceHistory {
    pub fn new() -> Self{
        ClubFinancialBalanceHistory{
            history: LinkedList::new()
        }
    }
    
    pub fn get(&self, date: NaiveDate) -> Option<&ClubFinancialBalance>{
        for (history_date, item) in self.history.iter() {
            if *history_date == date {
                return Some(item);
            }
        }
            
        None
    }
    
    pub fn add(&mut self, date: NaiveDate, balance: ClubFinancialBalance){
        self.history.push_front((date, balance))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_date_not_found_none() {
        let history = ClubFinancialBalanceHistory::new();

        let date = NaiveDate::from_ymd(2020, 2, 1);
        
        let result = history.get(date);
   
        assert!(result.is_none());
    }

    #[test]
    fn get_date_exist_return_balance() {
        let mut history = ClubFinancialBalanceHistory::new();

        let balance = ClubFinancialBalance::new(123);
        let date = NaiveDate::from_ymd(2020, 2, 1);

        history.add(date, balance);
        
        let result = history.get(date);

        assert!(result.is_some());
        assert_eq!(123, result.unwrap().amount);
    }
}
