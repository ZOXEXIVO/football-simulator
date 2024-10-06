import {Injectable} from "@angular/core";
import {MatchDto} from "./match.service";
import {Container} from "pixi.js";

@Injectable({
    providedIn: 'root',
})
export class MatchDataService {

    public match: MatchDto | null = null;

    setData(match: MatchDto) {
        this.match = match;
    }

    // setPlayerGraphicsObject(playerId: number, container: Container){
    //     const player = this.match.players.find((player) => player.id == playerId);
    //     if(player) {
    //         player.obj = container;
    //     } else {
    //         console.error('player not found, playerId = ' + playerId);
    //     }
    // }

    // getData(timestamp: number): Observable<MatchDataResultModel> {
    //     // ball
    //     let ts = -1;
    //     while (ts < timestamp && this.matchData.ball.currentCoordIdx < this.matchData.ball.data.length) {
    //         ts = this.matchData.ball.data[this.matchData.ball.currentCoordIdx].timestamp;
    //         this.matchData.ball.currentCoordIdx++;
    //     }
    //
    //     const ballResult = this.matchData.ball.data[this.matchData.ball.currentCoordIdx - 1];
    //
    //     // players
    //     let playerResults = [];
    //     for (const player of this.matchData.players) {
    //         let pts = -1;
    //         while (pts < timestamp && player.currentCoordIdx < player.data.length) {
    //             pts = player.data[player.currentCoordIdx].timestamp;
    //             player.currentCoordIdx++;
    //         }
    //
    //         if (pts == -1) {
    //             continue;
    //         }
    //
    //         const playerData = player.data[player.currentCoordIdx - 1];
    //
    //         playerResults.push(new PlayerDataResultModel(player.id, playerData));
    //     }
    //
    //     return of(new MatchDataResultModel(playerResults, ballResult));
    // }
}
