import {HttpClient} from "@angular/common/http";
import {Injectable} from "@angular/core";
import {Observable} from "rxjs";
import {MatchDataResultModel} from "./match.data.service";

@Injectable({
    providedIn: 'root',
})
export class MatchService {
    constructor(private http: HttpClient) {
    }

    get(league_slug: string, match_id: string): Observable<MatchDto> {
        return this.http.get<MatchDto>(`/api/match/${league_slug}/${match_id}`);
    }

    data(league_slug: string, match_id: string): Observable<MatchDataResultModel> {
        return this.http.get<MatchDataResultModel>(`/api/match/${league_slug}/${match_id}/data`);
    }
}

export interface MatchDataDto {
    player_data: Map<number, number[][]>,
    player_data_len: number,
    ball_data: number[][]
}

export class ObjectPositionDto {
    constructor(timestamp: number, x: number, y: number, z: number) {
        this.timestamp = timestamp;
        this.x = x;
        this.y = y;
        this.z = z;
    }

    timestamp: number;
    x: number;
    y: number;
    z: number;
}

// Lineup

export interface MatchDto {
    home_team_name: string,
    home_team_slug: string,
    home_squad: MatchSquadDto,

    away_team_name: string,
    away_team_slug: string,
    away_squad: MatchSquadDto,

    score: MatchScoreDto,

    match_time_ms: number
}

export interface MatchScoreDto {
    home_goals: number,
    away_goals: number,
}

export interface MatchSquadDto {
    main: MatchPlayerDto[],
    substitutes: MatchPlayerDto[]
}

export interface MatchPlayerDto {
    id: number,
    first_name: string
    last_name: string
    middle_name: string,
    position: string
    team_slug: string
    start_position: number[],
    is_home: boolean
}
