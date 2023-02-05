import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy } from '@ngneat/until-destroy';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import {MatchDto} from "../services/match.data.service";
@UntilDestroy()
@Component({
  templateUrl: './match.get.component.html',
  styleUrls: ['./match.get.component.scss']
})
export class MatchGetComponent {
  public match: MatchDto | null = null;

  leagueSlug: string = '';
  matchId: string = '';

  constructor(private leftMenuService: LeftMenuService,
    private route: ActivatedRoute,
    private titleService: TitleService) {

    this.leagueSlug = this.route.snapshot.params["league_slug"];
    this.matchId = this.route.snapshot.params["match_id"];
  }

  ngOnInit(): void {

  }

  initLeftMenu(match: MatchDto) {
    this.leftMenuService.setMenu([{ items: [{ url: '/', title: 'Home', icon: 'fa-home' }] },
    { items: [{ url: '/inbox', title: 'Inbox', icon: 'fa-inbox' }] },
   // { items: player.neighbor_teams.map(subteam => ({ url: `/teams/${subteam.slug}`, title: subteam.name, icon: 'fa-user-friends' })) },
    //{ items: [{ url: `/teams/${player.team_slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
    { items: [{ url: '/calendar', title: 'Calendar', icon: 'fa-calendar-alt' }] },
    ]);
  }
}
