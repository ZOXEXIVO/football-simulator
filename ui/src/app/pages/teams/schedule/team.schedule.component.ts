import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { TitleService } from 'src/app/shared/services/title.service';
import { TeamScheduleDto, TeamService } from '../services/team.service';
import {PageComponent} from "../../page.component";
import {TopHeaderService} from "../../../shared/top-header/services/top.header.service";
import {LeftMenuService} from "../../../shared/left-menu/services/left.menu.service";
import {ProcessService} from "../../../shared/process/services/process.service";

@UntilDestroy()
@Component({
  templateUrl: './team.schedule.component.html',
  styleUrls: ['./team.schedule.component.scss']
})
export class TeamScheduleComponent extends PageComponent {
  public teamSchedule: TeamScheduleDto | null = null;

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
      this.service.getSchedule(params["slug"]).pipe(untilDestroyed(this)).subscribe(teamScheduleData => {
        this.teamSchedule = teamScheduleData;
        this.initLeftMenu(teamScheduleData);

        this.titleService.setTitle(teamScheduleData.team_name + ' Schedule');
        this.topHeaderService.setContent('Schedule',
          teamScheduleData.team_name, '/leagues/' + teamScheduleData.league_slug);
      });
    });
  }

  initLeftMenu(teamSchedule: TeamScheduleDto) {
    this.leftMenuService.setMenu([{ items: [{ url: '/', title: 'Home', icon: 'fa-home' }] },
    { items: [{ url: '/inbox', title: 'Inbox', icon: 'fa-inbox' }] },
    { items: [{ url: `/teams/${teamSchedule.team_slug}`, title: teamSchedule.team_name, icon: 'fa-user-friends' }] },
    // { items: [{ url: `/teams/${team.slug}/schedule`, title: 'Schedule', icon: 'fa-inbox' }] },
    { items: [{ url: '/calendar', title: 'Calendar', icon: 'fa-calendar-alt' }] },
    ]);
  }

  createRange(number: number) {
    return new Array(number).fill(0)
      .map((n, index) => index + 1);
  }
}
