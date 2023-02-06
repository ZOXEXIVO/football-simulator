import {Injectable} from "@angular/core";
import {Observable, of, Subject} from "rxjs";
import {MatchDto, MatchService, ObjectPositionDto} from "./match.api.service";
import {BallModel, MatchModel, PlayerModel} from "../play/models/models";

@Injectable({
  providedIn: 'root',
})
export class MatchDataService {
  leagueSlug: string = '';
  matchId: string = '';

  offset = 0;
  limit = 300;

  public matchData: MatchModel = new MatchModel();
  lastLoadedTimestamp = 0;

  constructor(private matchService: MatchService) {

  }

  init(leagueSlug: string, matchId: string): Observable<any> {
    this.leagueSlug = leagueSlug;
    this.matchId = matchId;

    return this.loadData();
  }

  loadData(): Observable<any> {
    const subject = new Subject<any>();

    this.matchService.get(this.leagueSlug, this.matchId, this.offset, this.limit).subscribe(matchData => {
      this.updateMatchData(matchData);

      this.offset += this.limit;

      subject.next({});
    });

    return subject.asObservable();
  }

  updateMatchData(matchData: MatchDto) {
    if (this.matchData.ball.data.length == 0) {
      // ball
      this.matchData.ball = new BallModel(matchData.ball_data
        .map(data => new ObjectPositionDto(data[0], data[1], data[2]))
      );

      // players
      for (const [playerId, data] of Object.entries(matchData.player_data)) {
        let playerData = data as number[][];
        this.matchData.players.push(new PlayerModel(Number(playerId), playerData.map(dt => new ObjectPositionDto(dt[0], dt[1], dt[2]))));
      }

      if (matchData.ball_data.length > 0) {
        this.lastLoadedTimestamp = matchData.ball_data[matchData.ball_data.length - 1][0]
      }
    } else {
      // ball
      this.matchData.ball.data.push(...matchData.ball_data.map(data => new ObjectPositionDto(data[0], data[1], data[2])));

      // players
      for (const playerData of this.matchData.players) {
        const newPlayerData = matchData.player_data.get(playerData.id);
        if (newPlayerData) {
          playerData.data.push(...newPlayerData.map(data => new ObjectPositionDto(data[0], data[1], data[2])));
        }
      }

      if (matchData.ball_data.length > 0) {
        this.lastLoadedTimestamp = matchData.ball_data[matchData.ball_data.length - 1][0];
      }
    }
  }

  getData(timestamp: number): Observable<MatchDataResultModel> {
    // ball
    let ts = -1;
    while (ts < timestamp) {
      let taVal = this.matchData.ball.data[this.matchData.ball.currentCoordIdx];
      if(taVal){
        ts = taVal.timestamp;
      }
      this.matchData.ball.currentCoordIdx++;
    }

    const ballResult = this.matchData.ball.data[this.matchData.ball.currentCoordIdx];

    // players
    let playerResults = [];

    for (const player of this.matchData.players) {
      let pts = -1;
      while (pts  < timestamp) {
        let ptsVal = player.data[player.currentCoordIdx];
        if(ptsVal){
          pts = ptsVal.timestamp;
        }
        player.currentCoordIdx++;
      }

      playerResults.push(new PlayerDataResultModel(player.id, player.data[player.currentCoordIdx]));
    }

    return of(new MatchDataResultModel(playerResults, ballResult));
  }
}

export class MatchDataResultModel {
  constructor(players: PlayerDataResultModel[], ball: ObjectPositionDto) {
    this.players = players;
    this.ball = ball;
  }

  public players: PlayerDataResultModel[];
  public ball: ObjectPositionDto;
}

export class PlayerDataResultModel {
  constructor(playerId: number, position: ObjectPositionDto) {
    this.playerId = playerId;
    this.position = position;
  }

  public playerId: number;
  public position: ObjectPositionDto;
}
