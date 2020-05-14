import { Routes } from '@angular/router';
import { InboxComponent } from './inbox/inbox.component';
import { MainComponent } from './main/main.component';
import { GameComponent } from './game.component';
import { CountryListComponent } from './country/list/country.list.component';
import { CountryDetailsComponent } from './country/details/country.details.component';

export const gameRoutes: Routes = [
  {
    path: 'game/:gameId',
    component: GameComponent,
    children: [
      { path: 'main', component: MainComponent },
      { path: 'countries', component: CountryListComponent },
      { path: 'country/:countryId', component: CountryDetailsComponent },
      { path: 'league', component: InboxComponent },
      { path: 'inbox', component: InboxComponent },
    ]
  }   
];
