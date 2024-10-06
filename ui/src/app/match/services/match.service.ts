import {HttpClient} from "@angular/common/http";
import {Injectable} from "@angular/core";
import {Observable} from "rxjs";
import {Container} from "pixi.js";

@Injectable({
    providedIn: 'root',
})
export class MatchService {
    constructor(private http: HttpClient) {
    }

    get(league_slug: string, match_id: string): Observable<MatchDto> {
        return this.http.get<MatchDto>(`/api/match/${league_slug}/${match_id}`);
    }

    data(league_slug: string, match_id: string): Observable<MatchDataDto> {
        return this.http.get<MatchDataDto>(`/api/match/${league_slug}/${match_id}/data`);
    }
}

export interface MatchDataDto {
    player_positions: Map<number, ObjectPositionDto[]>,
    ball_positions: ObjectPositionDto[]
}

export class ObjectPositionDto {
    constructor(timestamp: number, position: number[]) {
        this.timestamp = timestamp;
        this.position = position;
    }

    timestamp: number;
    position: number[];
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

    match_time_ms: number,

    players: MatchPlayerDto[]
    ball: MatchBallDto
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
    displayName: string;
    position: string
    team_slug: string
    start_position: number[],
    is_home: boolean

    obj: Container,
    currentCoordIdx: number
}

export class MatchBallDto {
    public obj: Container | null;
    public currentCoordIdx: number;

    constructor() {
        this.obj = null;
        this.currentCoordIdx = 0;
    }
}

