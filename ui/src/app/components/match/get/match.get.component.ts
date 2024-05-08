import {Component, OnInit} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {UntilDestroy} from '@ngneat/until-destroy';
import {LeftMenuService} from '../../shared/left-menu/services/left.menu.service';
import {MatchDto} from "../services/match.api.service";
import {MatchDataService} from "../services/match.data.service";
import {MatchModel} from "../play/models/models";
import {MatchPlayService} from "../services/match.play.service";

@UntilDestroy()
@Component({
    templateUrl: './match.get.component.html',
    styleUrls: ['./match.get.component.scss']
})
export class MatchGetComponent implements OnInit {
    public match: MatchDto | null = null;

    leagueSlug: string = '';
    matchId: string = '';

    constructor(private leftMenuService: LeftMenuService,
                private matchPlayService: MatchPlayService,
                private matchDataService: MatchDataService,
                private route: ActivatedRoute) {

        this.leagueSlug = this.route.snapshot.params["league_slug"];
        this.matchId = this.route.snapshot.params["match_id"];
    }

    ngOnInit(): void {
        this.matchPlayService.init(this.leagueSlug, this.matchId);

        this.matchPlayService.lineupCompleted$.subscribe(() => {
            this.initLeftMenu(this.matchDataService.matchData);
        });
    }

    initLeftMenu(match: MatchModel) {
        this.leftMenuService.setMenu([{items: [{url: '/', title: 'Home', icon: 'fa-home'}]},
            {
                items: [{
                    url: `/teams/${match.home_team.slug}`,
                    title: match.home_team.name,
                    icon: 'fa-light fa-people-group'
                }]
            },
            {
                items: [{
                    url: `/teams/${match.away_team.slug}`,
                    title: match.away_team.name,
                    icon: 'fa-light fa-people-group'
                }]
            },
        ]);
    }
}
