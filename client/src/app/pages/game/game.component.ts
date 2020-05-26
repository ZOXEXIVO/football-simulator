import { Component, OnInit, ChangeDetectionStrategy, ChangeDetectorRef } from '@angular/core';
import { FootballApi } from 'src/client/football.api.client';
import { ActivatedRoute } from '@angular/router';
import { HeaderService } from './services/header.service';

@Component({
  templateUrl: './game.component.html'
})
export class GameComponent implements OnInit {
  isProcessing: Boolean = false;

  headerTitle: String = "";

  constructor(private api: FootballApi, 
    private route: ActivatedRoute, 
    public headerService: HeaderService) {
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
    this.headerService.get().subscribe(title => {
      setTimeout(() => this.headerTitle = title);
    });
  }
}
