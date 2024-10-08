import {HttpClientModule, provideHttpClient, withInterceptorsFromDi} from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouterModule } from '@angular/router';
import { LeftMenuComponent } from './left-menu/left.menu.component';
import { LeftMenuService } from './left-menu/services/left.menu.service';
import { ProcessComponent } from './process/process.component';
import { TopHeaderComponent } from './top-header/top.header.component';
import {CommonModule} from "@angular/common";

@NgModule({
  declarations: [
    LeftMenuComponent,
    TopHeaderComponent,
    ProcessComponent
  ],
  imports: [
    RouterModule,
    CommonModule
  ],
  exports: [
    LeftMenuComponent,
    TopHeaderComponent,
    ProcessComponent
  ],
  providers:[provideHttpClient(withInterceptorsFromDi())]
})
export class SharedModule { }
