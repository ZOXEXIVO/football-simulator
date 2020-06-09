import { Routes } from '@angular/router';
import { CountryDetailsComponent } from './details/country.details.component';
import { CountryListComponent } from './list/country.list.component';
import { CountryComponent } from './country.component';

export const COUNTRY_ROUTES: Routes = [
  {
    path: 'countries', component: CountryComponent,
    children: [
      { path: '', component: CountryListComponent },
      { path: ':countryId', component: CountryDetailsComponent }
    ]
  }
];
