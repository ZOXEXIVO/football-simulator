import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { HeaderService } from '../../services/header.service';
import { ClubApi, PlayerDto } from 'src/client/club.api.client';

@Component({
  templateUrl: './club.squad.component.html',
  styleUrls: ['./club.squad.component.less']
})
export class ClubSquadComponent implements OnInit {
  isLoading: Boolean = false;

  players: PlayerDto[];

  constructor(private clubApi: ClubApi,
    private route: ActivatedRoute,
    private headerService: HeaderService) {
  }

  ngOnInit() {
    this.route.parent.parent.params.subscribe(gameParams => {
      this.route.parent.params.subscribe(clubParams => {
        this.isLoading = true;
        debugger;
        this.clubApi.getPlayers(gameParams["gameId"], clubParams["clubId"]).subscribe(data => {
            this.players = data.players;

            //this.headerService.setHeader(data.country.name, 'Select leagues');

            this.isLoading = false;
          })
        });   
    });
  }
}
