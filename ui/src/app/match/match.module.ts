import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { MatchGetComponent } from './get/match.get.component';
import {MatchPlayComponent} from "./play/play.match.component";
import {matchRoutes} from "./match.routes";
import {MatchComponent} from "./match.component";
import {CommonModule} from "@angular/common";
import {SharedModule} from "../shared/shared.module";

@NgModule({
  declarations: [
    MatchComponent,
    MatchGetComponent,
    MatchPlayComponent
  ],
  imports: [
    RouterModule,
    CommonModule,
    SharedModule,
    RouterModule.forChild(matchRoutes)
  ],
  providers: [],
  bootstrap: [MatchComponent]
})
export class MatchModule { }
