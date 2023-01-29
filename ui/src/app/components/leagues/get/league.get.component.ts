import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import { LeagueDto, LeagueService } from '../services/league.service';

@UntilDestroy()
@Component({
  templateUrl: './league.get.component.html',
  styleUrls: ['./league.get.component.scss']
})
export class LeagueGetComponent {
  public league: LeagueDto | null = null;

  constructor(private leftMenuService: LeftMenuService,
    private service: LeagueService,
    private route: ActivatedRoute,
    private titleService: TitleService) {
  }
  ngOnInit(): void {
    this.leftMenuService.setMenu([
      { items: [{ url: '/', title: 'Home', icon: 'fa-home' }] }
    ]);

    this.route.params.subscribe(params => {
      this.service.get(params["slug"]).pipe(untilDestroyed(this)).subscribe(leagueData => {
        this.league = leagueData;

        this.titleService.setTitle(leagueData.name + ', ' + leagueData.country_name);
      });
    });
  }
}
