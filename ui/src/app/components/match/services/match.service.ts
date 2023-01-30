import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { Observable } from "rxjs";

@Injectable({
   providedIn: 'root',
})
export class MatchService {
   constructor(private http: HttpClient) {
   }

   get(league_slug: String, match_id: String, offset: number, limit: number): Observable<MatchDto> {
      return this.http.get<MatchDto>(`/api/match/${league_slug}/${match_id}?offset=${offset}&limit=${limit}`);
   }
}

export interface MatchDto {
    player_data: Map<number, PlayerPositionDto[]>,
    player_data_len: number,
    ball_data: BallPositionDto[],
    home_team_players: number[],
    away_team_players: number[]
}

export interface BallPositionDto {
    timestamp: number,
    x: number,
    y: number
}

export interface PlayerPositionDto {
    timestamp: number,
    x: number,
    y: number
}

export interface PlayerPositionDto {
    timestamp: number,
    x: number,
    y: number
}