import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouterModule } from '@angular/router';
import { SharedModule } from '../shared/shared.module';
import { MatchGetComponent } from './get/match.get.component';

@NgModule({
  declarations: [
    MatchGetComponent
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
export class MatchModule { }
