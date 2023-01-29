import { Injectable } from "@angular/core";
import { BehaviorSubject, of } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class TopHeaderService {
    public content$: BehaviorSubject<String> = new BehaviorSubject<String>('');

    setContent(content: String) {
        this.content$.next(content);
    }
}