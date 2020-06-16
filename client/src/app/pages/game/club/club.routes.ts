import { Routes } from '@angular/router';
import { ClubComponent } from './club.component';
import { ClubSquadComponent } from './squad/club.squad.component';
import { PLAYER_ROUTES } from './player/player.routes';

export const CLUB_ROUTES: Routes = [
  {
    path: 'clubs/:clubId',
    component: ClubComponent,
    children: [
      { path: 'squad', component: ClubSquadComponent },
      ...PLAYER_ROUTES
    ]
  }   
];
