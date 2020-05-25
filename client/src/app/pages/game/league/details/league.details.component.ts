import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { CountryGetCountryDto, FootballApi } from 'src/client/football.api.client';

@Component({
  templateUrl: './league.details.component.html'
})
export class LeagueDetailsComponent implements OnInit {
  isLoading: Boolean = false;

  country: CountryGetCountryDto;

  constructor(private api: FootballApi, private route: ActivatedRoute) {
  }

  ngOnInit() {
    this.route.parent.params.subscribe(params => {
      this.isLoading = true;
      this.api.country(params["gameId"], this.route.snapshot.params.countryId)
      .subscribe(data => {
        this.country = data.country;
        this.isLoading = false;
      })
    });
  }
}