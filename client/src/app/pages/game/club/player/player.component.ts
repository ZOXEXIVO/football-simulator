import { Component, OnInit } from '@angular/core';
import { FootballApi } from 'src/client/football.api.client';
import { ActivatedRoute } from '@angular/router';
import { HeaderService } from '../../services/header.service';

@Component({
  templateUrl: './player.component.html'
})
export class PlayerComponent implements OnInit {
  isProcessing: Boolean = false;

  headerTitle: String = "";
  headerSubTitle: String = "";

  constructor(private api: FootballApi, 
    private route: ActivatedRoute, 
    public headerService: HeaderService) {
  }

  ngOnInit() {
    this.headerService.get().subscribe(header => {
      setTimeout(() => {
        this.headerTitle = header.title;
        this.headerSubTitle = header.subTitle;
      });
    });
  }
}
