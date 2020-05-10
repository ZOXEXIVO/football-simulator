use chrono::NaiveDate;

#[derive(Debug)]
pub struct ClubSponsorship {
    pub sponsorship_contracts: Vec<ClubSponsorshipContract>
}

impl ClubSponsorship {
    pub fn new(contracts: Vec<ClubSponsorshipContract>) -> Self {
        ClubSponsorship {
            sponsorship_contracts: contracts
        }
    }

    fn remove_expired_contracts(&mut self, date: NaiveDate) {
        self.sponsorship_contracts.retain(|contract| !contract.is_expired(date))
    }
    
    pub fn get_sponsorship_incomes(&mut self, date: NaiveDate) -> Vec<(String, u32)> {
        self.remove_expired_contracts(date);
        
        let mut result = Vec::with_capacity(self.sponsorship_contracts.len());
        
        for contract in &self.sponsorship_contracts{
            result.push((contract.sponsor_name.clone(), contract.wage));
        }
        
        result
    }
}


#[derive(Debug)]
pub struct ClubSponsorshipContract {
    sponsor_name: String,
    wage: u32,
    expiration: NaiveDate
}

impl ClubSponsorshipContract {
    pub fn new(sponsor_name: String, wage: u32, expiration: NaiveDate) -> Self {
        ClubSponsorshipContract {
            sponsor_name,
            wage,
            expiration  
        }
    }
    
    pub fn is_expired(&self, date: NaiveDate) -> bool {
        self.expiration >= date
    }
}
