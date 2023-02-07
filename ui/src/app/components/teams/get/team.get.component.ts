import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import { TeamDto, TeamService } from '../services/team.service';
import {TopHeaderService} from "../../shared/top-header/services/top.header.service";
import {ProcessService} from "../../shared/process/services/process.service";
import {BaseComponent} from "../../base.component";

@UntilDestroy()
@Component({
  templateUrl: './team.get.component.html',
  styleUrls: ['./team.get.component.scss']
})
export class TeamGetComponent extends BaseComponent {
  public team: TeamDto | null = null;

  constructor(private leftMenuService: LeftMenuService,
    private service: TeamService,
    private topHeaderService: TopHeaderService,
    private route: ActivatedRoute,
    private titleService: TitleService,
    private processService: ProcessService) {
    super(processService);
  }
  override onDataRefresh(): void {
    this.route.params.subscribe(params => {
      this.service.get(params["slug"]).pipe(untilDestroyed(this)).subscribe(teamData => {
        this.team = teamData;
        this.initLeftMenu(teamData);

        this.titleService.setTitle(this.team.name + ', ' + this.team.league_name);
        this.topHeaderService.setContent(this.team.name, this.team.league_name, '/leagues/' + this.team.league_slug)
      });
    });
  }

  initLeftMenu(team: TeamDto) {
    this.leftMenuService.setMenu([{ items: [{ url: '/', title: 'Home', icon: 'fa-home' }] },
    { items: [{ url: '/inbox', title: 'Inbox', icon: 'fa-inbox' }] },
    { items: team.neighbor_teams.map(subteam => ({ url: `/teams/${subteam.slug}`, title: subteam.name, icon: 'fa-light fa-people-group' })) },
    { items: [{ url: `/teams/${team.slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
    { items: [{ url: '/calendar', title: 'Calendar', icon: 'fa-calendar-alt' }] },
    ]);
  }

  createRange(number: number) {
    return new Array(number).fill(0)
      .map((n, index) => index + 1);
  }
}
