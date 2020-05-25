import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';
import { LeagueDetailsComponent } from './details/league.details.component';
import { CommonModule } from '@angular/common';

@NgModule({
  declarations: [
    LeagueDetailsComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    BrowserModule
  ],
  providers: [
  ],
})
export class LeagueModule { } 
