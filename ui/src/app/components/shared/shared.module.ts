import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { LeftMenuComponent } from './left-menu/left.menu.component';
import { LeftMenuService } from './left-menu/services/left.menu.service';
import { TopHeaderContentComponent } from './top-header/content/top.header.content.component';
import { TopHeaderComponent } from './top-header/top.header.component';

@NgModule({
  declarations: [
    LeftMenuComponent,
    TopHeaderComponent,
    TopHeaderContentComponent
  ],
  imports: [
    BrowserModule
  ],
  exports: [
    LeftMenuComponent,
    TopHeaderComponent,
    TopHeaderContentComponent],

  providers: [LeftMenuService]
})
export class SharedModule { }
