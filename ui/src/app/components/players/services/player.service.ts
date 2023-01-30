import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";

@Injectable({
   providedIn: 'root',
})
export class PlayerService {
   constructor(private http: HttpClient) {
   }

   get(slug: String): Observable<TeamDto> {
      return this.http.get<TeamDto>('/api/teams/' + slug);
   }

   getSchedule(slug: String): Observable<TeamScheduleDto> {
      return this.http.get<TeamScheduleDto>('/api/teams/' + slug + '/schedule');
   }
}

export interface PlayerDto {
   pub id: u32,
   pub first_name: &'p str,
   pub last_name: &'p str,
   pub middle_name: Option<&'p str>,
   pub position: &'p str,
   pub contract: Option<PlayerContractDto>,
   pub birth_date: String,
   pub age: u8,
   pub team_slug: &'p str,
   pub team_name: &'p str,
   pub country_id: u32,
   pub country_code: &'p str,
   pub country_name: &'p str,
   pub skills: PlayerSkillsDto,
   pub conditions: u8,
   pub current_ability: u8,
   pub potential_ability: u8,

   pub value: &'p str,

   pub preferred_foot: &'p str,

   pub player_attributes: PlayerAttributesDto,

   pub neighbor_teams: Vec<ClubTeam<'p>>,

   pub statistics: PlayerStatistics,

   pub status: PlayerStatusDto,
}

export interface PlayerDto {
   pub id: u32,
   pub first_name: &'p str,
   pub last_name: &'p str,
   pub middle_name: Option<&'p str>,
   pub position: &'p str,
   pub contract: Option<PlayerContractDto>,
   pub birth_date: String,
   pub age: u8,
   pub team_slug: &'p str,
   pub team_name: &'p str,
   pub country_id: u32,
   pub country_code: &'p str,
   pub country_name: &'p str,
   pub skills: PlayerSkillsDto,
   pub conditions: u8,
   pub current_ability: u8,
   pub potential_ability: u8,

   pub value: &'p str,

   pub preferred_foot: &'p str,

   pub player_attributes: PlayerAttributesDto,

   pub neighbor_teams: Vec<ClubTeam<'p>>,

   pub statistics: PlayerStatistics,

   pub status: PlayerStatusDto,
}

#[derive(Serialize)]
pub struct PlayerStatistics {
    pub played: u16,
    pub played_subs: u16,

    pub goals: u16,
    pub assists: u16,
    pub penalties: u16,
    pub player_of_the_match: u8,
    pub yellow_cards: u8,
    pub red_cards: u8,

    pub shots_on_target: f32,
    pub tackling: f32,
    pub passes: u8,

    pub average_rating: f32,
}

#[derive(Serialize)]
pub struct ClubTeam<'c> {
    pub name: &'c str,
    pub slug: &'c str,
    pub reputation: u16,
}

#[derive(Serialize)]
pub struct PlayerContractDto {
    pub salary: u32,
    pub expiration: String,
    pub squad_status: String,
}

#[derive(Serialize)]
pub struct PlayerSkillsDto {
    pub technical: TechnicalDto,
    pub mental: MentalDto,
    pub physical: PhysicalDto,
}

#[derive(Serialize)]
pub struct TechnicalDto {
    pub corners: u8,
    pub crossing: u8,
    pub dribbling: u8,
    pub finishing: u8,
    pub first_touch: u8,
    pub free_kick_taking: u8,
    pub heading: u8,
    pub long_shots: u8,
    pub long_throws: u8,
    pub marking: u8,
    pub passing: u8,
    pub penalty_taking: u8,
    pub tackling: u8,
    pub technique: u8,
}

#[derive(Serialize)]
pub struct MentalDto {
    pub aggression: u8,
    pub anticipation: u8,
    pub bravery: u8,
    pub composure: u8,
    pub concentration: u8,
    pub decisions: u8,
    pub determination: u8,
    pub flair: u8,
    pub leadership: u8,
    pub off_the_ball: u8,
    pub positioning: u8,
    pub teamwork: u8,
    pub vision: u8,
    pub work_rate: u8,
}

#[derive(Serialize)]
pub struct PhysicalDto {
    pub acceleration: u8,
    pub agility: u8,
    pub balance: u8,
    pub jumping_reach: u8,
    pub natural_fitness: u8,
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,

    pub match_readiness: u8,
}

#[derive(Serialize)]
pub struct PlayerAttributesDto {
    pub international_apps: u16,
    pub international_goals: u16,

    pub under_21_international_apps: u16,
    pub under_21_international_goals: u16,
}

#[derive(Serialize)]
pub struct PlayerStatusDto {
    pub statuses: Vec<PlayerStatusType>,
}