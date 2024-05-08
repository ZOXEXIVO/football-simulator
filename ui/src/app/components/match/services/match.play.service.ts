import {Injectable} from "@angular/core";
import {ReplaySubject, Subject} from "rxjs";
import {MatchDataResultModel, MatchDataService} from "./match.data.service";
import {MatchLineupSetupCompleted} from "../play/models/models";

@Injectable({
    providedIn: 'root',
})
export class MatchPlayService {
    currentState: MatchEvent = MatchEvent.None;

    matchEvents = new Subject<MatchEvent>();
    public matchEvents$ = this.matchEvents.asObservable();

    lineupCompleted = new ReplaySubject<MatchLineupSetupCompleted>();
    public lineupCompleted$ = this.lineupCompleted.asObservable();

    public objectPositionChanged$ = new Subject<MatchDataResultModel>();

    currentTime = 0;
    changeTimeStamp = 10;

    constructor(private matchDataService: MatchDataService) {
    }

    init(leagueSlug: string, matchId: string){
        this.matchDataService.init(leagueSlug, matchId).subscribe(lineupData => {
            this.lineupCompleted.next(lineupData);
        });
    }

    tick() {
        this.incrementTime();

        this.matchDataService.getData(this.currentTime).subscribe(data => {
            this.objectPositionChanged$.next(data);
        });
    }

    incrementTime(){
        if (this.currentState === MatchEvent.InProcess) {
            this.currentTime += this.changeTimeStamp;
        }
    }

    start() {
        this.matchEvents.next(MatchEvent.InProcess);
    }

    pause() {
        this.matchEvents.next(MatchEvent.Paused);
    }

    stop() {
        this.matchEvents.next(MatchEvent.Ended);
    }
}

export enum MatchEvent {
    None,
    InProcess,
    Paused,
    Ended
}