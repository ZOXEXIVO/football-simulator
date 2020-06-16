import { Routes } from '@angular/router';
import { PlayerComponent } from './player.component';
import { PlayerGeneralComponent } from './general/player.general.component';

export const PLAYER_ROUTES: Routes = [
  {
    path: 'players/:playerId',
    component: PlayerComponent,
    children: [
      { path: 'general', component: PlayerGeneralComponent }
    ]
  }   
];
