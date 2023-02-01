import { HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouterModule } from '@angular/router';
import { LeftMenuComponent } from './left-menu/left.menu.component';
import { LeftMenuService } from './left-menu/services/left.menu.service';
import { ProcessComponent } from './process/process.component';
import { TopHeaderComponent } from './top-header/top.header.component';

@NgModule({
  declarations: [
    LeftMenuComponent,
    TopHeaderComponent,
    ProcessComponent
  ],
  imports: [
    RouterModule,
    BrowserModule,
    HttpClientModule
  ],
  exports: [
    LeftMenuComponent,
    TopHeaderComponent,
    ProcessComponent
  ],

  providers: [LeftMenuService]
})
export class SharedModule { }
