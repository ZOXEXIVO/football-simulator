import { Component, OnInit } from '@angular/core';
import { FootballApi, GameListDto } from 'src/client/football.api.client';
import { Router } from '@angular/router';

@Component({
  templateUrl: './search.component.html'
})
export class SearchComponent implements OnInit {
  isGameCreating: Boolean = false;

  public searchResults: GameListDto[];

  constructor(private api: FootballApi, private router: Router) {
  }
  
  createGame() {
    this.isGameCreating = true;

    this.api.createGame().subscribe(data => {
      this.router.navigate(['game', data.game_id, 'main']);
      this.isGameCreating = false;
   })
  }

  ngOnInit() {
     this.api.games().subscribe(data => {
        this.searchResults = data.games;
     })
  }
}
