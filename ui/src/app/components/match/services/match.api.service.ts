import {HttpClient} from "@angular/common/http";
import {Injectable} from "@angular/core";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root',
})
export class MatchService {
  constructor(private http: HttpClient) {
  }

  get(league_slug: string, match_id: string, offset: number, limit: number): Observable<MatchDto> {
    return this.http.get<MatchDto>(`/api/match/${league_slug}/${match_id}?offset=${offset}&limit=${limit}`);
  }
}

export interface MatchDto {
  player_data: Map<number, number[][]>,
  player_data_len: number,
  ball_data: number[][],
  home_team_players: number[],
  away_team_players: number[]
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
