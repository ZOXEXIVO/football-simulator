import {ChangeDetectionStrategy, ChangeDetectorRef, Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {UntilDestroy, untilDestroyed} from '@ngneat/until-destroy';
import {TitleService} from 'src/app/shared/services/title.service';
import {LeftMenuService} from '../../shared/left-menu/services/left.menu.service';
import {MatchDto} from "../services/match.api.service";
import {MatchDataService} from "../services/match.data.service";
import {TopHeaderService} from "../../shared/top-header/services/top.header.service";
import {MatchModel} from "../play/models/models";

@UntilDestroy()
@Component({
  templateUrl: './match.get.component.html',
  styleUrls: ['./match.get.component.scss']
})
export class MatchGetComponent implements OnInit {
  public match: MatchDto | null = null;

  leagueSlug: string = '';
  matchId: string = '';

  currentTime: number = 0;
  matchTimeMs: number =  -1;

  lineupLoaded: boolean = false;

  constructor(private leftMenuService: LeftMenuService,
              public matchDataService: MatchDataService,
              private route: ActivatedRoute,
              private titleService: TitleService,
              private topHeaderService: TopHeaderService) {

    this.leagueSlug = this.route.snapshot.params["league_slug"];
    this.matchId = this.route.snapshot.params["match_id"];
  }

  ngOnInit(): void {
    this.matchDataService.init(this.leagueSlug, this.matchId).pipe(untilDestroyed(this)).subscribe(matchLineupCompleted => {
      this.matchTimeMs = matchLineupCompleted.matchTimeMs;

      const data = this.matchDataService.matchData;

      this.titleService.setTitle(`${data.home_team.name} : ${data.away_team.name}`)
      this.topHeaderService.setContent(`${data.home_team.name} ${data.score.home_goals} : ${data.score.away_goals} ${data.away_team.name}`, '', '/', false);
      this.lineupLoaded = true;

      this.initLeftMenu(data);

      this.startMatch();
    });
  }

  startMatch(){
    setInterval(() => {
      this.currentTime += 10;
    },100);
  }

  initLeftMenu(match: MatchModel) {
    this.leftMenuService.setMenu([{items: [{url: '/', title: 'Home', icon: 'fa-home'}]},
      { items: [{ url: `/teams/${match.home_team.slug}`, title: match.home_team.name, icon: 'fa-light fa-people-group' }] },
      { items: [{ url: `/teams/${match.away_team.slug}`, title: match.away_team.name, icon: 'fa-light fa-people-group' }] },
    ]);
  }
}
