import { Routes } from '@angular/router';
import { GameComponent } from './game.component';
import { COUNTRY_ROUTES } from './country/country.routes';
import { LEAGUE_ROUTES } from './league/league.routes';
import { CLUB_ROUTES } from './club/club.routes';

export const GAME_ROUTES: Routes = [
  {
    path: 'game/:gameId',
    component: GameComponent,
    children: [
      ...COUNTRY_ROUTES,
      ...LEAGUE_ROUTES,
      ...CLUB_ROUTES
    ]
  }  
];
