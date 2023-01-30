import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
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

  constructor(private leftMenuService: LeftMenuService,
    private service: MatchService,
    private route: ActivatedRoute,
    private titleService: TitleService) {
  }
  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const leagueSlug = params["league_slug"];
      const matchId = params["match_id"];

      this.service.get(leagueSlug, matchId, params["offset"], params["limit"]).pipe(untilDestroyed(this)).subscribe(matchData => {
        this.match = matchData;
        this.initLeftMenu(matchData);

        //this.titleService.setTitle(playerData.last_name + ' ' + playerData.first_name + ', ' + playerData.team_name);
      });
    });
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
