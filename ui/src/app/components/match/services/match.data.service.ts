import {HttpClient} from "@angular/common/http";
import {Injectable} from "@angular/core";
import {Observable, Subject} from "rxjs";
import {BallModel, MatchModel, PlayerModel} from "../play/models";

@Injectable({
  providedIn: 'root',
})
export class MatchDataService {
  leagueSlug: string = '';
  matchId: string = '';

  offset = 0;
  limit = 300;

  matchData: MatchModel = new MatchModel();

  constructor(private matchService: MatchService) {

  }

  init(leagueSlug: string, matchId: string): Observable<any> {
    const subject = new Subject<any>();

    this.leagueSlug = leagueSlug;
    this.matchId = matchId;

    this.matchService.get(this.leagueSlug, this.matchId, this.offset, this.limit).subscribe(matchData => {
      this.updateMatchData(matchData);

      this.offset += this.limit;

      subject.next({});
    });

    return subject.asObservable();
  }

  updateMatchData(matchData: MatchDto){
    if (this.matchData.ball.data.length == 0) {
      // ball
      const ball = new BallModel();
      ball.data = matchData.ball_data;
      this.matchData.ball = ball;

      // players
      for (const [playerId, playerData] of matchData.player_data) {
        const player = new PlayerModel(playerId);
        player.data = playerData;
        this.matchData.players.push(player);
      }

    } else {
      // ball
      this.matchData.ball.data.push(...matchData.ball_data);

      // players
      for (const playerData of this.matchData.players) {
        const newPlayerData = matchData.player_data.get(playerData.id);
        if (newPlayerData) {
          playerData.data.push(...newPlayerData);
        }
      }
    }
  }
}

// Api Service
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
