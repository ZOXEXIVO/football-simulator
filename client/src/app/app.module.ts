import { BrowserModule } from '@angular/platform-browser';
import { NgModule, ErrorHandler } from '@angular/core';
import { routing } from './app.routes';

import { AppComponent } from './app.component';

import { HttpClientModule } from '@angular/common/http';
import { SharedModule } from './shared/shared.module';
import { GlobalErrorHandler } from './shared/errorHandling/globalErrorHandler';
import { RouterModule } from '@angular/router';
import { HomeModule } from './pages/home/home.module';
import { GameModule } from './pages/game/game.module';

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    HttpClientModule,
    HomeModule,
    GameModule,
    SharedModule,
    RouterModule,
    routing
  ],
  providers: [
    {
      provide: ErrorHandler,
      useClass: GlobalErrorHandler
    }
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
