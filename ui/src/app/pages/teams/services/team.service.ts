import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";

@Injectable({
   providedIn: 'root',
})
export class TeamService {
   constructor(private http: HttpClient) {
   }

   get(slug: String): Observable<TeamDto> {
      return this.http.get<TeamDto>('/api/teams/' + slug);
   }

   getSchedule(slug: String): Observable<TeamScheduleDto> {
      return this.http.get<TeamScheduleDto>('/api/teams/' + slug + '/schedule');
   }
}

export interface TeamDto {
   name: string,
   slug: String,
   league_slug: String,
   league_name: String,
   players: TeamPlayerDto[],
   neighbor_teams: ClubTeamDto[]
}

export interface TeamPlayerDto {
   id: number,
   last_name: String,
   first_name: String,
   behaviour: String,
   position: String,
   position_sort: String,
   value: String,

   injured: boolean,
   country_slug: String,
   country_code: String,
   country_name: String,


   conditions: number,
   current_ability: number,
   potential_ability: number,
   status: PlayerStatusDto,
}

export interface PlayerStatusDto {
   statuses: PlayerStatusItemDto[]
}

export interface PlayerStatusItemDto {
   statuses: String[]
}

export interface ClubTeamDto {
   slug: String,
   name: String,

   reputation: number,
}

// Schedule

export interface TeamScheduleDto {
   team_name: String,
   team_slug: String,
   league_slug: String,
   league_name: String,
   neighbor_teams: ClubTeamDto[],
   items: TeamScheduleItemDto[]
}

export interface TeamScheduleItemDto {
   date: String,
   time: String,
   opponent_slug: String,
   opponent_name: String,
   is_home: boolean,
   competition_id: number,
   competition_name: String,
   result: TeamScheduleItemResultDto
}


export interface TeamScheduleItemResultDto {
   match_id: string,
   home_goals: number,
   away_goals: number
}
