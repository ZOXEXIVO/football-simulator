import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { TeamGetComponent } from './get/team.get.component';
import { TeamScheduleComponent } from './schedule/team.schedule.component';
import {CommonModule} from "@angular/common";
import {SharedModule} from "../../shared/shared.module";

@NgModule({
  declarations: [
    TeamGetComponent,
    TeamScheduleComponent
  ],
  imports: [
    RouterModule,
    SharedModule,
    CommonModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: []
})
export class TeamModule { }
