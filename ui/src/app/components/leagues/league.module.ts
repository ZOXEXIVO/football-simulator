import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouterModule } from '@angular/router';
import { SharedModule } from '../shared/shared.module';
import { LeagueGetComponent } from './get/league.get.component';

@NgModule({
  declarations: [
    LeagueGetComponent,
  ],
  imports: [
    RouterModule,
    SharedModule,
    BrowserModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: []
})
export class LeagueModule { }
