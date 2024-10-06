import {ModuleWithProviders} from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {PageComponent} from "./pages/page.component";
import {MatchComponent} from "./match/match.component";
import {AppModule} from "./app.module";

export const rootRoutes: Routes = [
  {
    path: '',
    component: PageComponent,
    loadChildren: () => import("./pages/page.module").then(m => m.PageModule)
  },
  {
    path: 'match',
    component: MatchComponent,
    loadChildren: () => import("./match/match.module").then(m => m.MatchModule)
  }
];

export const routes: ModuleWithProviders<AppModule> = RouterModule.forRoot(rootRoutes);