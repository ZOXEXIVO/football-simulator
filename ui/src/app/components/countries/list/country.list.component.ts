import {Component, OnInit} from '@angular/core';
import {Observable, of} from 'rxjs';
import {TitleService} from 'src/app/shared/services/title.service';
import {LeftMenuService} from '../../shared/left-menu/services/left.menu.service';
import {CountryListDto, CountryService} from '../services/country.service';
import {TopHeaderService} from "../../shared/top-header/services/top.header.service";
import {BaseComponent} from "../../base.component";
import {ProcessService} from "../../shared/process/services/process.service";

@Component({
  selector: 'country-list',
  templateUrl: './country.list.component.html',
  styleUrls: ['./country.list.component.scss']
})
export class CountryListComponent extends BaseComponent {
  countries$: Observable<CountryListDto[]> = of([]);

  constructor(
    private leftMenuService: LeftMenuService,
    private service: CountryService,
    private topHeaderService: TopHeaderService,
    private titleService: TitleService,
    private processService: ProcessService) {
    super(processService);
    this.countries$ = service.getList();

    this.titleService.setTitle('Available countries');
    this.topHeaderService.setContent('Select country', 'Select any country to inspect it all', '/');
  }

  override onDataRefresh() {
    this.leftMenuService.setMenu([
      {items: [{url: '/', title: 'Home', icon: 'fa-home'}]}
    ])
  }
}
