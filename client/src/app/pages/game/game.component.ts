import { Component, OnInit } from '@angular/core';
import { FootballApi } from 'src/client/football.api.client';
import { ActivatedRoute } from '@angular/router';

@Component({
  templateUrl: './game.component.html'
})
export class GameComponent implements OnInit {
  isProcessing: Boolean = false;

  constructor(private api: FootballApi, private route: ActivatedRoute) {
  }

  processGame() {
    let gameId = this.route.snapshot.params.gameId;

    this.isProcessing = true;

    this.api.processGame(gameId).subscribe(data => {
      this.isProcessing = false;
      this.ngOnInit();
    });
  }

  ngOnInit() {

  }
}
