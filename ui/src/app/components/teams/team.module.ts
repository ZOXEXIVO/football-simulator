import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { SharedModule } from '../shared/shared.module';
import { TeamGetComponent } from './get/team.get.component';
import { TeamScheduleComponent } from './schedule/team.schedule.component';

@NgModule({
  declarations: [
    TeamGetComponent,
    TeamScheduleComponent
  ],
  imports: [
    SharedModule,
    BrowserModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: []
})
export class TeamModule { }
