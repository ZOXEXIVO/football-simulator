import { Routes } from '@angular/router';
import { SearchComponent } from './search/search.component';
import { HomeComponent } from './home.component';
import { HelpComponent } from './help/help.component';

export const homeRoutes: Routes = [
  {
    path: '',
    component: HomeComponent,
    children: [
      { path: 'search', component: SearchComponent },
      { path: 'help', component: HelpComponent }
    ]
  }
];
