import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from '../../shared/shared.module';
import { GameComponent } from './game.component';
import { RouterModule } from '@angular/router';
import { InboxModule } from './inbox/inbox.module';
import { LeftMenuComponent } from './menus/left.menu.component';
import { MainModule } from './main/main.module';

@NgModule({
  declarations: [
    GameComponent,
    LeftMenuComponent
  ],
  imports: [
    SharedModule,
    BrowserModule,
    RouterModule,
    InboxModule,
    MainModule
  ],
  providers: [
  ],
})
export class GameModule { }
