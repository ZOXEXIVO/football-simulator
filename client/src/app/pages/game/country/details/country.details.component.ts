import { Component, OnInit } from '@angular/core';

@Component({
  templateUrl: './country.details.component.html',
  styleUrls: ['./country.details.component.css']
})
export class CountryDetailsComponent { //} implements OnInit {
  // isLoading: Boolean = false;

  // countries: CountryListResponse;

  // constructor(private api: FootballApi, private route: ActivatedRoute) {
  // }

  // ngOnInit() {
  //   this.route.parent.params.subscribe(params => {
  //     this.isLoading = true;
  //     this.api.countries(params["gameId"])
  //     .subscribe(data => {
  //       this.countries = data;
  //       this.isLoading = false;
  //     })
  //   });
  // }
}
