import { Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Observable, Subject } from "rxjs";

@Injectable({
    providedIn: 'root',
  })
export class FootballApi {
    private host = "http://localhost:18000";

    constructor(private http: HttpClient) {  
        
    }

    games(): Observable<GameListResultDto>  {
      const result = new Subject<GameListResultDto>();

      this.http.get(this.host + '/games').subscribe((data: GameListResultDto) => {
            result.next(data);
      });

      return result;
    }    

    createGame() {
        const result = new Subject<GameCreateResultDto>();

      this.http.post(this.host + '/games/create', {}).subscribe((data: GameCreateResultDto) => {
            result.next(data);
      });

      return result;
    }    
}

export class GameCreateResultDto{
    game_id: String;
    elapsed: Number;
}

export class GameListResultDto{
    games: GameListDto[]
}

export class GameListDto{
    id: String;
    date: String;
}