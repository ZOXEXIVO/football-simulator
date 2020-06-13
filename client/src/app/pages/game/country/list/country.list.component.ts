import { Component, OnInit } from '@angular/core';
import { FootballApi, CountryListResponse } from 'src/client/football.api.client';
import { ActivatedRoute} from '@angular/router';
import { HeaderService } from '../../services/header.service';

@Component({
  templateUrl: './country.list.component.html',
  styleUrls: ['./country.list.component.less']
})
export class CountryListComponent implements OnInit {
  isLoading: Boolean = false;

  countries: CountryListResponse;

  constructor(private api: FootballApi, private route: ActivatedRoute, private headerService: HeaderService) {
   
  }

  ngOnInit() {
    this.headerService.setHeader('Countries', 'Select country');

    this.route.parent.parent.params.subscribe(params => {
      this.isLoading = true;
      this.api.countries(params["gameId"])
      .subscribe(data => {
        this.countries = data;
        this.isLoading = false;
      })
    });
  }
}
