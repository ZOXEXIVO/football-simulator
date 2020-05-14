import { Component, OnInit } from '@angular/core';
import { FootballApi, CountryListResponse } from 'src/client/football.api.client';
import { ActivatedRoute} from '@angular/router';

@Component({
  templateUrl: './country.list.component.html',
  styleUrls: ['./country.list.component.css']
})
export class CountryListComponent implements OnInit {
  isLoading: Boolean = false;

  countries: CountryListResponse;

  constructor(private api: FootballApi, private route: ActivatedRoute) {
  }

  ngOnInit() {
    this.route.parent.params.subscribe(params => {
      this.isLoading = true;
      this.api.countries(params["gameId"])
      .subscribe(data => {
        this.countries = data;
        this.isLoading = false;
      })
    });
  }
}
