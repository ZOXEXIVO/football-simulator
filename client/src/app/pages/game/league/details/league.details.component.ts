import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { FootballApi, LeagueGetLeagueDto } from 'src/client/football.api.client';

@Component({
  templateUrl: './league.details.component.html'
})
export class LeagueDetailsComponent implements OnInit {
  isLoading: Boolean = false;

  league: LeagueGetLeagueDto;

  constructor(private api: FootballApi, private route: ActivatedRoute) {
  }

  ngOnInit() {
    this.route.parent.params.subscribe(params => {
      this.isLoading = true;
      this.api.league(params["gameId"], this.route.snapshot.params.leagueId)
      .subscribe(data => {
        this.league = data.league;
        this.isLoading = false;
      })
    });
  }
}
