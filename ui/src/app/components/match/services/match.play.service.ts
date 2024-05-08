import {Injectable} from "@angular/core";
import {Observable, Subject} from "rxjs";
import {MatchDataResultModel} from "./match.data.service";

@Injectable({
    providedIn: 'root',
})
export class MatchPlayService {
    matchEvents= new Subject<MatchEvent>();
    matchDataChanged= new Subject<MatchDataResultModel>();

    currentTime = 0;

    constructor() {
    }

    start(){
        this.matchEvents.next(MatchEvent.Started);
    }

    pause(){
        this.matchEvents.next(MatchEvent.Paused);
    }

    stop(){
        this.matchEvents.next(MatchEvent.Ended);
    }

    getMatchEventsSubscriptions(): Observable<MatchEvent> {
        return this.matchEvents.asObservable();
    }

    getMatchDataChangedSubscriptions(): Observable<MatchEvent> {
        return this.matchEvents.asObservable();
    }
}

export enum MatchEvent {
    Started,
    Paused,
    Ended
}