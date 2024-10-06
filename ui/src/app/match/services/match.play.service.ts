import {Injectable} from "@angular/core";
import {Subject} from "rxjs";
import {MatchDataService} from "./match.data.service";

@Injectable({
    providedIn: 'root',
})
export class MatchPlayService {
    currentState: MatchEvent = MatchEvent.None;

    // matchEvents = new Subject<MatchEvent>();
    // public matchEvents$ = this.matchEvents.asObservable();
    //
    // lineupCompleted = new Subject<MatchLineupModel>();
    // public lineupCompleted$ = this.lineupCompleted.asObservable();

    timeChanged = new Subject<number>();
    public timeChanged$ = this.timeChanged.asObservable();

    // public objectPositionChanged = new Subject<MatchDataResultModel>();
    // public objectPositionChanged$ = this.objectPositionChanged.asObservable();

    currentTime = 0;
    changeTimeStamp = 10;

    constructor(private matchDataService: MatchDataService) {
    }

    // init(leagueSlug: string, matchId: string) {
    //     this.matchDataService.init(leagueSlug, matchId).subscribe(lineupData => {
    //         this.lineupCompleted.next(lineupData);
    //     });
    // }

    tick() {
        // if (this.currentState === MatchEvent.InProcess) {
        //     this.incrementTime();
        //
        //     this.matchDataService.getData(this.currentTime).subscribe(data => {
        //         this.objectPositionChanged.next(data);
        //     });
        // }
    }

    incrementTime() {
        this.currentTime += this.changeTimeStamp;
        this.timeChanged.next(this.currentTime);
    }

    // startMatch() {
    //     this.currentState = MatchEvent.InProcess;
    //     this.matchEvents.next(MatchEvent.InProcess);
    // }
    //
    // pause() {
    //     this.matchEvents.next(MatchEvent.Paused);
    // }
    //
    // stop() {
    //     this.matchEvents.next(MatchEvent.Ended);
    //}
}

export enum MatchEvent {
    None,
    InProcess,
    Paused,
    Ended
}