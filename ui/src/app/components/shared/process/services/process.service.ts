import {HttpClient} from "@angular/common/http";
import {EventEmitter, Injectable} from "@angular/core";
import {Observable, tap} from "rxjs";
import {untilDestroyed} from "@ngneat/until-destroy";

@Injectable({
  providedIn: 'root',
})
export class ProcessService {
  public OnProcess: EventEmitter<any> = new EventEmitter<any>();

  constructor(private http: HttpClient) {
  }

  process(): Observable<Object> {
    return this.http.post('/api/game/process', {}).pipe(
      tap((data: any) => {
        this.OnProcess.emit(data);
      })
    );
  }
}
