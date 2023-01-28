import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { MenuModule } from '../menus/menu.module';

import { CountryGetComponent } from './get/country.get.component';
import { CountryListComponent } from './list/country.list.component';

@NgModule({
  declarations: [
    CountryGetComponent,
    CountryListComponent
  ],
  imports: [
    BrowserModule,
    MenuModule
  ],
  providers: [],
  bootstrap: []
})
export class CountryModule { }
