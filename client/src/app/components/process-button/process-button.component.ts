import { Component, OnInit } from '@angular/core';
import { FootballApi } from 'src/client/football.api.client';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'process-button',
  templateUrl: './process-button.component.html'
})
export class ProcessButtonComponent implements OnInit {
  public isProcessing: Boolean = false;


  constructor(private api: FootballApi,
    private route: ActivatedRoute) {
  }

  ngOnInit(): void {

  }

  processGame() {
    let gameId = this.getGameId();

    this.isProcessing = true;

    this.api.processGame(gameId).subscribe(data => {
      this.isProcessing = false;
    });
  }

  getGameId() {
    if (this.route.snapshot.params.gameId) {
      return this.route.snapshot.params.gameId;
    }

    if (this.route.parent.snapshot.params.gameId) {
      return this.route.parent.snapshot.params.gameId;
    }

    if (this.route.parent.parent.snapshot.params.gameId) {
      return this.route.parent.parent.snapshot.params.gameId;
    }

    return null;
  }
}
