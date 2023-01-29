import { Component, OnInit } from '@angular/core';
import { Observable, of } from 'rxjs';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import { CountryListDto, CountryService } from '../services/country.service';

@Component({
  selector: 'country-list',
  templateUrl: './country.list.component.html',
  styleUrls: ['./country.list.component.scss']
})
export class CountryListComponent implements OnInit {
  countries$: Observable<CountryListDto[]> = of([]);

  constructor(
    private leftMenuService: LeftMenuService, 
    service: CountryService, 
    private titleService: TitleService) {
    this.countries$ = service.getList();

    this.titleService.setTitle('Available countries');
  }
  ngOnInit(): void {
    this.leftMenuService.setMenu([
      { items: [{ url: '/', title: 'Home', icon: 'fa-home' }] }
    ])
  }
}
