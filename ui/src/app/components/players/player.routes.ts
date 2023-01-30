import { Routes } from '@angular/router';
import { PlayerGetComponent } from './get/player.get.component';

export const playerRoutes: Routes = [
  {
    path: 'teams/:team_slug/players/:player_id',
    component: PlayerGetComponent
  }
];