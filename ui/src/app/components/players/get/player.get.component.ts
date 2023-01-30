import { Component } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';

@UntilDestroy()
@Component({
  templateUrl: './player.get.component.html',
  styleUrls: ['./player.get.component.scss']
})
export class PlayerGetComponent {
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

        this.titleService.setTitle(this.team.name);
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
