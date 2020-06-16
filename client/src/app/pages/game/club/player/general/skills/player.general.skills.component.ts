import { Component, Input } from '@angular/core';
import { PlayerSkillsDto } from 'src/client/player.api.client';

@Component({
  selector: 'player-skills',
  templateUrl: './player.general.skills.component.html',
  styleUrls: ['./player.general.skills.component.less']
})
export class PlayerGeneralSkillComponent { 
  @Input()
  public skills: PlayerSkillsDto;
}