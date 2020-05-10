import { ModuleWithProviders }  from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { homeRoutes } from './pages/home/home.routes';

export const routes: Routes = [
    ...homeRoutes,
  ];

  export const routing: ModuleWithProviders = RouterModule.forRoot(routes);
