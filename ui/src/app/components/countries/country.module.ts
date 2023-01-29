import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { SharedModule } from '../shared/shared.module';
import { CountryGetComponent } from './get/country.get.component';
import { CountryListComponent } from './list/country.list.component';

@NgModule({
  declarations: [
    CountryGetComponent,
    CountryListComponent,
  ],
  imports: [
    SharedModule,
    BrowserModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: []
})
export class CountryModule { }