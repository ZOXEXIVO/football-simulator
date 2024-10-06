import { Routes } from '@angular/router';
import { TeamGetComponent } from './get/team.get.component';
import { TeamScheduleComponent } from './schedule/team.schedule.component';

export const teamRoutes: Routes = [
  {
    path: 'teams/:slug',
    component: TeamGetComponent
  },
  {
    path: 'teams/:slug/schedule',
    component: TeamScheduleComponent
  }
];