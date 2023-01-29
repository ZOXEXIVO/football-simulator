import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { Observable, of } from 'rxjs';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import { CountryDto, CountryService } from '../services/country.service';

@UntilDestroy()
@Component({
  templateUrl: './country.get.component.html',
  styleUrls: ['./country.get.component.scss']
})
export class CountryGetComponent {
  public country: CountryDto | null = null;

  constructor(private leftMenuService: LeftMenuService,
    private service: CountryService,
    private route: ActivatedRoute) {
  }
  ngOnInit(): void {
    this.leftMenuService.setMenu([
      { items: [{ url: '/', title: 'Home', icon: 'fa-home' }] }
    ]);

    this.route.params.subscribe(params => {
      this.service.get(params["slug"]).pipe(untilDestroyed(this)).subscribe(countryData => {
        this.country = countryData
      });
    });
  }
}
