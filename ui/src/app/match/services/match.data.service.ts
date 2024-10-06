import {Injectable} from "@angular/core";
import {MatchDataDto, MatchDto, ObjectPositionDto} from "./match.service";
import {Container} from "pixi.js";
import {POLE_COORDS} from "../play/models/constants";

@Injectable({
    providedIn: 'root',
})
export class MatchDataService {
    public match: MatchDto | null = null;
    public matchData: MatchDataDto | null = null;

    setMatch(match: MatchDto) {
        this.match = match;
    }

    setMatchData(data: MatchDataDto) {
        this.matchData = data;
    }

    setPlayerGraphicsObject(playerId: number, container: Container){
        const player = this.match!.players.find((player) => player.id == playerId);
        if(player) {
            player.obj = container;
        } else {
            console.error('player not found, playerId = ' + playerId);
        }
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
            let player_position = lastData.players[player.id];

            if (player_position) {
                let playerPosition = this.translateToField(player_position.position.position[0], player_position.position.position[1]);

                player.obj!.x = playerPosition.x;
                player.obj!.y = playerPosition.y;
            }
        });
    }

    translateToField(x: number, y: number) {
        const real_field_width = 840;
        const real_field_height = 545;

        const screen_field_width = POLE_COORDS.tr.x - POLE_COORDS.tl.x;
        const screen_field_height = POLE_COORDS.br.y - POLE_COORDS.tr.y;

        const scaleX = screen_field_width / 840;
        const scaleY = screen_field_height / 545;

        return {
            x: POLE_COORDS.tl.x + (x * scaleX),
            y: POLE_COORDS.tl.y + (y * scaleY)
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

        let playerResults = [];
        for (const player of this.match!.players) {


            let pts = -1;
            while (pts < timestamp && player.currentCoordIdx < player.position.length) {
                pts = this.matchData!.player_positions[player.currentCoordIdx];
                player.currentCoordIdx++;
            }

            if (pts == -1) {
                continue;
            }

            const playerData = player.data[player.currentCoordIdx - 1];

            playerResults.push(new PlayerDataResultModel(player.id, playerData));
        }

        return new MatchResultData(players_results, ballResult);
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
        this.playerId = playerId;
        this.position = position;
    }

    public playerId: number;
    public position: ObjectPositionDto;
}