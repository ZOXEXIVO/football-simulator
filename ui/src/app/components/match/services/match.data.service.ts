import {Injectable} from "@angular/core";
import {Observable, of, Subject, switchMap} from "rxjs";
import {MatchDto, MatchService, ObjectPositionDto} from "./match.api.service";
import {BallModel, MatchModel, PlayerModel, SquadPlayerModel} from "../play/models/models";

@Injectable({
  providedIn: 'root',
})
export class MatchDataService {
  leagueSlug: string = '';
  matchId: string = '';

  offset = 0;
  limit = 100;

  public matchData: MatchModel = new MatchModel();
  lastLoadedTimestamp = 0;

  constructor(private matchService: MatchService) {

  }

  init(leagueSlug: string, matchId: string): Observable<any> {
    this.leagueSlug = leagueSlug;
    this.matchId = matchId;

    const subject = new Subject<any>();

    this.matchService.lineup(this.leagueSlug, this.matchId).subscribe(matchLineupData => {
      // setup ball
      this.matchData.ball = new BallModel([
        new ObjectPositionDto(0,
          matchLineupData.ball.start_position[0],
          matchLineupData.ball.start_position[1]
        )
      ]);

      // setup players
      for (const player of matchLineupData.home_squad.main) {
        let playerPosition = new ObjectPositionDto(0,
          player.start_position[0],
          player.start_position[1]
        );

        this.matchData.players.push(new PlayerModel(player.id, true, [playerPosition]));
      }

      for (const player of matchLineupData.away_squad.main) {
        let playerPosition = new ObjectPositionDto(0,
          player.start_position[0],
          player.start_position[1]
        );

        this.matchData.players.push(new PlayerModel(player.id, false, [playerPosition]));
      }

      // Squad

      // Home
      for (const homeSquadPlayer of matchLineupData.home_squad.main) {
        this.matchData.squad.home.push(new SquadPlayerModel(
          homeSquadPlayer.id,
          homeSquadPlayer.first_name,
          homeSquadPlayer.last_name,
          homeSquadPlayer.middle_name,
          homeSquadPlayer.position,
          homeSquadPlayer.team_slug
        ));
      }

      // Home subs
      for (const homeSubsSquadPlayer of matchLineupData.home_squad.substitutes) {
        this.matchData.squad.home_subs.push(new SquadPlayerModel(
          homeSubsSquadPlayer.id,
          homeSubsSquadPlayer.first_name,
          homeSubsSquadPlayer.last_name,
          homeSubsSquadPlayer.middle_name,
          homeSubsSquadPlayer.position,
          homeSubsSquadPlayer.team_slug
        ));
      }

      // Away

      for (const awaySquadPlayer of matchLineupData.away_squad.main) {
        this.matchData.squad.away.push(new SquadPlayerModel(
          awaySquadPlayer.id,
          awaySquadPlayer.first_name,
          awaySquadPlayer.last_name,
          awaySquadPlayer.middle_name,
          awaySquadPlayer.position,
          awaySquadPlayer.team_slug
        ));
      }

      for (const awaySubsSquadPlayer of matchLineupData.away_squad.substitutes) {
        this.matchData.squad.away.push(new SquadPlayerModel(
          awaySubsSquadPlayer.id,
          awaySubsSquadPlayer.first_name,
          awaySubsSquadPlayer.last_name,
          awaySubsSquadPlayer.middle_name,
          awaySubsSquadPlayer.position,
          awaySubsSquadPlayer.team_slug
        ));
      }

      subject.next({});
    });

    return subject.asObservable();
  }

  updateMatchData(matchDtaDto: MatchDto) {
    // ball
    this.matchData.ball.data.push(...matchDtaDto.ball_data.map(data => new ObjectPositionDto(data[0], data[1], data[2])));

    // players
    for (const playerData of this.matchData.players) {
      for (const [playerId, data] of Object.entries(matchDtaDto.player_data)) {
        if (playerData.id != Number(playerId)) {
          let newPlayerData = data as number[][];
          playerData.data.push(...newPlayerData.map(pd => new ObjectPositionDto(pd[0], pd[1], pd[2])));
        }
      }

    }

    if (matchDtaDto.ball_data.length > 0) {
      this.lastLoadedTimestamp = matchDtaDto.ball_data[matchDtaDto.ball_data.length - 1][0];
    }
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

  getData(timestamp: number): Observable<MatchDataResultModel> {
    //console.log('getData: try load data for timestamp: ' + timestamp + ' lastLoadedTimestamp: ' + this.lastLoadedTimestamp);

    // Check if data for the requested timestamp has not been loaded yet
    if (this.lastLoadedTimestamp < timestamp) {
      return this.loadData().pipe(
        switchMap(() => {
          return this.getData(timestamp);
        })
      );
    } else {
      // ball
      let ts = -1;
      while (ts < timestamp && this.matchData.ball.currentCoordIdx < this.matchData.ball.data.length) {
        ts = this.matchData.ball.data[this.matchData.ball.currentCoordIdx].timestamp;
        this.matchData.ball.currentCoordIdx++;
      }

      // players
      let playerResults = [];

      for (const player of this.matchData.players) {
        let pts = -1;
        while (pts < timestamp && player.currentCoordIdx < player.data.length) {
          pts = player.data[player.currentCoordIdx].timestamp;
          player.currentCoordIdx++;
        }

        playerResults.push(new PlayerDataResultModel(player.id, player.data[player.currentCoordIdx - 1]));
      }

      const ballResult = this.matchData.ball.data[this.matchData.ball.currentCoordIdx - 1];

      return of(new MatchDataResultModel(playerResults, ballResult));
    }
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
