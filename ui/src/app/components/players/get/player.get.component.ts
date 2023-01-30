import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { TitleService } from 'src/app/shared/services/title.service';
import { LeftMenuService } from '../../shared/left-menu/services/left.menu.service';
import { PlayerDto, PlayerService } from '../services/player.service';

@UntilDestroy()
@Component({
  templateUrl: './player.get.component.html',
  styleUrls: ['./player.get.component.scss']
})
export class PlayerGetComponent {
  public player: PlayerDto | null = null;

  constructor(private leftMenuService: LeftMenuService,
    private service: PlayerService,
    private route: ActivatedRoute,
    private titleService: TitleService) {
  }
  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const teamSlug = params["team_slug"];
      const playerId = params["player_id"];

      this.service.get(teamSlug, playerId).pipe(untilDestroyed(this)).subscribe(playerData => {
        this.player = playerData;
        this.initLeftMenu(playerData);

        this.titleService.setTitle(playerData.last_name + ' ' + playerData.first_name + ', ' + playerData.team_name);
      });
    });
  }

  initLeftMenu(player: PlayerDto) {
    this.leftMenuService.setMenu([{ items: [{ url: '/', title: 'Home', icon: 'fa-home' }] },
    { items: [{ url: '/inbox', title: 'Inbox', icon: 'fa-inbox' }] },
    { items: player.neighbor_teams.map(subteam => ({ url: `/teams/${subteam.slug}`, title: subteam.name, icon: 'fa-user-friends' })) },
    { items: [{ url: `/teams/${player.team_slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
    { items: [{ url: '/calendar', title: 'Calendar', icon: 'fa-calendar-alt' }] },
    ]);
  }

  createRange(number: number) {
    return new Array(number).fill(0)
      .map((n, index) => index + 1);
  }
}
