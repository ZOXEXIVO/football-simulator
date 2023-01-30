import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app.routing.module';
import { AppComponent } from './app.component';
import { CountryModule } from './components/countries/country.module';
import { SharedModule } from './components/shared/shared.module';
import { APP_BASE_HREF } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { LeagueModule } from './components/leagues/league.module';
import { TeamModule } from './components/teams/team.module';
import { PlayerModule } from './components/players/player.module';

@NgModule({
  declarations: [
    AppComponent,
  ],
  imports: [
    SharedModule,
    BrowserModule,
    AppRoutingModule,
    PlayerModule,
    CountryModule,
    LeagueModule,
    TeamModule,
    HttpClientModule
  ],
  providers: [{provide: APP_BASE_HREF, useValue: '/'}],
  bootstrap: [AppComponent]
})
export class AppModule { }
