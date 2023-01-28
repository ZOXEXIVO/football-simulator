import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app.routing.module';
import { AppComponent } from './app.component';
import { CountryModule } from './components/countries/country.module';
import { LeftMenuComponent } from './components/menus/left-menu/left.menu.component';
import { MenuModule } from './components/menus/menu.module';
import { SharedModule } from './components/shared/shared.module';
import { APP_BASE_HREF } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';

@NgModule({
  declarations: [
    AppComponent,
    LeftMenuComponent
  ],
  imports: [
    MenuModule,
    SharedModule,
    BrowserModule,
    AppRoutingModule,
    CountryModule,
    HttpClientModule
  ],
  providers: [{provide: APP_BASE_HREF, useValue: '/'}],
  bootstrap: [AppComponent]
})
export class AppModule { }
