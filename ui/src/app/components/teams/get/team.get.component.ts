import { Component } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import { TeamDto, TeamService } from '../services/team.service';

@UntilDestroy()
@Component({
  templateUrl: './team.get.component.html',
  styleUrls: ['./team.get.component.scss']
})
export class TeamGetComponent {
  public team: TeamDto | null = null;

  constructor(private leftMenuService: LeftMenuService,
    private service: TeamService,
    private route: ActivatedRoute,
    private titleService: TitleService) {
  }
  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.service.get(params["slug"]).pipe(untilDestroyed(this)).subscribe(teamData => {
        this.team = teamData;
        this.initLeftMenu(teamData);

        this.titleService.setTitle(this.team.name + ', ' + this.team.league_name);
      });
    });
  }

  initLeftMenu(team: TeamDto) {
    this.leftMenuService.setMenu([{ items: [{ url: '/', title: 'Home', icon: 'fa-home' }] },
    { items: [{ url: '/inbox', title: 'Inbox', icon: 'fa-inbox' }] },
    { items: team.neighbor_teams.map(subteam => ({ url: `/teams/${subteam.slug}`, title: subteam.name, icon: 'fa-user-friends' })) },
    { items: [{ url: `/teams/${team.slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
    { items: [{ url: '/calendar', title: 'Calendar', icon: 'fa-calendar-alt' }] },
    ]);
  }

  createRange(number: number) {
    return new Array(number).fill(0)
      .map((n, index) => index + 1);
  }
}