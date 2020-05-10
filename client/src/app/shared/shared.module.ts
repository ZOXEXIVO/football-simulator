import { NgModule } from '@angular/core';
import { SpinnerComponent } from './spinners/default/spinner.component';
import { LineSpinnerComponent } from './spinners/line/line.spinner.component';

@NgModule({
  declarations: [
    SpinnerComponent,
    LineSpinnerComponent
  ],
  exports: [
    SpinnerComponent,
    LineSpinnerComponent
  ]
})
export class SharedModule { }
