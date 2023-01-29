import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { SharedModule } from '../shared/shared.module';
import { LeagueGetComponent } from './get/league.get.component';

@NgModule({
  declarations: [
    LeagueGetComponent,
  ],
  imports: [
    SharedModule,
    BrowserModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: []
})
export class LeagueModule { }
