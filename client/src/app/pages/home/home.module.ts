import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from '../../shared/shared.module';
import { RouterModule } from '@angular/router';
import { HomeComponent } from './home.component';
import { SearchComponent } from './search/search.component';
import { HelpComponent } from './help/help.component';

@NgModule({
  declarations: [
    HomeComponent,
    HelpComponent,
    SearchComponent
  ],
  imports: [
    SharedModule,
    BrowserModule,
    RouterModule
  ],
  providers: [
  ],
})
export class HomeModule { }
