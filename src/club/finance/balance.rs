use crate::club::ClubFinancialBalanceHistory;
use crate::people::NaiveDate;

#[derive(Debug)]
pub struct ClubFinances {
    pub balance: ClubFinancialBalance,
    pub history: ClubFinancialBalanceHistory
}

impl ClubFinances {
    pub fn new(amount: i32) -> Self {
        ClubFinances{
            balance: ClubFinancialBalance::new(amount),
            history: ClubFinancialBalanceHistory::new()
        }
    }

    pub fn start_new_month(&mut self, date: NaiveDate){
        self.history.add(date, self.balance.clone());
        self.balance.clear();
    }
}


#[derive(Debug, Clone)]
pub struct ClubFinancialBalance {
    pub amount: i32,
    pub income: i32,
    pub outcome: i32,
}


impl ClubFinancialBalance {
    pub fn new(amount: i32) -> Self {
        ClubFinancialBalance{
            amount,
            income: 0,
            outcome: 0,
        }
    }
    
    pub fn clear(&mut self){
        self.income = 0;
        self.outcome = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_new_month_is_correct() {
        let mut finances = ClubFinances::new(123);
        
        finances.balance.income = 1;
        finances.balance.outcome = 2;

        let date = NaiveDate::from_ymd(2020, 2, 1);

        finances.start_new_month(date);
        
        let history_result = finances.history.get(date);

        assert!(history_result.is_some());

        assert_eq!(123, finances.balance.amount);
        assert_eq!(0, finances.balance.income);
        assert_eq!(0, finances.balance.outcome);

        assert_eq!(123, history_result.unwrap().amount);
        assert_eq!(1, history_result.unwrap().income);
        assert_eq!(2, history_result.unwrap().outcome);
    }
}
