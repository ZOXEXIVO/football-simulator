import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";

@Injectable({
   providedIn: 'root',
})
export class PlayerService {
   constructor(private http: HttpClient) {
   }

   get(team_slug: String, playerId: number): Observable<PlayerDto> {
      return this.http.get<PlayerDto>('/api/teams/' + team_slug + '/players/' + playerId);
   }
}


export interface PlayerDto {
   id: number,
   first_name: string
   last_name: string
   middle_name: string
   position: string
   contract: PlayerContractDto,
   birth_date: String,
   age: number,
   team_slug: string
   team_name: string
   country_slug: string,
   country_code: string
   country_name: string
   skills: PlayerSkillsDto,
   conditions: number,
   current_ability: number,
   potential_ability: number,

   value: string

   preferred_foot: string

   player_attributes: PlayerAttributesDto,

   neighbor_teams: ClubTeam[],

   statistics: PlayerStatistics,

   status: PlayerStatusDto,
}

export interface PlayerStatistics {
    played: number,
    played_subs: number,

    goals: number,
    assists: number,
    penalties: number,
    player_of_the_match: number,
    yellow_cards: number,
    red_cards: number,

    shots_on_target: number,
    tackling: number,
    passes: number,

    average_rating: number,
}

export interface ClubTeam {
    name: string,
    slug: string,
    reputation: number,
}

export interface PlayerContractDto {
    salary: number,
    expiration: String,
    squad_status: String,
}


export interface PlayerSkillsDto {
    technical: TechnicalDto,
    mental: MentalDto,
    physical: PhysicalDto,
}

export interface TechnicalDto {
    corners: number,
    crossing: number,
    dribbling: number,
    finishing: number,
    first_touch: number,
    free_kick_taking: number,
    heading: number,
    long_shots: number,
    long_throws: number,
    marking: number,
    passing: number,
    penalty_taking: number,
    tackling: number,
    technique: number,
}

export interface MentalDto {
    aggression: number,
    anticipation: number,
    bravery: number,
    composure: number,
    concentration: number,
    decisions: number,
    determination: number,
    flair: number,
    leadership: number,
    off_the_ball: number,
    positioning: number,
    teamwork: number,
    vision: number,
    work_rate: number,
}

export interface PhysicalDto {
    acceleration: number,
    agility: number,
    balance: number,
    jumping_reach: number,
    natural_fitness: number,
    pace: number,
    stamina: number,
    strength: number,

    match_readiness: number,
}

export interface PlayerAttributesDto {
    international_apps: number,
    international_goals: number,

    under_21_international_apps: number,
    under_21_international_goals: number,
}

export interface PlayerStatusDto {
    statuses: string[]
}