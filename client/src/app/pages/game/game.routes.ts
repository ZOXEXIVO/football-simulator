import { Routes } from '@angular/router';
import { InboxComponent } from './inbox/inbox.component';
import { MainComponent } from './main/main.component';
import { GameComponent } from './game.component';
import { mainRoutes } from './main/main.routes';

export const gameRoutes: Routes = [
  {
    path: 'game/{gameId}',
    component: GameComponent,
    children: [
      { path: '', component: MainComponent },
      { path: 'inbox', component: InboxComponent },
    ]
  },
  ...mainRoutes   
];
