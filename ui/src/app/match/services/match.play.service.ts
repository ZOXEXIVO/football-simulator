import {Injectable} from "@angular/core";
import {Subject} from "rxjs";
import {MatchDataService} from "./match.data.service";

@Injectable({
    providedIn: 'root',
})
export class MatchPlayService {
    currentState: MatchEvent = MatchEvent.None;

    matchEvents = new Subject<MatchEvent>();
    public matchEvents$ = this.matchEvents.asObservable();

    timeChanged = new Subject<number>();
    public timeChanged$ = this.timeChanged.asObservable();

    currentTime = 0;
    changeTimeStamp = 10;

    constructor(private matchDataService: MatchDataService) {
    }

    tick() {
        if (this.currentState === MatchEvent.InProcess) {
            this.incrementTime();

            this.matchDataService.refreshData(this.currentTime);
        }
    }

    incrementTime() {
        this.currentTime += this.changeTimeStamp;
        this.timeChanged.next(this.currentTime);
    }

    startMatch() {
        this.currentState = MatchEvent.InProcess;
        this.matchEvents.next(MatchEvent.InProcess);
    }

    pause() {
        this.matchEvents.next(MatchEvent.Paused);
    }

    stop() {
        this.matchEvents.next(MatchEvent.Ended);
    }

    reset(){
        this.currentTime = 0;
    }
}

export enum MatchEvent {
    None,
    InProcess,
    Paused,
    Ended
}