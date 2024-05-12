import {Injectable} from "@angular/core";
import {finalize, Observable, of, Subject, switchMap} from "rxjs";
import {MatchDto, MatchService, ObjectPositionDto} from "./match.api.service";
import {
    BallModel,
    MatchLineupModel,
    MatchModel,
    PlayerModel,
    SquadPlayerModel,
    TeamModel
} from "../play/models/models";
import {Container} from "pixi.js";

@Injectable({
    providedIn: 'root',
})
export class MatchDataService {
    leagueSlug: string = '';
    matchId: string = '';

    loadTimestampSize = 10000;

    isBusy = false;

    public matchData: MatchModel = new MatchModel();

    lastLoadedTimestamp = 0;
    loadDataFinished = false;

    constructor(private matchService: MatchService) {

    }

    init(leagueSlug: string, matchId: string): Observable<MatchLineupModel> {
        this.leagueSlug = leagueSlug;
        this.matchId = matchId;

        const subject = new Subject<MatchLineupModel>();

        this.matchService.lineup(this.leagueSlug, this.matchId).subscribe(matchLineupData => {
            this.matchData.score.home_goals = matchLineupData.score.home_goals;
            this.matchData.score.away_goals = matchLineupData.score.away_goals;

            this.matchData.home_team = new TeamModel(
                matchLineupData.home_team_name,
                matchLineupData.home_team_slug
            );

            this.matchData.away_team = new TeamModel(
                matchLineupData.away_team_name,
                matchLineupData.away_team_slug
            );

            // setup ball
            const ballData =
             new BallModel([
                new ObjectPositionDto(0,
                    matchLineupData.ball.start_position[0],
                    matchLineupData.ball.start_position[1],
                    0
                )
            ]);

            this.matchData.ball = ballData;

            // setup players
            const playersData: PlayerModel[] = [];

            for (const player of matchLineupData.home_squad.main) {
                let playerPosition = new ObjectPositionDto(0,
                    player.start_position[0],
                    player.start_position[1],
                    0
                );

                const displayName = player.last_name;

                playersData.push(new PlayerModel(player.id, player.position, displayName, true, [playerPosition]));
            }

            this.matchData.players = playersData;

            // push event to ui
            subject.next(new MatchLineupModel(matchLineupData.match_time_ms, ballData, playersData));

            for (const player of matchLineupData.away_squad.main) {
                let playerPosition = new ObjectPositionDto(0,
                    player.start_position[0],
                    player.start_position[1],
                    player.start_position[2],
                );

                const displayName = player.last_name;

                this.matchData.players.push(new PlayerModel(
                    player.id, player.position, displayName, false, [playerPosition]));
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
                this.matchData.squad.away_subs.push(new SquadPlayerModel(
                    awaySubsSquadPlayer.id,
                    awaySubsSquadPlayer.first_name,
                    awaySubsSquadPlayer.last_name,
                    awaySubsSquadPlayer.middle_name,
                    awaySubsSquadPlayer.position,
                    awaySubsSquadPlayer.team_slug
                ));
            }
        });

        return subject.asObservable();
    }

    setPlayerGraphicsObject(playerId: number, container: Container){
        const player = this.matchData.players.find((player) => player.id == playerId);
        if(player) {
            player.obj = container;
        } else {
            console.error('player not found, playerId = ' + playerId);
        }
    }

    updateLocalData(matchDtaDto: MatchDto) {
        // players
        for (const playerData of this.matchData.players) {
            for (const [playerId, data] of Object.entries(matchDtaDto.player_data)) {
                if (playerData.id == Number(playerId)) {
                    let newPlayerData = data as number[][];

                    const pData = newPlayerData.map(pd => new ObjectPositionDto(pd[0], pd[1], pd[2], pd[3]));
                    playerData.data.push(...pData);
                }
            }
        }

        // balls
        for (const ballData of matchDtaDto.ball_data) {
            this.matchData.ball.data.push(new ObjectPositionDto(ballData[0], ballData[1], ballData[2], ballData[3]))
        }
    }

    loadRemoteData(): Observable<any> {
        const subject = new Subject<any>();

        if (this.loadDataFinished) {
            return of({});
        }

        const start_timestamp = this.lastLoadedTimestamp;
        const end_timestamp = start_timestamp + this.loadTimestampSize;

        this.matchService.get(this.leagueSlug, this.matchId, start_timestamp, end_timestamp).subscribe(matchData => {
            if (matchData.ball_data.length == 0) {
                this.loadDataFinished = true;
            }

            this.updateLocalData(matchData);

            this.lastLoadedTimestamp = end_timestamp;

            subject.next({});
        });

        return subject.asObservable();
    }

    getData(timestamp: number): Observable<MatchDataResultModel> {
        if (timestamp + 100 > this.lastLoadedTimestamp) {
            if (!this.isBusy) {
                this.isBusy = true;
                return this.loadRemoteData().pipe(
                    switchMap(() => {
                        return this.getLocalData(timestamp).pipe(finalize(() => {
                            this.isBusy = false;
                        }));
                    })
                );
            } else {
                return this.getLocalData(timestamp);
            }
        }

        return this.getLocalData(timestamp);
    }

    getLocalData(timestamp: number): Observable<MatchDataResultModel> {
        // ball
        let ts = -1;
        while (ts < timestamp && this.matchData.ball.currentCoordIdx < this.matchData.ball.data.length) {
            ts = this.matchData.ball.data[this.matchData.ball.currentCoordIdx].timestamp;
            this.matchData.ball.currentCoordIdx++;
        }

        const ballResult = this.matchData.ball.data[this.matchData.ball.currentCoordIdx - 1];

        // players
        let playerResults = [];
        for (const player of this.matchData.players) {
            let pts = -1;
            while (pts < timestamp && player.currentCoordIdx < player.data.length) {
                pts = player.data[player.currentCoordIdx].timestamp;
                player.currentCoordIdx++;
            }

            if (pts == -1) {
                continue;
            }

            const playerData = player.data[player.currentCoordIdx - 1];

            playerResults.push(new PlayerDataResultModel(player.id, playerData));
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
