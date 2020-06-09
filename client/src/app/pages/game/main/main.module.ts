import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';
import { MainComponent } from './main.component';
import { CommonModule } from '@angular/common';

@NgModule({
  declarations: [
    MainComponent
  ],
  imports: [
    CommonModule,
    SharedModule,
    BrowserModule
  ],
  providers: [
  ],
})
export class MainModule { }
