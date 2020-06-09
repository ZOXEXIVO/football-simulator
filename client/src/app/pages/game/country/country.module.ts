import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { SharedModule } from 'src/app/shared/shared.module';
import { CountryListComponent } from './list/country.list.component';
import { CountryDetailsComponent } from './details/country.details.component';
import { RouterModule } from '@angular/router';
import { CommonModule } from '@angular/common';
import { CountryComponent } from './country.component';
import { ComponentsModule } from 'src/app/components/components.module';


@NgModule({
  declarations: [
    CountryComponent,
    CountryListComponent,
    CountryDetailsComponent
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
export class CountryModule { } 
