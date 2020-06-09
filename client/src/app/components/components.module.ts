import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { BrowserModule } from '@angular/platform-browser';
import { ProcessButtonComponent } from './process-button/process-button.component';
import { SharedModule } from '../shared/shared.module';

@NgModule({
  declarations: [
    ProcessButtonComponent,
  ],
  imports: [
    BrowserModule,
    SharedModule,
    CommonModule
  ],
  exports: [
    ProcessButtonComponent,
  ]
})
export class ComponentsModule { }
