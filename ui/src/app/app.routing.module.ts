import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { countryRoutes } from './components/countries/country.routes';
import { leagueRoutes } from './components/leagues/league.routes';
import { playerRoutes } from './components/players/player.routes';
import { teamRoutes } from './components/teams/team.routes';

const routes: Routes = [
  ...countryRoutes,
  ...leagueRoutes,
  ...teamRoutes,
  ...playerRoutes,
  {path: '**', redirectTo: '/countries', pathMatch: 'full'}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
