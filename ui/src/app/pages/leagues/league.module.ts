import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { LeagueGetComponent } from './get/league.get.component';
import {CommonModule} from "@angular/common";
import {SharedModule} from "../../shared/shared.module";

@NgModule({
  declarations: [
    LeagueGetComponent,
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
export class LeagueModule { }
