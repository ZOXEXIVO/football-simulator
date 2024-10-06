import {Injectable} from "@angular/core";
import {MatchDataDto, MatchDto, ObjectPositionDto} from "./match.service";
import {Container} from "pixi.js";

@Injectable({
    providedIn: 'root',
})
export class MatchDataService {
    public match: MatchDto | null = null;
    public matchData: MatchDataDto | null = null;

    public width: number = 0;
    public height: number = 0;

    setMatch(match: MatchDto) {
        this.match = match;
    }

    setMatchData(data: MatchDataDto) {
        this.matchData = data;
    }

    setResolution(width: number, height: number){
        this.width = width;
        this.height = height;
    }

    refreshData(timestamp: number){
        let lastData = this.getData(timestamp);

        // update ball position
        if (lastData.ball) {
            let ballPosition = this.translateToField(lastData.ball.position[0], lastData.ball.position[1]);

            this.match!.ball!.obj!.x = ballPosition.x;
            this.match!.ball!.obj!.y = ballPosition.y;
        }

        // update players position
        this.match?.players.forEach(player => {
            let player_data = lastData.players.find((p) => p.id == player.id);

            if (player_data?.position) {
                let playerPosition = this.translateToField(player_data.position.position[0], player_data.position.position[1]);

                player.obj!.x = playerPosition.x;
                player.obj!.y = playerPosition.y;
            }
        });
    }

    translateToField(x: number, y: number) {
        const real_field_width = this.width - 100;
        const real_field_height = this.height;

        const inner_field_width = 840;
        const inner_field_height = 545;

        // Calculate the scaling factors
        const scale_x = real_field_width / inner_field_width;
        const scale_y = real_field_height / inner_field_height;

        // Apply the scaling to translate coordinates
        return {
            x: 50 + x * scale_x,
            y: y * scale_y
        };
    }

    getData(timestamp: number): MatchResultData {
        // ball
        let ts = this.matchData!.ball_positions[this.match!.ball.currentCoordIdx].timestamp;
        while (ts < timestamp && this.match!.ball.currentCoordIdx < this.matchData!.ball_positions.length) {
            ts = this.matchData!.ball_positions[this.match!.ball.currentCoordIdx].timestamp;
            this.match!.ball.currentCoordIdx++;
        }

        const ballResult = this.matchData!.ball_positions[this.match!.ball.currentCoordIdx - 1];

        let players_results: PlayerDataResultModel[] = [];

        Object.entries(this.matchData?.player_positions!).forEach(([key, value]: [string, ObjectPositionDto[]]) => {
            const player = this.match!.players.find((player) => player.id == Number(key))!;

            if(player){
                let dt = value![player.currentCoordIdx];

                if(dt) {
                    let pts = dt.timestamp;

                    while (pts < timestamp && player.currentCoordIdx < value!.length) {
                        dt = value![player.currentCoordIdx];

                        if(dt) {
                            pts = dt.timestamp;
                            player.currentCoordIdx++;
                        }
                    }

                    const playerPosition = value![player.currentCoordIdx];

                    players_results.push(new PlayerDataResultModel(player.id, playerPosition));
                }
            }
        });

        return new MatchResultData(players_results, ballResult);
    }

    setPlayerGraphicsObject(playerId: number, container: Container){
        const player = this.match!.players.find((player) => player.id == playerId);
        if(player) {
            player.obj = container;
        } else {
            console.error('player not found, playerId = ' + playerId);
        }
    }
}

export class MatchResultData {
    constructor(players: PlayerDataResultModel[], ball: ObjectPositionDto) {
        this.players = players;
        this.ball = ball;
    }

    public players: PlayerDataResultModel[];
    public ball: ObjectPositionDto;
}

export class PlayerDataResultModel {
    constructor(playerId: number, position: ObjectPositionDto) {
        this.id = playerId;
        this.position = position;
    }

    public id: number;
    public position: ObjectPositionDto;
}