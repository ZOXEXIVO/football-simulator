import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';

import { RouterModule } from '@angular/router';
import { CommonModule } from '@angular/common';
import { ComponentsModule } from 'src/app/components/components.module';
import { PlayerComponent } from './player.component';
import { PlayerGeneralComponent } from './general/player.general.component';


@NgModule({
  declarations: [
    PlayerComponent,
    PlayerGeneralComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    RouterModule,
    BrowserModule,
    ComponentsModule
  ],
  providers: [
  ],
})
export class PlayerModule { } 
