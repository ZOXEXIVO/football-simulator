import { Routes } from '@angular/router';
import { InboxComponent } from './inbox/inbox.component';
import { MainComponent } from './main/main.component';
import { GameComponent } from './game.component';
import { CountryListComponent } from './country/list/country.list.component';
import { CountryDetailsComponent } from './country/details/country.details.component';
import { LeagueDetailsComponent } from './league/details/league.details.component';

export const gameRoutes: Routes = [
  {
    path: 'game/:gameId',
    component: GameComponent,
    children: [
      { path: 'main', component: MainComponent },
      { path: 'countries', component: CountryListComponent },
      { path: 'countries/:countryId', component: CountryDetailsComponent },
      { path: 'countries/:countryId/league/:leagueId', component: LeagueDetailsComponent },
      { path: 'inbox', component: InboxComponent },
    ]
  }   
];
