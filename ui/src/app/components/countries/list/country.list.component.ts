import { Component, OnInit } from '@angular/core';
import { Observable, of } from 'rxjs';
import { CountryDto } from 'src/client/country.api.client';
import { LeftMenuService } from '../../menus/left-menu/services/left.menu.service';
import { CountryService } from '../services/country.service';

@Component({
  selector: 'country-list',
  templateUrl: './country.list.component.html',
  styleUrls: ['./country.list.component.scss']
})
export class CountryListComponent implements OnInit {
  countries$: Observable<CountryDto[]> = of([]);

  constructor(private leftMenuService: LeftMenuService, service: CountryService) {
    this.countries$ = service.getList();
  }
  ngOnInit(): void {
    this.leftMenuService.setMenu([
      { items: [{ url: '/', title: 'Home', icon: 'fa-home' }] }
    ])
  }
}
