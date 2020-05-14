import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';
import { LeagueDetailsComponent } from './details/league.details.component';
import { LeagueListComponent } from './list/league.list.component';

@NgModule({
  declarations: [
    LeagueListComponent,
    LeagueDetailsComponent
  ],
  imports: [
    SharedModule,
    BrowserModule
  ],
  providers: [
  ],
})
export class LeagueModule { } 
