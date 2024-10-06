import {Component, OnInit} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {UntilDestroy} from '@ngneat/until-destroy';
import {MatchDto, MatchService} from "../services/match.service";
import {MatchPlayService} from "../services/match.play.service";
import {MatchDataService} from "../services/match.data.service";

@UntilDestroy()
@Component({
    templateUrl: './match.get.component.html',
    styleUrls: ['./match.get.component.scss']
})
export class MatchGetComponent implements OnInit {
    public match: MatchDto | null = null;

    leagueSlug: string = '';
    matchId: string = '';

    matchTimeMs: number = 0;

    constructor(private matchPlayService: MatchPlayService,
                private matchDataService: MatchDataService,
                private matchService: MatchService,
                private route: ActivatedRoute) {

        this.leagueSlug = this.route.snapshot.params["league_slug"];
        this.matchId = this.route.snapshot.params["match_id"];
    }

    ngOnInit(): void {
        this.matchService.get(this.leagueSlug, this.matchId).subscribe(data => {
           this.matchDataService.setData(data);
        });
    }
}
