import {ChangeDetectionStrategy, ChangeDetectorRef, Component, ElementRef, ViewChild} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {UntilDestroy} from '@ngneat/until-destroy';
import {TitleService} from 'src/app/shared/services/title.service';
import {LeftMenuService} from '../../shared/left-menu/services/left.menu.service';
import {MatchDto} from "../services/match.api.service";

@UntilDestroy()
@Component({
  templateUrl: './match.get.component.html',
  styleUrls: ['./match.get.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MatchGetComponent {
  public match: MatchDto | null = null;

  leagueSlug: string = '';
  matchId: string = '';

  currentTime: number = 0;

  constructor(private leftMenuService: LeftMenuService,
              private route: ActivatedRoute,
              private titleService: TitleService,
              private changeDetectorRef: ChangeDetectorRef) {

    this.leagueSlug = this.route.snapshot.params["league_slug"];
    this.matchId = this.route.snapshot.params["match_id"];
  }

  ngOnInit(): void {

  }

  playMatchTick(time: number) {
    this.currentTime = time;
    this.changeDetectorRef.detectChanges();
  }

  initLeftMenu(match: MatchDto) {
    this.leftMenuService.setMenu([{items: [{url: '/', title: 'Home', icon: 'fa-home'}]},
      {items: [{url: '/inbox', title: 'Inbox', icon: 'fa-inbox'}]},
      // { items: player.neighbor_teams.map(subteam => ({ url: `/teams/${subteam.slug}`, title: subteam.name, icon: 'fa-user-friends' })) },
      //{ items: [{ url: `/teams/${player.team_slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
      {items: [{url: '/calendar', title: 'Calendar', icon: 'fa-calendar-alt'}]},
    ]);
  }
}
