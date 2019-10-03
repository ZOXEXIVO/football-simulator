use crate::models::*;

pub trait Visitor {
   fn visitClub(club: &Club);
   fn visitCountry(country: &Country);
   fn visitLeague(langue: &League);
   fn visitPlayer(player: &Player);
}
