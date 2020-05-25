import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from '../../shared/shared.module';
import { GameComponent } from './game.component';
import { RouterModule } from '@angular/router';
import { InboxModule } from './inbox/inbox.module';
import { LeftMenuComponent } from './menus/left.menu.component';
import { MainModule } from './main/main.module';
import { CountryModule } from './country/country.module';
import { LeagueModule } from './league/league.module';

@NgModule({
  declarations: [
    GameComponent,
    LeftMenuComponent
  ],
  imports: [
    SharedModule,
    CountryModule,
    LeagueModule,
    BrowserModule,
    RouterModule,
    InboxModule,
    MainModule
  ],
  providers: [
  ],
})
export class GameModule { }
