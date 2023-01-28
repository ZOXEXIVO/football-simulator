import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { LeftMenuService } from './left-menu/services/left.menu.service';

@NgModule({
  declarations: [    
  ],
  imports: [
    BrowserModule
  ],
  providers: [LeftMenuService],
  bootstrap: []
})
export class MenuModule { }
