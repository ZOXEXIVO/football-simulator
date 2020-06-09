import { Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Observable, Subject } from "rxjs";

@Injectable({
  providedIn: 'root',
})
export class ClubApi {
  constructor(private http: HttpClient) {
  }

  getPlayers(gameId, clubId): Observable<PlayerGetResponse> {
    const result = new Subject<PlayerGetResponse>();

    this.http.get('/api/game/' + gameId + '/club/' + clubId + '/players').subscribe((data: PlayerGetResponse) => {
      result.next(data);
    });

    return result;
  }
}

export class PlayerGetResponse {
  players: PlayerDto[];
}

export class PlayerDto {
  first_name: String;
  last_name: String;
  middle_name: String;
}
