use crate::club::{ClubFinancialBalanceHistory, ClubFinanceResult, ClubSponsorship, ClubSponsorshipContract};
use crate::context::GlobalContext;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct ClubFinances {
    pub balance: ClubFinancialBalance,
    pub history: ClubFinancialBalanceHistory,
    pub sponsorship: ClubSponsorship
}

impl ClubFinances {
    pub fn new(amount: i32, sponsorship_contract: Vec<ClubSponsorshipContract>) -> Self {
        ClubFinances{
            balance: ClubFinancialBalance::new(amount),
            history: ClubFinancialBalanceHistory::new(),
            sponsorship: ClubSponsorship::new(sponsorship_contract)
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> ClubFinanceResult {
        let result = ClubFinanceResult::new();

        if ctx.simulation.is_month_beginning() {
            self.start_new_month(ctx.simulation.date.date())
        }

        if ctx.simulation.is_year_beginning() {
            for (_, wage) in self.sponsorship.get_sponsorship_incomes(ctx.simulation.date.date()) {
                self.balance.push_income(wage)
            }
        }

        result
    }
    
    pub fn push_salary(&mut self, amount: u32){
        self.balance.push_outcome(amount);
    }
    
    fn start_new_month(&mut self, date: NaiveDate){
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
    
    pub fn push_income(&mut self, wage: u32){
        self.amount = self.amount + wage as i32;
        self.income = self.income + wage as i32;
    }

    pub fn push_outcome(&mut self, wage: u32){
        self.amount = self.amount - wage as i32;
        self.outcome = self.outcome + wage as i32;
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
        let mut finances = ClubFinances::new(123, Vec::new());
        
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

    #[test]
    fn balance_push_income_is_correct() {
        let mut finances = ClubFinancialBalance::new(-1);

        finances.amount = 1;
        finances.income = 2;
        finances.outcome = 3;

        finances.push_income(20);

        assert_eq!(21, finances.amount);
        assert_eq!(22, finances.income);
        assert_eq!(3, finances.outcome);
    }

    #[test]
    fn balance_push_outcome_is_correct() {
        let mut finances = ClubFinancialBalance::new(-1);

        finances.amount = 10;
        finances.income = 20;
        finances.outcome = 30;

        finances.push_outcome(5);

        assert_eq!(5, finances.amount);
        assert_eq!(20, finances.income);
        assert_eq!(35, finances.outcome);
    }

    #[test]
    fn balance_clear_is_correct() {
        let mut finances = ClubFinancialBalance::new(-1);

        finances.amount = 1;
        finances.income = 2;
        finances.outcome = 3;

        finances.clear();

        assert_eq!(1, finances.amount);
        assert_eq!(0, finances.income);
        assert_eq!(0, finances.outcome);
    }
}
