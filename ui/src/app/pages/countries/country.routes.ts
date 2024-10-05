import { Routes } from '@angular/router';
import { CountryGetComponent } from './get/country.get.component';
import { CountryListComponent } from './list/country.list.component';

export const countryRoutes: Routes = [
  {
    path: 'countries',
    component: CountryListComponent,
  },
  {
    path: 'countries/:slug',
    component: CountryGetComponent,
    // children: [
    //   { path: '', component: ClubMentionComponent },
    //   { path: 'page/:page', component: ClubMentionComponent },
    //   { path: 'players', component: ClubPlayerComponent },
    // ]
  }
];