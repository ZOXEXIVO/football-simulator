import { ModuleWithProviders }  from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HOME_ROUTES } from './pages/home/home.routes';
import { GAME_ROUTES } from './pages/game/game.routes';

export const APP_ROUTES: Routes = [
    ...HOME_ROUTES,
    ...GAME_ROUTES,
    {
      path: '', redirectTo: '/search', pathMatch: 'full'
    },
  ];

  export const routing: ModuleWithProviders<RouterModule> = RouterModule.forRoot(APP_ROUTES);
