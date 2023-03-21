import {HttpClient} from "@angular/common/http";
import {Injectable} from "@angular/core";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root',
})
export class MatchService {
  constructor(private http: HttpClient) {
  }

  lineup(league_slug: string, match_id: string): Observable<MatchLineupDto> {
    return this.http.get<MatchLineupDto>(`/api/match/${league_slug}/${match_id}/lineup`);
  }

  get(league_slug: string, match_id: string, offset: number, limit: number): Observable<MatchDto> {
    return this.http.get<MatchDto>(`/api/match/${league_slug}/${match_id}?offset=${offset}&limit=${limit}`);
  }
}

export interface MatchDto {
  player_data: Map<number, number[][]>,
  player_data_len: number,
  ball_data: number[][]
}

export class ObjectPositionDto {
  constructor(timestamp: number, x: number, y: number) {
    this.timestamp = timestamp;
    this.x = x;
    this.y = y;
  }

  timestamp: number;
  x: number;
  y: number;
}

// Lineup

export interface MatchLineupDto {
  home_team_name: string,
  home_team_slug: string,
  home_squad: MatchLineupSquadDto,

  away_team_name: string,
  away_team_slug: string,
  away_squad: MatchLineupSquadDto,

  ball: MatchLineupBallDto,

  score: MatchLineupScoreDto,

  match_time_ms: number
}

export interface MatchLineupScoreDto {
  home_goals: number,
  away_goals: number,
}

export interface MatchLineupSquadDto {
  main: MatchLineupPlayerDto[],
  substitutes: MatchLineupPlayerDto[]
}

export interface MatchLineupPlayerDto {
   id: number,
   first_name: string
   last_name: string
   middle_name: string,
   position: string
   team_slug: string
   start_position: number[],
   is_home: boolean
}

export interface MatchLineupBallDto {
  start_position: number[]
}
