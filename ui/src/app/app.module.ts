import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import {routes} from './app.routing.module';
import { AppComponent } from './app.component';
import { APP_BASE_HREF } from '@angular/common';
import {provideHttpClient, withInterceptorsFromDi} from '@angular/common/http';

@NgModule({
  declarations: [
    AppComponent,
  ],
  imports: [
    BrowserModule,
    routes
  ],
  providers: [{provide: APP_BASE_HREF, useValue: '/'}, provideHttpClient(withInterceptorsFromDi())],
  bootstrap: [AppComponent]
})
export class AppModule { }
