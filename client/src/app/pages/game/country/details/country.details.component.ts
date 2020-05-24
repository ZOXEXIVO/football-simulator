import { Component, OnInit} from '@angular/core';
import { FootballApi, CountryGetCountryDto } from 'src/client/football.api.client';
import { ActivatedRoute } from '@angular/router';

@Component({
  templateUrl: './country.details.component.html',
  styleUrls: ['./country.details.component.css']
})
export class CountryDetailsComponent implements OnInit {  
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
