import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { of } from 'rxjs';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import { MatchDto, MatchService } from '../services/match.service';

@UntilDestroy()
@Component({
  templateUrl: './match.get.component.html',
  styleUrls: ['./match.get.component.scss']
})
export class MatchGetComponent {
  public match: MatchDto | null = null;

  leagueSlug: string = '';
  matchId: string = '';

  offset = 0;
  limit = 300;

  constructor(private leftMenuService: LeftMenuService,
    private service: MatchService,
    private route: ActivatedRoute,
    private titleService: TitleService) {
  }
  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.leagueSlug = params["league_slug"];
      this.matchId = params["match_id"];
    });

    this.loadMatchData(this.offset, this.limit);
  }

  initLeftMenu(match: MatchDto) {
    this.leftMenuService.setMenu([{ items: [{ url: '/', title: 'Home', icon: 'fa-home' }] },
    { items: [{ url: '/inbox', title: 'Inbox', icon: 'fa-inbox' }] },
   // { items: player.neighbor_teams.map(subteam => ({ url: `/teams/${subteam.slug}`, title: subteam.name, icon: 'fa-user-friends' })) },
    //{ items: [{ url: `/teams/${player.team_slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
    { items: [{ url: '/calendar', title: 'Calendar', icon: 'fa-calendar-alt' }] },
    ]);
  }

  loadMatchData(offset: number, limit: number) {
    this.service.get(this.leagueSlug, this.matchId, offset, limit).pipe(untilDestroyed(this)).subscribe(matchData => {
      this.match = matchData;
      

    });
  }
}
