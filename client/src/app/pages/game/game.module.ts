import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from '../../shared/shared.module';
import { GameComponent } from './game.component';
import { RouterModule } from '@angular/router';
import { MainModule } from './main/main.module';
import { CountryModule } from './country/country.module';
import { LeagueModule } from './league/league.module';
import { ClubModule } from './club/club.module';
import { ComponentsModule } from 'src/app/components/components.module';
import { PlayerModule } from './player/player.module';

@NgModule({
  declarations: [
    GameComponent
  ],
  imports: [
    SharedModule,
    CountryModule,
    ClubModule,
    PlayerModule,
    LeagueModule,
    ComponentsModule,
    BrowserModule,
    RouterModule,
    MainModule
  ],
  providers: [
  ],
})
export class GameModule { }
