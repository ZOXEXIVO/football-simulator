import { Routes } from '@angular/router';
import { LeagueDetailsComponent } from './details/league.details.component';
import { LeagueComponent } from './league.component';

export const LEAGUE_ROUTES: Routes = [
  {
    path: 'leagues', component: LeagueComponent,
    children: [
      { path: ':leagueId', component: LeagueDetailsComponent }
    ]
  }
];
