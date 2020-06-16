import { Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Observable, Subject } from "rxjs";

@Injectable({
  providedIn: 'root',
})
export class PlayerApi {
  constructor(private http: HttpClient) {
  }

  getPlayer(gameId, clubId, playerId): Observable<PlayerGetResponse> {
    const result = new Subject<PlayerGetResponse>();

    this.http.get('/api/game/' + gameId + '/club/' + clubId + '/players/' + playerId).subscribe((data: PlayerGetResponse) => {
      result.next(data);
    });

    return result;
  }
}

export class PlayerGetResponse {
  player: PlayerDto;
}

export class PlayerDto {
  id: Number;
  first_name: String;
  last_name: String;
  middle_name: String;
  club_name: String;
  skills: PlayerSkillsDto;
}

export class PlayerSkillsDto {
  technical: TechnicalDto;
  mental: MentalDto;
  physical: PhysicalDto;
}

export class TechnicalDto {
  corners: Number;
  crossing: Number;
  dribbling: Number;
  finishing: Number;
  first_touch: Number;
  free_kick_taking: Number;
  heading: Number;
  long_shots: Number;
  long_throws: Number;
  marking: Number;
  passing: Number;
  penalty_taking: Number;
  tackling: Number;
  technique: Number;
}

export class MentalDto {
  aggression: Number;
  anticipation: Number;
  bravery: Number;
  composure: Number;
  concentration: Number;
  decisions: Number;
  determination: Number;
  flair: Number;
  leadership: Number;
  off_the_ball: Number;
  positioning: Number;
  teamwork: Number;
  vision: Number;
  work_rate: Number;
}

export class PhysicalDto {
  acceleration: Number;
  agility: Number;
  balance: Number;
  jumping_reach: Number;
  natural_fitness: Number;
  pace: Number;
  stamina: Number;
  strength: Number;

  match_readiness: Number;
}

