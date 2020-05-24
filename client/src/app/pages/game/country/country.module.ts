import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';
import { CountryListComponent } from './list/country.list.component';
import { CountryDetailsComponent } from './details/country.details.component';
import { RouterModule } from '@angular/router';


@NgModule({
  declarations: [
    CountryListComponent,
    CountryDetailsComponent
  ],
  imports: [
    SharedModule,
    RouterModule,
    BrowserModule
  ],
  providers: [
  ],
})
export class CountryModule { } 
