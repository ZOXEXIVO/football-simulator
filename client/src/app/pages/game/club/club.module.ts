import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';

import { RouterModule } from '@angular/router';
import { ClubSquadComponent } from './squad/club.squad.component';
import { CommonModule } from '@angular/common';
import { ComponentsModule } from 'src/app/components/components.module';
import { ClubComponent } from './club.component';
import { PlayerModule } from './player/player.module';


@NgModule({
  declarations: [
    ClubComponent,
    ClubSquadComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    RouterModule,
    BrowserModule,
    ComponentsModule,
    PlayerModule
  ],
  providers: [
  ],
})
export class ClubModule { } 
