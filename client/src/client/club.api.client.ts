import { Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Observable, Subject } from "rxjs";

@Injectable({
  providedIn: 'root',
})
export class ClubApi {
  constructor(private http: HttpClient) {
  }

  getPlayers(gameId, clubId): Observable<PlayersGetResponse> {
    const result = new Subject<PlayersGetResponse>();

    this.http.get('/api/game/' + gameId + '/club/' + clubId + '/players').subscribe((data: PlayersGetResponse) => {
      result.next(data);
    });

    return result;
  }
}

export class PlayersGetResponse {
  players: PlayerListDto[];
}

export class PlayerListDto {
  id: Number;
  first_name: String;
  last_name: String;
  middle_name: String;
}