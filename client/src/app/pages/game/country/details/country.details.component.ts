import { Component, OnInit} from '@angular/core';
import { FootballApi, CountryGetCountryDto } from 'src/client/football.api.client';
import { ActivatedRoute } from '@angular/router';
import { HeaderService } from '../../services/header.service';

@Component({
  templateUrl: './country.details.component.html',
  styleUrls: ['./country.details.component.css']
})
export class CountryDetailsComponent implements OnInit {  
  isLoading: Boolean = false;

  country: CountryGetCountryDto;

  constructor(private api: FootballApi, 
    private route: ActivatedRoute, 
    private headerService: HeaderService) {
  }

  ngOnInit() {
    this.route.parent.parent.params.subscribe(params => {
      this.isLoading = true;

      this.api.country(params["gameId"], this.route.snapshot.params.countryId).subscribe(data => {
        this.country = data.country;

        this.headerService.setHeader(data.country.name, 'Select leagues');

        this.isLoading = false;
      })
    });
  }
}
