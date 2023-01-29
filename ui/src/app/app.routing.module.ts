import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { countryRoutes } from './components/countries/country.routes';
import { leagueRoutes } from './components/leagues/league.routes';

const routes: Routes = [
  ...countryRoutes,
  ...leagueRoutes,
  {path: '**', redirectTo: '/countries', pathMatch: 'full'}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
