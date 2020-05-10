import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { InboxComponent } from './inbox.component';
import { SharedModule } from 'src/app/shared/shared.module';


@NgModule({
  declarations: [
    InboxComponent
  ],
  imports: [
    SharedModule,
    BrowserModule
  ],
  providers: [
  ],
})
export class InboxModule { }
