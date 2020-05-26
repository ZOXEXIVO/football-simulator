import { Injectable, ChangeDetectorRef } from "@angular/core";
import { Observable, Subject } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class HeaderService {
    public titleSubject: Subject<String> = new Subject<String>();

    setHeader(title: String){
        this.titleSubject.next(title);
    }

    get(): Observable<String> {
        return this.titleSubject.asObservable();
    }
}