import { Routes } from '@angular/router';
import { MatchGetComponent } from './get/match.get.component';

export const matchRoutes: Routes = [
  {
    path: 'match/:league_slug/:match_id',
    component: MatchGetComponent
  }
];