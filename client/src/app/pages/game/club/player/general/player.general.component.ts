import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { HeaderService } from '../../../services/header.service';
import { PlayerApi, PlayerDto } from 'src/client/player.api.client';

@Component({
  templateUrl: './player.general.component.html',
  styleUrls: ['./player.general.component.less']
})
export class PlayerGeneralComponent implements OnInit {
  isLoading: Boolean = false;

  player: PlayerDto;

  constructor(private playerApi: PlayerApi,
    private route: ActivatedRoute,
    private headerService: HeaderService) {
  }

  ngOnInit() {

    this.headerService.setHeader('Ivan Ivanov', 'Select leagues');

    this.route.parent.parent.parent.params.subscribe(gameParams => {
      this.route.parent.parent.params.subscribe(clubParams => {
        this.route.parent.params.subscribe(playerParams => {

          this.isLoading = true;
          this.playerApi.getPlayer(gameParams["gameId"], clubParams["clubId"], playerParams["playerId"]).subscribe(data => {
             this.player = data.player;

             this.headerService.setHeader(this.player.first_name.toString() + " " + this.player.last_name, this.player.club_name);
          });          
        });
      });
    });
  }
}
