import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { CountryGetComponent } from './get/country.get.component';
import { CountryListComponent } from './list/country.list.component';
import {CommonModule} from "@angular/common";
import {SharedModule} from "../../shared/shared.module";

@NgModule({
  declarations: [
    CountryGetComponent,
    CountryListComponent,
  ],
  imports: [
    RouterModule,
    SharedModule,
    CommonModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: []
})
export class CountryModule { }
