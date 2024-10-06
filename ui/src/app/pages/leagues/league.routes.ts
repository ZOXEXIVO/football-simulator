import { Routes } from '@angular/router';
import { LeagueGetComponent } from './get/league.get.component';

export const leagueRoutes: Routes = [
  {
    path: 'leagues/:slug',
    component: LeagueGetComponent
  }
];