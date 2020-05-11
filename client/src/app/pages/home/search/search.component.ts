import { Component, OnInit } from '@angular/core';
import { FootballApi, GameListDto } from 'src/client/football.api.client';

@Component({
  templateUrl: './search.component.html'
})
export class SearchComponent implements OnInit {
  public searchResults: GameListDto[];

  constructor(private api: FootballApi) {
  }
  
  creataGame() {
    this.api.createGame().subscribe(data => {
      alert(data.game_id);
   })
  }

  ngOnInit(): void {
     this.api.games().subscribe(data => {
        this.searchResults = data.games;
     })
  }
}
