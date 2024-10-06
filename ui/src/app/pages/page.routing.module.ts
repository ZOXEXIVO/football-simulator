import { Routes } from '@angular/router';
import {countryRoutes} from "./countries/country.routes";
import {leagueRoutes} from "./leagues/league.routes";
import {teamRoutes} from "./teams/team.routes";
import {playerRoutes} from "./players/player.routes";

export const pageRoutes: Routes = [
  ...countryRoutes,
  ...leagueRoutes,
  ...teamRoutes,
  ...playerRoutes,
  {path: '', redirectTo: '/countries', pathMatch: 'full'}
];
