import { Routes } from '@angular/router';
import { SearchComponent } from './search/search.component';
import { HomeComponent } from './home.component';
import { HelpComponent } from './help/help.component';

export const HOME_ROUTES: Routes = [
  {
    path: '',
    component: HomeComponent,
    children: [
      { path: '', component: SearchComponent },
      { path: 'help', component: HelpComponent }
    ]
  }
];
